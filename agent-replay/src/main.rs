// agent-replay · Punto de Entrada Principal
// Hito 7: CLI moderna con Clap, integración de todos los módulos del curso

use agent_replay::{
    cargar_sesion_desde_archivo,
    filtro::{filtrar_por_herramienta, filtrar_solo_errores, filtrar_solo_lentos},
    sesion_demo,
    vista::{ejecutar_tui, mostrar_linea_de_tiempo},
    Herramienta, Sesion,
};
use clap::Parser;
use std::path::PathBuf;

/// CLI para reproducir y analizar sesiones de agentes autónomos
#[derive(Parser, Debug)]
#[command(name = "agent-replay")]
#[command(author = "Carolina <carolinarb1994@gmail.com>")]
#[command(version = "0.2.0")]
#[command(about = "Reproductor interactivo de sesiones de agentes de IA en el terminal", long_about = None)]
struct Cli {
    /// Ruta a un archivo JSON con la sesión grabada
    #[arg(short, long)]
    archivo: Option<PathBuf>,

    /// Ejecutar en modo CLI plano sin abrir la interfaz TUI interactiva
    #[arg(long, default_value_t = false)]
    no_tui: bool,

    /// Filtrar eventos por nombre de herramienta (read, edit, write, bash, grep, glob)
    #[arg(short = 'H', long)]
    filtro_herramienta: Option<String>,

    /// Mostrar solo eventos lentos (duración >= 2000 ms)
    #[arg(long, default_value_t = false)]
    solo_lentos: bool,

    /// Mostrar solo eventos que resultaron en error
    #[arg(long, default_value_t = false)]
    solo_errores: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // 1. Cargar la sesión: desde archivo o usar demo
    let mut sesion: Sesion = match &cli.archivo {
        Some(ruta) => {
            println!("Cargando sesión desde {}...", ruta.display());
            cargar_sesion_desde_archivo(ruta)?
        }
        None => {
            // Si no se especifica archivo, cargamos la demo integrada
            sesion_demo()
        }
    };

    // 2. Aplicar filtros si el usuario los solicitó por CLI (Hito 6)
    if let Some(ref herr_str) = cli.filtro_herramienta {
        let herr_objetivo = match herr_str.to_lowercase().as_str() {
            "read" => Herramienta::Read,
            "edit" => Herramienta::Edit,
            "write" => Herramienta::Write,
            "bash" => Herramienta::Bash,
            "grep" => Herramienta::Grep,
            "glob" => Herramienta::Glob,
            otro => Herramienta::Custom(otro.to_string()),
        };
        let eventos_filtrados = filtrar_por_herramienta(sesion.eventos(), &herr_objetivo);
        sesion.eventos = eventos_filtrados.into_iter().cloned().collect();
    }

    if cli.solo_lentos {
        let eventos_filtrados = filtrar_solo_lentos(sesion.eventos());
        sesion.eventos = eventos_filtrados.into_iter().cloned().collect();
    }

    if cli.solo_errores {
        let eventos_filtrados = filtrar_solo_errores(sesion.eventos());
        sesion.eventos = eventos_filtrados.into_iter().cloned().collect();
    }

    // 3. Renderizar: TUI interactiva o modo texto continuo
    if cli.no_tui {
        mostrar_linea_de_tiempo(&sesion);
    } else {
        // En entornos no interactivos o si el usuario corre cargo run normal,
        // intentamos la TUI; si falla por no tener TTY, caemos a modo texto elegantemente.
        if let Err(e) = ejecutar_tui(sesion.clone()) {
            eprintln!("Aviso: no se pudo iniciar TUI ({e}). Mostrando modo texto estándar:");
            mostrar_linea_de_tiempo(&sesion);
        }
    }

    Ok(())
}
