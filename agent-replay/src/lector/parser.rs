// agent-replay · Lector y Parser de Sesiones
// Hito 4 (Lecciones 13-16): Result<T, E>, operador `?` y errores idiomáticos
// Hito 5 (Lecciones 17-20): Implementación de traits estándar (Display, Error, From)

use crate::modelo::{EstadoEjecucion, Evento, Herramienta, Sesion};
use std::fmt;
use std::fs;
use std::path::Path;

/// Lección 16: Enum para representar cualquier error posible al leer una sesión
#[derive(Debug)]
pub enum ErrorLector {
    Io(std::io::Error),
    Json(serde_json::Error),
    FormatoInvalido(String),
}

impl fmt::Display for ErrorLector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorLector::Io(err) => write!(f, "Error de E/S al leer el archivo: {err}"),
            ErrorLector::Json(err) => write!(f, "Error de formato JSON: {err}"),
            ErrorLector::FormatoInvalido(msg) => write!(f, "Formato de sesión inválido: {msg}"),
        }
    }
}

impl std::error::Error for ErrorLector {}

// Lección 16 & 18: Implementación de `From` para permitir la conversión automática con el operador `?`
impl From<std::io::Error> for ErrorLector {
    fn from(err: std::io::Error) -> Self {
        ErrorLector::Io(err)
    }
}

impl From<serde_json::Error> for ErrorLector {
    fn from(err: serde_json::Error) -> Self {
        ErrorLector::Json(err)
    }
}

/// Lee un archivo JSON de disco y deserializa una `Sesion` completa
/// Usa el operador `?` para propagar errores limpiamente
pub fn cargar_sesion_desde_archivo<P: AsRef<Path>>(ruta: P) -> Result<Sesion, ErrorLector> {
    let contenido = fs::read_to_string(ruta.as_ref())?;
    let sesion: Sesion = serde_json::from_str(&contenido)?;
    if sesion.eventos.is_empty() {
        return Err(ErrorLector::FormatoInvalido(
            "La sesión no contiene ningún evento".to_string(),
        ));
    }
    Ok(sesion)
}

/// Provee una sesión demo rica en detalles para pruebas y presentaciones inmediatas
pub fn sesion_demo() -> Sesion {
    let mut sesion = Sesion::nueva(
        "Antigravity Agent (Claude Opus 4.6)",
        "Refactorización y resolución de bug en algoritmo de enrutamiento",
    );

    let eventos = vec![
        Evento::nuevo(
            1,
            0,
            Herramienta::Read,
            120,
            Some("src/router/mod.rs".to_string()),
            EstadoEjecucion::Exito,
        ),
        Evento::nuevo(
            2,
            1,
            Herramienta::Grep,
            340,
            Some("fn dispatch_request".to_string()),
            EstadoEjecucion::Exito,
        ),
        Evento::nuevo(
            3,
            3,
            Herramienta::Edit,
            180,
            Some("Añadir match para rutas dinámicas con parámetros".to_string()),
            EstadoEjecucion::Exito,
        ),
        Evento::nuevo(
            4,
            5,
            Herramienta::Bash,
            2_450,
            Some("cargo test --test router_tests".to_string()),
            EstadoEjecucion::Error("assertion failed: router.match('/api/v1') == Some(Handler)".to_string()),
        ),
        Evento::nuevo(
            5,
            9,
            Herramienta::Read,
            95,
            Some("tests/router_tests.rs".to_string()),
            EstadoEjecucion::Exito,
        ),
        Evento::nuevo(
            6,
            10,
            Herramienta::Edit,
            110,
            Some("Corregir prefijo normalizado en parse_segments".to_string()),
            EstadoEjecucion::Exito,
        ),
        Evento::nuevo(
            7,
            12,
            Herramienta::Bash,
            1_890,
            Some("cargo test --test router_tests".to_string()),
            EstadoEjecucion::Exito,
        ),
        Evento::nuevo(
            8,
            15,
            Herramienta::Write,
            150,
            Some("docs/ROUTING.md con especificación de endpoints".to_string()),
            EstadoEjecucion::Exito,
        ),
        Evento::nuevo(
            9,
            16,
            Herramienta::Bash,
            620,
            Some("git status && cargo clippy".to_string()),
            EstadoEjecucion::Exito,
        ),
    ];

    for evento in eventos {
        sesion.agregar_evento(evento);
    }

    sesion
}
