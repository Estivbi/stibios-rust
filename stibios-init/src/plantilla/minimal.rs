// stibios-init · Plantilla Minimal
// Hito 5: Implementación de Trait

use super::trait_plantilla::GeneradorPlantilla;
use crate::modelo::ElementoEstructura;

pub struct PlantillaMinimal;

impl GeneradorPlantilla for PlantillaMinimal {
    fn nombre(&self) -> &'static str {
        "Minimal"
    }

    fn descripcion(&self) -> &'static str {
        "Estructura mínima indispensable con CLAUDE.md y carpetas esenciales"
    }

    fn generar_elementos(&self, nombre_proyecto: &str) -> Vec<ElementoEstructura> {
        vec![
            ElementoEstructura::carpeta("specs", "Especificaciones ligeras"),
            ElementoEstructura::carpeta("src", "Código fuente principal"),
            ElementoEstructura::archivo("CLAUDE.md", "Contexto básico para agentes", Some(format!("# {nombre_proyecto}\nProyecto ágil con agentes autónomos.\n"))),
            ElementoEstructura::archivo(".gitignore", "Archivos ignorados", Some("/target\n".into())),
        ]
    }
}
