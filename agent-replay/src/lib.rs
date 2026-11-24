//! agent-replay: Reproductor interactivo de sesiones de agentes de IA
//! 
//! Proyecto pedagógico que implementa los conceptos del curso aprenderust.dev (Hitos 1 al 7).
//! Lecciones 1 a 26: Sintaxis básica, Ownership, Structs/Enums, Colecciones y Errores,
//! Traits y Genéricos, Closures e Iteradores, Concurrencia y Ecosistema (TUI + CLI).

pub mod modelo;
pub mod lector;
pub mod filtro;
pub mod vista;

// Re-exportamos los tipos más usados para conveniencia del usuario de la biblioteca
pub use modelo::{Evento, Herramienta, Sesion, EstadisticasSesion, EstadoEjecucion, NivelVelocidad};
pub use lector::{cargar_sesion_desde_archivo, sesion_demo};
