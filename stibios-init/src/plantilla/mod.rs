//! Módulo de Plantillas de Estructura de Proyectos
//!
//! Hito 5 (Lecciones 17-20): Traits y Polimorfismo
//! Hito 6 (Lección 23): Punteros Inteligentes Box<dyn Trait>

pub mod trait_plantilla;
pub mod sdd;
pub mod tdd;
pub mod minimal;

pub use trait_plantilla::GeneradorPlantilla;
pub use sdd::PlantillaSdd;
pub use tdd::PlantillaTdd;
pub use minimal::PlantillaMinimal;

use crate::modelo::Metodologia;

/// Lección 23: Devuelve un puntero inteligente con dispatch dinámico `Box<dyn GeneradorPlantilla>`
pub fn obtener_plantilla(metodologia: Metodologia) -> Box<dyn GeneradorPlantilla> {
    match metodologia {
        Metodologia::Sdd => Box::new(PlantillaSdd),
        Metodologia::Tdd => Box::new(PlantillaTdd),
        Metodologia::Minimal => Box::new(PlantillaMinimal),
    }
}
