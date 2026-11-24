//! Módulo de vistas y presentación
//!
//! Hito 5: Traits y polimorfismo (`trait Renderizador`)
//! Hito 7: Interfaz visual en terminal enriquecida (ANSI CLI y Ratatui TUI)

pub mod timeline;
pub mod tui;

pub use timeline::mostrar_linea_de_tiempo;
pub use tui::ejecutar_tui;
