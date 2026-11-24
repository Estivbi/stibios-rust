// stibios-init · Plantilla TDD (Test-Driven Development)
// Hito 5: Implementación de Trait

use super::trait_plantilla::GeneradorPlantilla;
use crate::modelo::ElementoEstructura;

pub struct PlantillaTdd;

impl GeneradorPlantilla for PlantillaTdd {
    fn nombre(&self) -> &'static str {
        "TDD (Test-Driven Development)"
    }

    fn descripcion(&self) -> &'static str {
        "Enfoque riguroso 'Red -> Green -> Refactor' con tests unitarios e integrados"
    }

    fn generar_elementos(&self, nombre_proyecto: &str) -> Vec<ElementoEstructura> {
        let readme_tests = format!(
            "# Suite de Pruebas: {nombre_proyecto}\n\n\
            ## Reglas TDD\n\
            1. Escribe el test primero (falla en rojo).\n\
            2. Escribe el código mínimo para pasar el test (verde).\n\
            3. Refactoriza manteniendo los tests verdes.\n"
        );

        vec![
            ElementoEstructura::carpeta("tests/unit", "Pruebas unitarias de componentes"),
            ElementoEstructura::carpeta("tests/integration", "Pruebas de integración de extremo a extremo"),
            ElementoEstructura::carpeta("tests/fixtures", "Datos de prueba estáticos"),
            ElementoEstructura::archivo("tests/README.md", "Guía y convención de pruebas TDD", Some(readme_tests)),
            ElementoEstructura::archivo("CLAUDE.md", "Contexto enfocado en testing primero", Some(format!("# Contexto TDD: {nombre_proyecto}\nPrioridad absoluta a la cobertura de pruebas.\n"))),
            ElementoEstructura::archivo(".gitignore", "Ignorar artefactos de compilación", Some("/target\nCargo.lock\n".into())),
        ]
    }
}
