// agent-replay · Vista de Línea de Tiempo (Modo CLI)
// Hito 1 a 6: Formateo elegante en terminal con barras ASCII proporcionales y colores ANSI

use crate::modelo::{EstadisticasSesion, Sesion};

/// Genera una barra visual ASCII proporcional al tiempo (hasta 20 caracteres)
fn barra_duracion(ms: u32, max_ms: u32) -> String {
    let ancho_max = 20;
    if max_ms == 0 {
        return "·".to_string();
    }
    let bloques = ((ms as f64 / max_ms as f64) * ancho_max as f64).round() as usize;
    let bloques = bloques.max(1).min(ancho_max);
    "■".repeat(bloques)
}

/// Muestra en la terminal estándar la línea de tiempo completa de la sesión
pub fn mostrar_linea_de_tiempo(sesion: &Sesion) {
    let eventos = sesion.eventos();
    let stats = sesion.estadisticas();
    let max_ms = eventos.iter().map(|e| e.duracion_ms).max().unwrap_or(1);

    println!("\n\x1b[1;36m======================================================================\x1b[0m");
    println!(
        "\x1b[1;37m  AGENT-REPLAY · Reproductor de Sesión de Agente\x1b[0m"
    );
    println!("\x1b[1;36m======================================================================\x1b[0m");
    println!("  \x1b[1mAgente:\x1b[0m      {}", sesion.agente);
    println!("  \x1b[1mObjetivo:\x1b[0m    {}", sesion.descripcion);
    println!("  \x1b[1mDuración:\x1b[0m    {}s en sesión · {} llamadas totales", sesion.duracion_total_segundos, stats.total_eventos);
    println!("\x1b[1;36m----------------------------------------------------------------------\x1b[0m");
    println!("  \x1b[2mTiempo  Herramienta  Duración     Gráfico               Detalle\x1b[0m");
    println!("\x1b[1;36m----------------------------------------------------------------------\x1b[0m");

    for evento in eventos {
        let velocidad = evento.nivel_velocidad();
        let color = velocidad.color_ansi();
        let reset = "\x1b[0m";
        let icono = evento.herramienta.icono();
        let nombre = evento.herramienta.como_str();
        let barra = barra_duracion(evento.duracion_ms, max_ms);
        let status_mark = if evento.estado.es_exito() {
            "\x1b[32m✔\x1b[0m"
        } else {
            "\x1b[31m✖\x1b[0m"
        };

        let detalle = evento.resumen_detalle();
        // Truncamos detalle si supera 42 caracteres para no desbordar
        let detalle_corto = if detalle.chars().count() > 42 {
            format!("{}…", &detalle[..detalle.char_indices().nth(40).map_or(40, |(idx, _)| idx)])
        } else {
            detalle.to_string()
        };

        println!(
            "  t+{seg:>2}s  {icono} {nombre:<5}  {color}{ms:>5} ms{reset}  {status_mark} {color}{barra:<20}{reset}  \x1b[2m{detalle_corto}\x1b[0m",
            seg = evento.segundo_inicio,
            ms = evento.duracion_ms,
        );
    }

    mostrar_resumen_estadisticas(&stats);
}

fn mostrar_resumen_estadisticas(stats: &EstadisticasSesion) {
    println!("\x1b[1;36m----------------------------------------------------------------------\x1b[0m");
    println!("\x1b[1;37m  RESUMEN & MÉTRICAS POR HERRAMIENTA\x1b[0m");
    println!("\x1b[1;36m----------------------------------------------------------------------\x1b[0m");

    let mut herramientas: Vec<_> = stats.desglose.iter().collect();
    // Ordenamos por cantidad de llamadas descendente
    herramientas.sort_by(|a, b| b.1.cantidad.cmp(&a.1.cantidad));

    for (herr, metrica) in herramientas {
        println!(
            "  {icono} {nombre:<7} | {calls:>2} llamadas | {total:>6} ms acum. | prom: {prom:>4} ms | errores: {errs}",
            icono = herr.icono(),
            nombre = herr.como_str(),
            calls = metrica.cantidad,
            total = metrica.duracion_total_ms,
            prom = metrica.promedio_ms(),
            errs = metrica.llamadas_con_error,
        );
    }

    println!("\x1b[1;36m----------------------------------------------------------------------\x1b[0m");
    println!(
        "  \x1b[1mTotal:\x1b[0m {} ms ({:.2}s) de cómputo en herramientas | Errores: {}\n",
        stats.duracion_total_ms,
        stats.duracion_total_ms as f64 / 1000.0,
        stats.llamadas_con_error
    );
}
