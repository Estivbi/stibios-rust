// agent-replay · Modelo de Eventos
// Hito 3 (Lecciones 9-12): Structs, Enums y Option<T>
// Hito 2 (Lecciones 6-8): Referencias y préstamos

use serde::{Deserialize, Serialize};
use std::fmt;

// Lección 2: Constantes en mayúsculas con tipos explícitos
pub const UMBRAL_LENTO_MS: u32 = 2_000;
pub const UMBRAL_MEDIO_MS: u32 = 500;

/// Lección 10: Enums con variantes sin datos y variantes con datos asociados (Custom).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Herramienta {
    Read,
    Edit,
    Write,
    Bash,
    Grep,
    Glob,
    Custom(String),
}

impl Herramienta {
    /// Devuelve un icono representativo para mostrar en terminal/TUI
    pub fn icono(&self) -> &'static str {
        match self {
            Herramienta::Read => "📖",
            Herramienta::Edit => "✏️ ",
            Herramienta::Write => "📝",
            Herramienta::Bash => "⚡",
            Herramienta::Grep => "🔍",
            Herramienta::Glob => "📂",
            Herramienta::Custom(_) => "🔧",
        }
    }

    /// Nombre amigable de la herramienta como slice de texto (&str sin clonar)
    pub fn como_str(&self) -> &str {
        match self {
            Herramienta::Read => "Read",
            Herramienta::Edit => "Edit",
            Herramienta::Write => "Write",
            Herramienta::Bash => "Bash",
            Herramienta::Grep => "Grep",
            Herramienta::Glob => "Glob",
            Herramienta::Custom(ref s) => s.as_str(),
        }
    }
}

impl fmt::Display for Herramienta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.como_str())
    }
}

/// Lección 10: Enum para representar el resultado de la ejecución
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EstadoEjecucion {
    Exito,
    Error(String),
}

impl EstadoEjecucion {
    pub fn es_exito(&self) -> bool {
        matches!(self, EstadoEjecucion::Exito)
    }
}

/// Clasificación de velocidad de ejecución
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NivelVelocidad {
    Rapido,
    Medio,
    Lento,
}

impl NivelVelocidad {
    pub fn etiqueta(&self) -> &'static str {
        match self {
            NivelVelocidad::Rapido => "rápido",
            NivelVelocidad::Medio => "medio",
            NivelVelocidad::Lento => "lento",
        }
    }

    pub fn color_ansi(&self) -> &'static str {
        match self {
            NivelVelocidad::Rapido => "\x1b[32m", // Verde
            NivelVelocidad::Medio => "\x1b[33m",  // Amarillo
            NivelVelocidad::Lento => "\x1b[31m",  // Rojo
        }
    }
}

/// Lección 9: Struct para modelar un evento del agente
/// Hito 3: Uso de `Option<String>` para campos opcionales (Lección 11)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evento {
    pub id: usize,
    pub segundo_inicio: u32,
    pub herramienta: Herramienta,
    pub duracion_ms: u32,
    /// Lección 11: Option<String> evita nulls y obliga a manejar la ausencia de valor explícitamente.
    pub detalle: Option<String>,
    pub estado: EstadoEjecucion,
}

impl Evento {
    /// Constructor idiomático en Rust (función asociada, sin `self`)
    pub fn nuevo(
        id: usize,
        segundo_inicio: u32,
        herramienta: Herramienta,
        duracion_ms: u32,
        detalle: Option<String>,
        estado: EstadoEjecucion,
    ) -> Self {
        Self {
            id,
            segundo_inicio,
            herramienta,
            duracion_ms,
            detalle,
            estado,
        }
    }

    /// Lección 9: Método con referencia compartida `&self` (no consume el struct)
    pub fn nivel_velocidad(&self) -> NivelVelocidad {
        if self.duracion_ms >= UMBRAL_LENTO_MS {
            NivelVelocidad::Lento
        } else if self.duracion_ms >= UMBRAL_MEDIO_MS {
            NivelVelocidad::Medio
        } else {
            NivelVelocidad::Rapido
        }
    }

    pub fn es_lento(&self) -> bool {
        self.duracion_ms >= UMBRAL_LENTO_MS
    }

    /// Retorna una descripción resumida usando borrowing (&str)
    pub fn resumen_detalle(&self) -> &str {
        match &self.detalle {
            Some(texto) => texto.as_str(),
            None => "-",
        }
    }
}
