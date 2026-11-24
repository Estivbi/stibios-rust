//! Módulo de filtros y operaciones funcionales sobre eventos
//!
//! Hito 6 (Lecciones 21-23): Closures, iteradores avanzados y programación funcional en Rust.

use crate::modelo::{Evento, Herramienta};

/// Lección 21: Función de orden superior que acepta un closure como predicado.
/// Usa referencias a eventos `&Evento` para no clonar ni mover ownership.
pub fn filtrar_eventos<'a, F>(eventos: &'a [Evento], predicado: F) -> Vec<&'a Evento>
where
    F: Fn(&Evento) -> bool,
{
    // Lección 22: Los iteradores son 'lazy' y `filter` solo evalúa lo necesario
    eventos.iter().filter(|evento| predicado(evento)).collect()
}

/// Filtra eventos que pertenecen a una herramienta específica
pub fn filtrar_por_herramienta<'a>(
    eventos: &'a [Evento],
    herramienta: &Herramienta,
) -> Vec<&'a Evento> {
    filtrar_eventos(eventos, |e| &e.herramienta == herramienta)
}

/// Filtra solo los eventos considerados lentos (duración >= UMBRAL_LENTO_MS)
pub fn filtrar_solo_lentos(eventos: &[Evento]) -> Vec<&Evento> {
    filtrar_eventos(eventos, |e| e.es_lento())
}

/// Filtra solo los eventos que terminaron en error
pub fn filtrar_solo_errores(eventos: &[Evento]) -> Vec<&Evento> {
    filtrar_eventos(eventos, |e| !e.estado.es_exito())
}

/// Lección 22: Uso de `fold` y transformaciones con iteradores para calcular tiempo acumulado
pub fn tiempo_acumulado_ms(eventos: &[&Evento]) -> u32 {
    eventos.iter().map(|e| e.duracion_ms).sum()
}
