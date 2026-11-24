// agent-replay · Terminal User Interface (TUI Interactiva)
// Hito 7 (Lecciones 24-26): Concurrencia, Ecosistema con Ratatui y Crossterm

use crate::modelo::{Evento, NivelVelocidad, Sesion};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table, Wrap},
    Frame, Terminal,
};
use std::io;
use std::time::Duration;

pub fn ejecutar_tui(sesion: Sesion) -> Result<(), Box<dyn std::error::Error>> {
    // Configuración de la terminal en modo Raw
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app_res = ejecutar_bucle_eventos(&mut terminal, &sesion);

    // Restauración limpia de la terminal incluso si hubo error
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    app_res
}

fn ejecutar_bucle_eventos(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    sesion: &Sesion,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut indice_seleccionado: usize = 0;
    let eventos = sesion.eventos();
    let stats = sesion.estadisticas();

    loop {
        terminal.draw(|f| {
            renderizar_interfaz(f, sesion, eventos, &stats, indice_seleccionado);
        })?;

        if event::poll(Duration::from_millis(150))? {
            if let CEvent::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down | KeyCode::Char('j') => {
                        if !eventos.is_empty() && indice_seleccionado + 1 < eventos.len() {
                            indice_seleccionado += 1;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if indice_seleccionado > 0 {
                            indice_seleccionado -= 1;
                        }
                    }
                    KeyCode::Home => {
                        indice_seleccionado = 0;
                    }
                    KeyCode::End => {
                        if !eventos.is_empty() {
                            indice_seleccionado = eventos.len() - 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn renderizar_interfaz(
    f: &mut Frame,
    sesion: &Sesion,
    eventos: &[Evento],
    stats: &crate::modelo::EstadisticasSesion,
    seleccionado: usize,
) {
    let area_principal = f.area();

    // División vertical: Cabecera, Cuerpo Central, Barra de estado inferior
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area_principal);

    // 1. Cabecera
    let titulo = Line::from(vec![
        Span::styled(" 🦀 AGENT-REPLAY ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(format!("· Agente: {} ", sesion.agente), Style::default().fg(Color::White)),
    ]);
    let subtitulo = Line::from(vec![
        Span::styled(format!(" Misión: {} ", sesion.descripcion), Style::default().fg(Color::DarkGray)),
        Span::styled(format!(" [{} eventos | {} ms total] ", stats.total_eventos, stats.duracion_total_ms), Style::default().fg(Color::Yellow)),
    ]);
    let cabecera = Paragraph::new(vec![titulo, subtitulo])
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(cabecera, chunks[0]);

    // 2. Cuerpo: Timeline a la izquierda (60%) y Detalle/Métricas a la derecha (40%)
    let cuerpo_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(chunks[1]);

    // Lista de Eventos (Timeline)
    let items: Vec<ListItem> = eventos
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let es_sel = i == seleccionado;
            let icono = e.herramienta.icono();
            let color_vel = match e.nivel_velocidad() {
                NivelVelocidad::Rapido => Color::Green,
                NivelVelocidad::Medio => Color::Yellow,
                NivelVelocidad::Lento => Color::Red,
            };

            let prefix = if es_sel { "▶ " } else { "  " };
            let mark = if e.estado.es_exito() { "✔" } else { "✖" };

            let linea = Line::from(vec![
                Span::styled(prefix, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(format!("t+{:>2}s ", e.segundo_inicio), Style::default().fg(Color::DarkGray)),
                Span::raw(format!("{icono} {:<5} ", e.herramienta.como_str())),
                Span::styled(format!("{:>5} ms ", e.duracion_ms), Style::default().fg(color_vel)),
                Span::styled(format!("[{mark}] "), Style::default().fg(if e.estado.es_exito() { Color::Green } else { Color::Red })),
            ]);

            if es_sel {
                ListItem::new(linea).style(Style::default().bg(Color::Rgb(30, 45, 60)))
            } else {
                ListItem::new(linea)
            }
        })
        .collect();

    let lista = List::new(items)
        .block(
            Block::default()
                .title(" Línea de Tiempo (↑/↓ o j/k para navegar) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        );
    f.render_widget(lista, cuerpo_chunks[0]);

    // Panel Derecho: Detalle del evento seleccionado y Métricas
    let derecha_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(cuerpo_chunks[1]);

    // Detalle de Evento
    let detalle_widget = if let Some(ev) = eventos.get(seleccionado) {
        let detalle_txt = ev.resumen_detalle();
        let estado_txt = match &ev.estado {
            crate::modelo::EstadoEjecucion::Exito => "Éxito (200 OK)",
            crate::modelo::EstadoEjecucion::Error(msg) => msg.as_str(),
        };

        let contenido = vec![
            Line::from(vec![
                Span::styled("ID Evento: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("#{}", ev.id), Style::default().add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Herramienta: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{} {}", ev.herramienta.icono(), ev.herramienta.como_str()), Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("Duración: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{} ms ({})", ev.duracion_ms, ev.nivel_velocidad().etiqueta()), Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::styled("Estado: ", Style::default().fg(Color::DarkGray)),
                Span::styled(estado_txt, Style::default().fg(if ev.estado.es_exito() { Color::Green } else { Color::Red })),
            ]),
            Line::from(""),
            Line::from(Span::styled("Detalle / Parámetros:", Style::default().fg(Color::White).add_modifier(Modifier::UNDERLINED))),
            Line::from(Span::styled(detalle_txt, Style::default().fg(Color::White))),
        ];

        Paragraph::new(contenido)
            .wrap(Wrap { trim: true })
            .block(Block::default().title(" Inspector de Evento ").borders(Borders::ALL).border_style(Style::default().fg(Color::Green)))
    } else {
        Paragraph::new("Sin eventos seleccionados")
            .block(Block::default().borders(Borders::ALL))
    };
    f.render_widget(detalle_widget, derecha_chunks[0]);

    // Desglose de Herramientas
    let rows: Vec<Row> = stats
        .desglose
        .iter()
        .map(|(h, m)| {
            Row::new(vec![
                format!("{} {}", h.icono(), h.como_str()),
                format!("{}", m.cantidad),
                format!("{} ms", m.duracion_total_ms),
                format!("{} ms", m.promedio_ms()),
            ])
        })
        .collect();

    let tabla = Table::new(
        rows,
        [
            Constraint::Percentage(35),
            Constraint::Percentage(20),
            Constraint::Percentage(25),
            Constraint::Percentage(20),
        ],
    )
    .header(
        Row::new(vec!["Herramienta", "Llamadas", "Total ms", "Prom."])
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    )
    .block(
        Block::default()
            .title(" Desglose por Herramienta ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta)),
    );
    f.render_widget(tabla, derecha_chunks[1]);

    // 3. Barra de estado inferior
    let info_pie = Line::from(vec![
        Span::styled(" [q/Esc] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("Salir  "),
        Span::styled(" [j/k o ↑/↓] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("Navegar timeline  "),
        Span::styled(" [Home/End] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("Inicio/Fin  "),
        Span::styled(" | aprenderust.dev Hitos 1-7 ", Style::default().fg(Color::DarkGray)),
    ]);
    let pie = Paragraph::new(info_pie)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(pie, chunks[2]);
}
