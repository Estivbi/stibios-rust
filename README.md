# stibios-rust

Dos proyectos en Rust que se construyen siguiendo el curso de [aprenderust.dev](https://aprenderust.dev/hola-mundo), hito a hito:

- **agent-replay**: reproductor de sesiones de agentes en el terminal (línea de tiempo de herramientas, tiempos y archivos; TUI al final).
- **stibios-init**: CLI que crea un proyecto nuevo con la forma de trabajar de stibios-kit (SDD/TDD, agentes, skills, DESIGN.md).

El código es propio y está comentado en español con la lección que usa cada concepto, para poder estudiarlo mientras se hace el curso.

## Cómo ejecutarlo

```bash
cargo run -p agent-replay
cargo run -p stibios-init
```

## Hitos

| Hito | Lecciones | agent-replay | stibios-init | Estado |
|---|---|---|---|---|
| 1 | 1-5: main, variables, tipos, funciones, control de flujo | Línea de tiempo con eventos de ejemplo | Muestra la estructura que crearía | Hecho |
| 2 | 6-8: ownership, referencias, slices | Funciones que reciben `&[Evento]` y `&str` sin copiar | Validar el nombre del proyecto con `&str` | Pendiente |
| 3 | 9-12: structs, métodos, enums, Option | `struct Evento`, `enum Herramienta`, `Option` para campos ausentes | `struct Plantilla` y `enum TipoEntrada` | Pendiente |
| 4 | 13-16: Vec, String, HashMap, Result y `?` | Cargar y agrupar eventos por herramienta; errores al leer el archivo | Crear carpetas y archivos con `Result` | Pendiente |
| 5 | 17-20: genéricos, traits, lifetimes, módulos | Módulos `modelo`, `lector`, `vista` | Módulos y trait para plantillas | Pendiente |
| 6 | 21-23: closures, iteradores, smart pointers | Filtros y estadísticas con iteradores | Pipeline de pasos con closures | Pendiente |
| 7 | 24-26: concurrencia, tests, ecosistema | TUI con `ratatui`, lectura en otro hilo, tests | CLI con `clap`, tests de integración | Pendiente |

## Notas

- Los exámenes del curso (uno por bloque) se hacen en la web; el código de aquí cubre los mismos conceptos aplicados a los proyectos.
- No se copia el texto ni el código del curso. Solo se sigue su orden.
- Antes de publicar en GitHub, buscar si ya existe algo parecido para el README y ajustar el posicionamiento.
