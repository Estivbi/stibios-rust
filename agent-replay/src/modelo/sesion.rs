// agent-replay · Estructura de Sesión
// Hito 3 & 4: Modelado orientado a datos con Vec<Evento>

use super::evento::Evento;
use super::estadisticas::EstadisticasSesion;
use serde::{Deserialize, Serialize};

/// Representa una sesión completa ejecutada por un agente autónomo
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sesion {
    pub agente: String,
    pub descripcion: String,
    pub fecha: String,
    pub duracion_total_segundos: u32,
    pub eventos: Vec<Evento>,
}

impl Sesion {
    pub fn nueva(agente: impl Into<String>, descripcion: impl Into<String>) -> Self {
        Self {
            agente: agente.into(),
            descripcion: descripcion.into(),
            fecha: "2026-11-24".to_string(),
            duracion_total_segundos: 0,
            eventos: Vec::new(),
        }
    }

    pub fn agregar_evento(&mut self, evento: Evento) {
        if evento.segundo_inicio + (evento.duracion_ms / 1000) > self.duracion_total_segundos {
            self.duracion_total_segundos = evento.segundo_inicio + ((evento.duracion_ms + 999) / 1000);
        }
        self.eventos.push(evento);
    }

    /// Hito 2: Prestamos la colección interna como slice `&[Evento]`
    pub fn eventos(&self) -> &[Evento] {
        &self.eventos
    }

    /// Calcula las estadísticas usando borrowing
    pub fn estadisticas(&self) -> EstadisticasSesion {
        EstadisticasSesion::calcular(&self.eventos)
    }
}
