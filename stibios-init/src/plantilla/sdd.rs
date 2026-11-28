// stibios-init · Plantilla SDD (Spec-Driven Development) alineada con stibios-kit
// Hito 5: Implementación de Trait

use super::trait_plantilla::GeneradorPlantilla;
use crate::modelo::ElementoEstructura;

pub struct PlantillaSdd;

impl GeneradorPlantilla for PlantillaSdd {
    fn nombre(&self) -> &'static str {
        "SDD (Spec-Driven Development) · Stibios Kit"
    }

    fn descripcion(&self) -> &'static str {
        "Estructura oficial de stibios-kit: stibios.config.yml, AGENTS.md, CLAUDE.md, docs/backlog y docs/specs"
    }

    fn generar_elementos(&self, nombre_proyecto: &str) -> Vec<ElementoEstructura> {
        let stibios_config = format!(
            "# stibios.config.yml — configuración oficial del kit para {nombre_proyecto}\n\
            modulos: ui, seguridad, devops\n\
            tracker: none                    # none | jira | github | linear\n\
            prefijo_ticket: TCK\n\
            ticket_inicial: 1\n\
            rama_principal: main\n\
            formato_rama: \"{{ticket}}/{{descripcion}}\"\n\
            firmas: prohibir                 # prohibir | permitir\n\
            quien_abre_pr: persona           # persona | agente\n"
        );

        let agents_md = format!(
            "# AGENTS.md — Contrato del equipo de agentes de {nombre_proyecto}\n\n\
            Fuente de verdad para agentes y personas en este repositorio.\n\n\
            ## Stack y Arquitectura\n\
            - Proyecto: {nombre_proyecto}\n\
            - Metodología: SDD (Spec-Driven Development) + TDD\n\
            - Documentos de arquitectura: `docs/specs/`\n\
            - Backlog y tickets locales: `docs/backlog/`\n\n\
            ## Equipo de agentes (.claude/agents/)\n\
            | Agente | Rol | Es dueño de | Puede escribir |\n\
            |---|---|---|---|\n\
            | arquitecto | Diseño, specs y contratos | `docs/specs/` | Solo docs |\n\
            | backend | API, modelos y lógica | `src/` | Sí |\n\
            | frontend | Interfaz y componentes UI | `src/ui/` | Sí |\n\
            | seguridad | Auditoría de vulnerabilidades | — | No (solo informa) |\n\
            | devops | CI/CD y despliegue | `infra/`, `.github/` | Sí |\n\
            | pr-reviewer | Revisión contra specs | — | No (solo informa) |\n"
        );

        let claude_md = format!(
            "@AGENTS.md\n\n\
            ## Notas específicas para Claude Code ({nombre_proyecto})\n\n\
            - Los subagentes viven en `.claude/agents/` y se invocan por rol según la tarea.\n\
            - Comandos disponibles: `/ticket`, `/exportar`, `/spec`, `/plan`, `/tasks`, `/test`.\n\
            - Flujo SDD: redactar especificación en `docs/specs/` antes de codificar.\n\
            - Flujo TDD: escribir los tests primero (`tests/`).\n\
            - Si el proyecto incluye UI, consultar `DESIGN.md` antes de modificar interfaces.\n"
        );

        let design_md = format!(
            "# Documento de Diseño Técnico y UI (DESIGN.md)\n\n\
            ## Proyecto: {nombre_proyecto}\n\n\
            ### 1. Principios de Interfaz\n\
            - Claridad visual y jerarquía tipográfica.\n\
            - Estados de carga, error y éxito explícitos.\n\
            - Accesibilidad (a11y) y contraste adecuado.\n\n\
            ### 2. Tokens de Diseño\n\
            - Tipografía: Inter / JetBrains Mono (código).\n\
            - Espaciados: escala de 4px / 8px / 16px / 24px.\n"
        );

        let spec_sample = format!(
            "# Especificación SDD: 001-inicializacion.md\n\n\
            ## Historia de Usuario\n\
            Como desarrollador con agente de IA,\n\
            Quiero una estructura formal de proyecto,\n\
            Para asegurar que el agente entienda el contexto sin alucinaciones.\n\n\
            ## Criterios de Aceptación (Given / When / Then)\n\
            - **Dado** un entorno limpio,\n\
            - **Cuando** se ejecuta la inicialización con stibios-kit,\n\
            - **Entonces** se crean `stibios.config.yml`, `AGENTS.md` y `docs/` listos para usar.\n"
        );

        let ticket_sample = format!(
            "---\n\
            id: TCK-1\n\
            titulo: Inicialización del proyecto y configuración base\n\
            tipo: tarea\n\
            prioridad: alta\n\
            estado: listo\n\
            ---\n\n\
            # TCK-1: Configuración base con stibios-kit\n\n\
            Establecer la estructura de agentes, configuración y pipeline de integración.\n"
        );

        vec![
            ElementoEstructura::archivo("stibios.config.yml", "Configuración central del kit", Some(stibios_config)),
            ElementoEstructura::archivo("AGENTS.md", "Contrato de roles del equipo de agentes", Some(agents_md)),
            ElementoEstructura::archivo("CLAUDE.md", "Contexto de ejecución para Claude Code", Some(claude_md)),
            ElementoEstructura::archivo("DESIGN.md", "Directrices y tokens de diseño", Some(design_md)),
            ElementoEstructura::carpeta(".claude/agents", "Subagentes especializados"),
            ElementoEstructura::carpeta(".claude/skills", "Skills y automatizaciones"),
            ElementoEstructura::carpeta("docs/backlog", "Backlog local de tickets en Markdown"),
            ElementoEstructura::carpeta("docs/specs", "Especificaciones formales SDD"),
            ElementoEstructura::archivo("docs/specs/001-inicializacion.md", "Especificación SDD de ejemplo", Some(spec_sample)),
            ElementoEstructura::archivo("docs/backlog/TCK-1.md", "Ticket de backlog de ejemplo", Some(ticket_sample)),
            ElementoEstructura::carpeta("tests", "Suites de pruebas TDD"),
            ElementoEstructura::archivo(".gitignore", "Configuración de exclusiones de git", Some("/target\nCargo.lock\n.DS_Store\n".into())),
        ]
    }
}
