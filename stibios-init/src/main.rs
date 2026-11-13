// stibios-init · Hito 1
// Cubre las lecciones 1 a 5 del curso. De momento solo muestra qué estructura
// de proyecto crearía; todavía no escribe nada en disco.

// Lección 2: constantes globales en MAYÚSCULAS con tipo explícito.
const VERSION_PLANTILLA: u32 = 1;

// Lección 3: una tupla agrupa (ruta, descripción) y un array fija la longitud.
const ESTRUCTURA: [(&str, &str); 6] = [
    ("CLAUDE.md", "contexto del proyecto para Claude Code"),
    (".claude/agents/", "agentes especializados (arquitecto, frontend, backend...)"),
    (".claude/skills/", "skills del kit"),
    ("specs/", "especificaciones para trabajar con SDD"),
    ("tests/", "tests primero (TDD)"),
    ("DESIGN.md", "referencia de diseño"),
];

// Lección 4: la función devuelve su última expresión, sin `return` ni punto y coma.
fn es_carpeta(ruta: &str) -> bool {
    ruta.ends_with('/')
}

fn main() {
    // Lección 1: println! con interpolación.
    println!("stibios-init · plantilla v{VERSION_PLANTILLA}");
    println!("Se crearía esta estructura:\n");

    // Lección 5: `for` recorre el array; `if` decide el icono según el tipo de ruta.
    for (ruta, descripcion) in ESTRUCTURA {
        let tipo = if es_carpeta(ruta) { "carpeta" } else { "archivo" };
        println!("  {ruta:<18} ({tipo}) · {descripcion}");
    }
}
