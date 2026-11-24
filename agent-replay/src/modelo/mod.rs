//! Módulo de modelos de dominio para agent-replay
//!
//! Cubre conceptos de los Hitos 2, 3 y 4:
//! - Hito 2 (Lecciones 6-8): Referencias y slices (&[Evento], &str) sin copias innecesarias.
//! - Hito 3 (Lecciones 9-12): Structs, Enums con datos asociados, métodos con `impl`, y `Option<T>`.
//! - Hito 4 (Lecciones 13-16): Colecciones `Vec<T>`, `HashMap<K, V>` y `String`.

pub mod evento;
pub mod estadisticas;
pub mod sesion;

pub use evento::{Evento, Herramienta, EstadoEjecucion, NivelVelocidad, UMBRAL_LENTO_MS, UMBRAL_MEDIO_MS};
pub use estadisticas::{EstadisticasSesion, MetricasHerramienta};
pub use sesion::Sesion;
