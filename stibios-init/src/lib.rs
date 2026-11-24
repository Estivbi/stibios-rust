//! stibios-init: Generador CLI de scaffolding para proyectos con metodología SDD/TDD y agentes.
//!
//! Implementación de los Hitos 1 al 7 del curso aprenderust.dev (Lecciones 1 a 26).

pub mod modelo;
pub mod validador;
pub mod plantilla;
pub mod pipeline;
pub mod fs_builder;

pub use modelo::{ConfiguracionProyecto, ElementoEstructura, Metodologia, TipoElemento, VERSION_STIBIOS_INIT};
pub use validador::validar_nombre_proyecto;
pub use plantilla::{obtener_plantilla, GeneradorPlantilla};
pub use pipeline::{ContextoEjecucion, Pipeline};
pub use fs_builder::{construir_estructura, ErrorInit};
