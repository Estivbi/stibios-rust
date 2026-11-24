// stibios-init · Manejo de Errores y Constructor de Sistema de Archivos
// Hito 4 (Lecciones 13-16): Result<T, E>, operador `?`, Display y Error

use crate::modelo::{ConfiguracionProyecto, ElementoEstructura, TipoElemento};
use crate::validador::ErrorValidacion;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ErrorInit {
    Validacion(ErrorValidacion),
    Io(std::io::Error),
    DirectorioYaExiste(PathBuf),
    RutaInvalida(String),
}

impl fmt::Display for ErrorInit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorInit::Validacion(err) => write!(f, "Error de validación: {err}"),
            ErrorInit::Io(err) => write!(f, "Error de entrada/salida (disco): {err}"),
            ErrorInit::DirectorioYaExiste(p) => {
                write!(f, "El directorio de destino ya existe y no está vacío: {}. Usa --force para sobrescribir.", p.display())
            }
            ErrorInit::RutaInvalida(msg) => write!(f, "Ruta no válida: {msg}"),
        }
    }
}

impl std::error::Error for ErrorInit {}

impl From<ErrorValidacion> for ErrorInit {
    fn from(err: ErrorValidacion) -> Self {
        ErrorInit::Validacion(err)
    }
}

impl From<std::io::Error> for ErrorInit {
    fn from(err: std::io::Error) -> Self {
        ErrorInit::Io(err)
    }
}

/// Construye en disco o simula (dry-run) la estructura de archivos
pub fn construir_estructura(
    config: &ConfiguracionProyecto,
    elementos: &[ElementoEstructura],
) -> Result<usize, ErrorInit> {
    let base = &config.directorio_destino;

    if !config.dry_run {
        if base.exists() && !config.sobrescribir {
            let entradas = fs::read_dir(base)?.count();
            if entradas > 0 {
                return Err(ErrorInit::DirectorioYaExiste(base.clone()));
            }
        }
        fs::create_dir_all(base)?;
    }

    let mut contador_creados = 0;

    for elem in elementos {
        let ruta_completa = base.join(&elem.ruta_relativa);

        if config.dry_run {
            println!(
                "  \x1b[33m[DRY-RUN]\x1b[0m {:<8} {}  \x1b[2m({}) - {}\x1b[0m",
                elem.tipo,
                elem.ruta_relativa,
                if elem.tipo == TipoElemento::Carpeta { "dir" } else { "file" },
                elem.descripcion
            );
            contador_creados += 1;
            continue;
        }

        match elem.tipo {
            TipoElemento::Carpeta => {
                fs::create_dir_all(&ruta_completa)?;
                println!("  \x1b[32m✔ Creada carpeta:\x1b[0m  {}", elem.ruta_relativa);
            }
            TipoElemento::Archivo => {
                if let Some(padre) = ruta_completa.parent() {
                    fs::create_dir_all(padre)?;
                }
                let contenido = elem.contenido_inicial.as_deref().unwrap_or("");
                fs::write(&ruta_completa, contenido)?;
                println!("  \x1b[32m✔ Creado archivo:\x1b[0m  {}", elem.ruta_relativa);
            }
        }
        contador_creados += 1;
    }

    Ok(contador_creados)
}
