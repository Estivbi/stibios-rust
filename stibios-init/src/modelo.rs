// stibios-init · Modelo de Datos
// Hito 1 (Lecciones 1-5): Constantes, tipos
// Hito 3 (Lecciones 9-12): Structs, Enums y Option<T>
// Hito 4 (Lecciones 13-16): Vec<T> y String

use std::fmt;
use std::path::PathBuf;

pub const VERSION_STIBIOS_INIT: &str = "0.2.0";

/// Lección 10: Enum para representar el tipo de nodo en el sistema de archivos
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipoElemento {
    Archivo,
    Carpeta,
}

impl fmt::Display for TipoElemento {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TipoElemento::Archivo => write!(f, "archivo"),
            TipoElemento::Carpeta => write!(f, "carpeta"),
        }
    }
}

/// Lección 10: Metodologías de trabajo disponibles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Metodologia {
    #[default]
    Sdd, // Spec-Driven Development
    Tdd, // Test-Driven Development
    Minimal,
}

impl Metodologia {
    pub fn como_str(&self) -> &'static str {
        match self {
            Metodologia::Sdd => "sdd",
            Metodologia::Tdd => "tdd",
            Metodologia::Minimal => "minimal",
        }
    }

    pub fn descripcion(&self) -> &'static str {
        match self {
            Metodologia::Sdd => "Spec-Driven Development: Especificaciones formales + agentes especializados",
            Metodologia::Tdd => "Test-Driven Development: Enfoque riguroso 'tests primero' con suite completa",
            Metodologia::Minimal => "Estructura mínima esencial para proyectos rápidos con agentes",
        }
    }
}

impl std::str::FromStr for Metodologia {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sdd" => Ok(Metodologia::Sdd),
            "tdd" => Ok(Metodologia::Tdd),
            "minimal" => Ok(Metodologia::Minimal),
            otro => Err(format!(
                "Metodología '{otro}' desconocida. Opciones válidas: sdd, tdd, minimal"
            )),
        }
    }
}

/// Lección 9 & 11: Struct que modela cada archivo o carpeta a generar
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementoEstructura {
    pub ruta_relativa: String,
    pub tipo: TipoElemento,
    pub descripcion: String,
    /// Contenido inicial opcional (Some si es archivo con plantilla inicial, None si es carpeta o archivo vacío)
    pub contenido_inicial: Option<String>,
}

impl ElementoEstructura {
    pub fn carpeta(ruta: impl Into<String>, descripcion: impl Into<String>) -> Self {
        Self {
            ruta_relativa: ruta.into(),
            tipo: TipoElemento::Carpeta,
            descripcion: descripcion.into(),
            contenido_inicial: None,
        }
    }

    pub fn archivo(
        ruta: impl Into<String>,
        descripcion: impl Into<String>,
        contenido: Option<String>,
    ) -> Self {
        Self {
            ruta_relativa: ruta.into(),
            tipo: TipoElemento::Archivo,
            descripcion: descripcion.into(),
            contenido_inicial: contenido,
        }
    }
}

/// Configuración global de la ejecución del scaffolding
#[derive(Debug, Clone)]
pub struct ConfiguracionProyecto {
    pub nombre: String,
    pub directorio_destino: PathBuf,
    pub metodologia: Metodologia,
    pub dry_run: bool,
    pub sobrescribir: bool,
}
