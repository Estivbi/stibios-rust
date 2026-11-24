// agent-replay · Estadísticas de Sesión
// Hito 4 (Lecciones 13-16): HashMap, Vec, colecciones
// Hito 6 (Lecciones 21-23): Iteradores y closures

use super::evento::{Evento, Herramienta};
use std::collections::HashMap;

/// Métricas agrupadas para una herramienta en particular
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MetricasHerramienta {
    pub cantidad: usize,
    pub duracion_total_ms: u32,
    pub llamadas_con_error: usize,
}

impl MetricasHerramienta {
    pub fn promedio_ms(&self) -> u32 {
        if self.cantidad == 0 {
            0
        } else {
            self.duracion_total_ms / (self.cantidad as u32)
        }
    }
}

/// Estadísticas globales calculadas sobre la sesión
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EstadisticasSesion {
    pub total_eventos: usize,
    pub duracion_total_ms: u32,
    pub llamadas_con_error: usize,
    pub desglose: HashMap<Herramienta, MetricasHerramienta>,
}

impl EstadisticasSesion {
    /// Hito 2 (Lección 7): Recibe una referencia a un slice `&[Evento]` para no clonar los datos
    /// Hito 4 (Lección 15): Uso de `HashMap` para agrupar dinámicamente
    /// Hito 6 (Lección 22): Uso de iteradores
    pub fn calcular(eventos: &[Evento]) -> Self {
        let mut desglose: HashMap<Herramienta, MetricasHerramienta> = HashMap::new();
        let mut duracion_total_ms = 0;
        let mut llamadas_con_error = 0;

        for evento in eventos {
            duracion_total_ms += evento.duracion_ms;
            if !evento.estado.es_exito() {
                llamadas_con_error += 1;
            }

            // Lección 15: Entry API de HashMap para insertar o modificar in-place
            let metrica = desglose.entry(evento.herramienta.clone()).or_default();
            metrica.cantidad += 1;
            metrica.duracion_total_ms += evento.duracion_ms;
            if !evento.estado.es_exito() {
                metrica.llamadas_con_error += 1;
            }
        }

        Self {
            total_eventos: eventos.len(),
            duracion_total_ms,
            llamadas_con_error,
            desglose,
        }
    }

    pub fn herramienta_mas_usada(&self) -> Option<(&Herramienta, &MetricasHerramienta)> {
        // Hito 6: max_by_key con closure
        self.desglose.iter().max_by_key(|(_, metrica)| metrica.cantidad)
    }

    pub fn herramienta_mas_lenta(&self) -> Option<(&Herramienta, &MetricasHerramienta)> {
        self.desglose.iter().max_by_key(|(_, metrica)| metrica.duracion_total_ms)
    }
}
