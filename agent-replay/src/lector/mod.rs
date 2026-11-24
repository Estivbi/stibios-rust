//! Módulo de lectura y deserialización de sesiones
//!
//! Hito 4 (Lecciones 13-16): Manejo robusto de errores con Result y el operador `?`.

pub mod parser;

pub use parser::{cargar_sesion_desde_archivo, sesion_demo, ErrorLector};
