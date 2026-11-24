// stibios-init · Plantilla SDD (Spec-Driven Development)
// Hito 5: Implementación de Trait

use super::trait_plantilla::GeneradorPlantilla;
use crate::modelo::ElementoEstructura;

pub struct PlantillaSdd;

impl GeneradorPlantilla for PlantillaSdd {
    fn nombre(&self) -> &'static str {
        "SDD (Spec-Driven Development)"
    }

    fn descripcion(&self) -> &'static str {
        "Enfoque formal: Especificaciones -> Diseño -> Agentes -> Implementación"
    }

    fn generar_elementos(&self, nombre_proyecto: &str) -> Vec<ElementoEstructura> {
        let claude_md = format!(
            "# Contexto del Proyecto: {nombre_proyecto}\n\n\
            Este proyecto sigue la metodología Stibios SDD (Spec-Driven Development).\n\n\
            ## Flujo de Trabajo\n\
            1. Definir especificación en `specs/` antes de escribir código.\n\
            2. Diseñar arquitectura técnica en `DESIGN.md`.\n\
            3. Implementar con agentes especializados y TDD.\n\n\
            ## Agentes Disponibles\n\
            - Arquitecto de Software (.claude/agents/architect.md)\n\
            - Ingeniero de Calidad y Tests (.claude/agents/qa-tester.md)\n"
        );

        let design_md = format!(
            "# Documento de Diseño Técnico (DESIGN.md)\n\n\
            ## Proyecto: {nombre_proyecto}\n\n\
            ### 1. Objetivos del Sistema\n\
            - [ ] Especificar requisitos funcionales principales.\n\
            - [ ] Modularización limpia y bajo acoplamiento.\n\n\
            ### 2. Arquitectura de Módulos\n\
            - `specs/`: Contratos de interfaces y casos de uso.\n\
            - `src/`: Código productivo.\n\
            - `tests/`: Suites de pruebas automatizadas.\n"
        );

        let spec_sample = format!(
            "# Especificación Inicial: 001-core-domain.md\n\n\
            ## Caso de Uso Principal: Inicialización\n\
            - Dado un entorno limpio,\n\
            - Cuando se ejecuta el sistema con parámetros válidos,\n\
            - Entonces debe inicializarse con estado OK.\n"
        );

        vec![
            ElementoEstructura::carpeta(".claude/agents", "Agentes especializados de IA"),
            ElementoEstructura::carpeta(".claude/skills", "Habilidades y utilidades de automatización"),
            ElementoEstructura::carpeta("specs", "Especificaciones formales del sistema"),
            ElementoEstructura::carpeta("tests", "Tests unitarios y de integración"),
            ElementoEstructura::archivo("CLAUDE.md", "Contexto del proyecto para agentes de IA", Some(claude_md)),
            ElementoEstructura::archivo("DESIGN.md", "Referencia y decisiones de diseño técnico", Some(design_md)),
            ElementoEstructura::archivo("specs/001-core-domain.md", "Especificación del dominio base", Some(spec_sample)),
            ElementoEstructura::archivo(".gitignore", "Configuración de exclusiones de git", Some("/target\nCargo.lock\n.DS_Store\n".into())),
        ]
    }
}
