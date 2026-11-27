# stibios-rust 🦀

[![Rust](https://img.shields.io/badge/rust-1.100%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-8%20passed-brightgreen.svg)]()

Dos herramientas de línea de comandos en **Rust** construidas de principio a fin siguiendo el programa completo de **[aprenderust.dev](https://aprenderust.dev/hola-mundo)** (Hitos 1 al 7, Lecciones 1 a 26):

1. **`agent-replay`**: Reproductor interactivo de sesiones de agentes de IA en el terminal (TUI interactiva con Ratatui + modo CLI directo, filtros funcionales por herramienta, inspector de eventos y métricas de rendimiento).
2. **`stibios-init`**: Generador CLI de scaffolding para proyectos con metodología SDD (Spec-Driven Development), TDD y agentes autónomos, validación estricta sin copias (`&str`) y pipeline desacoplado con closures.

---

## 🗺️ Mapa de Hitos y Conceptos del Curso

| Hito | Lecciones | Conceptos Clave en Rust | Implementación en `agent-replay` | Implementación en `stibios-init` | Estado |
|:---:|---|---|---|---|:---:|
| **1** | **1-5** | `main`, macros (`println!`), variables, tipos primitivos, constantes, funciones y control de flujo (`if`, `for`). | Línea de tiempo base con eventos y clasificación por umbrales de tiempo. | Maqueta de rutas y constantes de versión. | ✅ Hecho |
| **2** | **6-8** | *Ownership*, préstamo (*borrowing*), referencias (`&`, `&mut`) y *slices* (`&[T]`, `&str`) sin clones. | Funciones de cálculo y métricas recibiendo `&[Evento]` y préstamos de texto. | Validador de nombres de proyecto sobre `&str` con cero asignaciones heap. | ✅ Hecho |
| **3** | **9-12** | `struct`, métodos con `impl`, `enum` con datos asociados y `Option<T>` idiomático. | `struct Evento`, `enum Herramienta` con iconos, `enum EstadoEjecucion` y `Option<String>` para detalles. | `struct ElementoEstructura`, `enum TipoElemento`, `enum Metodologia`. | ✅ Hecho |
| **4** | **13-16** | Colecciones (`Vec<T>`, `HashMap<K, V>`), `String`, manejo de errores con `Result<T, E>` y operador `?`. | Agrupación y métricas en `HashMap`, lector de sesiones JSON con `ErrorLector` personalizado. | Generación de archivos y carpetas con `std::fs` y propagación de errores con `ErrorInit`. | ✅ Hecho |
| **5** | **17-20** | Genéricos, *traits*, métodos por defecto, polimorfismo y modularización (`mod`). | Desacoplamiento en módulos (`modelo`, `lector`, `filtro`, `vista`). | `trait GeneradorPlantilla` implementado por `PlantillaSdd`, `PlantillaTdd` y `PlantillaMinimal`. | ✅ Hecho |
| **6** | **21-23** | *Closures*, iteradores avanzados (`filter`, `map`, `fold`) y *smart pointers* (`Box<dyn Trait>`). | Filtros funcionales de orden superior (`filtrar_eventos<F>`), cálculo acumulado con iteradores. | `Pipeline` de ejecución desacoplado con closures dinámicos `Box<dyn Fn>`. | ✅ Hecho |
| **7** | **24-26** | Ecosistema productivo, CLI con `clap`, TUI con `ratatui` + `crossterm`, y tests unitarios y de integración. | Interfaz TUI completa interactiva + modo CLI plano con `--no-tui` y flags de filtrado. | CLI con subcomandos `new`, `list`, `check` y suite completa con `cargo test`. | ✅ Hecho |

---

## ⚡ Instalación y Ejecución Rápida

### Opción 1: Instalar directamente con Cargo (en cualquier máquina con Rust)

```bash
# Instalar los binarios directamente desde GitHub:
cargo install --git https://github.com/Estivbi/stibios-rust agent-replay
cargo install --git https://github.com/Estivbi/stibios-rust stibios-init

# Ejecutar en tu terminal:
agent-replay
stibios-init --help
```

### Opción 2: Clonar y compilar localmente

```bash
# Clonar el repositorio
git clone https://github.com/Estivbi/stibios-rust.git
cd stibios-rust

# Ejecutar todos los tests (8 tests pasan)
cargo test
```

### 1. Probar `agent-replay`

```bash
# Modo TUI interactivo (usa teclado: j/k o flechas para navegar eventos, q para salir)
cargo run -p agent-replay

# Modo CLI directo con barras ASCII y resumen de métricas
cargo run -p agent-replay -- --no-tui

# Filtrar solo llamadas a Bash que superen el umbral o fallaron
cargo run -p agent-replay -- --no-tui --filtro-herramienta bash
cargo run -p agent-replay -- --no-tui --solo-errores
```

#### Salida en Modo CLI:
```text
======================================================================
  AGENT-REPLAY · Reproductor de Sesión de Agente
======================================================================
  Agente:      Antigravity Agent (Claude Opus 4.6)
  Objetivo:    Refactorización y resolución de bug en algoritmo de enrutamiento
  Duración:    16s en sesión · 9 llamadas totales
----------------------------------------------------------------------
  Tiempo  Herramienta  Duración     Gráfico               Detalle
----------------------------------------------------------------------
  t+ 0s  📖 Read     120 ms  ✔ ■                     src/router/mod.rs
  t+ 1s  🔍 Grep     340 ms  ✔ ■■■                   fn dispatch_request
  t+ 3s  ✏️  Edit     180 ms  ✔ ■                     Añadir match para rutas dinámicas con pa…
  t+ 5s  ⚡ Bash    2450 ms  ✖ ■■■■■■■■■■■■■■■■■■■■  cargo test --test router_tests
  t+ 9s  📖 Read      95 ms  ✔ ■                     tests/router_tests.rs
  t+10s  ✏️  Edit     110 ms  ✔ ■                     Corregir prefijo normalizado en parse_se…
  t+12s  ⚡ Bash    1890 ms  ✔ ■■■■■■■■■■■■■■■       cargo test --test router_tests
  t+15s  📝 Write    150 ms  ✔ ■                     docs/ROUTING.md con especificación de en…
  t+16s  ⚡ Bash     620 ms  ✔ ■■■■■                 git status && cargo clippy
----------------------------------------------------------------------
  RESUMEN & MÉTRICAS POR HERRAMIENTA
----------------------------------------------------------------------
  ⚡ Bash    |  3 llamadas |   4960 ms acum. | prom: 1653 ms | errores: 1
  📖 Read    |  2 llamadas |    215 ms acum. | prom:  107 ms | errores: 0
  ✏️  Edit    |  2 llamadas |    290 ms acum. | prom:  145 ms | errores: 0
  🔍 Grep    |  1 llamadas |    340 ms acum. | prom:  340 ms | errores: 0
  📝 Write   |  1 llamadas |    150 ms acum. | prom:  150 ms | errores: 0
----------------------------------------------------------------------
  Total: 5955 ms (5.96s) de cómputo en herramientas | Errores: 1
```

---

### 2. Probar `stibios-init`

```bash
# Ver plantillas y metodologías disponibles
cargo run -p stibios-init -- list

# Simular la creación de un nuevo proyecto SDD (sin escribir en disco)
cargo run -p stibios-init -- new mi-agente --dry-run

# Crear un proyecto con metodología TDD
cargo run -p stibios-init -- new mi-agente-tdd --metodologia tdd

# Comprobar si el directorio actual cumple con la estructura Stibios
cargo run -p stibios-init -- check .
```

---

## 🧪 Pruebas Automatizadas

El proyecto incluye tests unitarios y de integración para cada componente:

```bash
$ cargo test
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 👩‍💻 Autora

Desarrollado con ❤️ por **Carolina** ([@Estivbi](https://github.com/Estivbi)) como proyecto integrador y pedagógico de aprendizaje de Rust.
Licencia dual MIT / Apache-2.0.
