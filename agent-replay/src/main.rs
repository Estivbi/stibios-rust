// agent-replay · Hito 1
// Cubre las lecciones 1 a 5 del curso: main y println!, variables, tipos,
// funciones y control de flujo. Todavía no lee sesiones reales: usa eventos
// de ejemplo fijos para practicar la base.

// Lección 2: las constantes van en MAYÚSCULAS y con tipo obligatorio.
const UMBRAL_LENTO_MS: u32 = 2_000;
const UMBRAL_MEDIO_MS: u32 = 500;

// Lección 3: una tupla agrupa valores de tipos distintos.
// (instante en segundos, nombre de la herramienta, duración en ms)
type Evento = (u32, &'static str, u32);

// Lección 3: un array tiene longitud fija y todos sus valores son del mismo tipo.
const EVENTOS_EJEMPLO: [Evento; 5] = [
    (0, "Read", 120),
    (2, "Grep", 340),
    (5, "Bash", 2_450),
    (9, "Edit", 90),
    (11, "Bash", 780),
];

// Lección 4: la última expresión, sin punto y coma, es el valor que devuelve la función.
fn etiqueta_duracion(duracion_ms: u32) -> &'static str {
    // Lección 5: `if` también es una expresión y puede devolver un valor.
    if duracion_ms >= UMBRAL_LENTO_MS {
        "lento"
    } else if duracion_ms >= UMBRAL_MEDIO_MS {
        "medio"
    } else {
        "rápido"
    }
}

// Lección 4: una función puede devolver una tupla para entregar varios datos a la vez.
fn resumen(eventos: &[Evento]) -> (usize, u32) {
    let mut total_ms = 0; // Lección 2: `mut` permite cambiar el valor.
    // Lección 5: `for` recorre una colección.
    for (_, _, duracion) in eventos {
        total_ms += duracion;
    }
    (eventos.len(), total_ms)
}

fn main() {
    // Lección 1: println! es una macro (por el `!`) e interpola valores con {}.
    println!("agent-replay · línea de tiempo de ejemplo");
    println!("----------------------------------------");

    for (segundo, herramienta, duracion) in EVENTOS_EJEMPLO {
        let etiqueta = etiqueta_duracion(duracion);
        println!("t+{segundo:>2}s  {herramienta:<5}  {duracion:>5} ms  [{etiqueta}]");
    }

    let (cantidad, total_ms) = resumen(&EVENTOS_EJEMPLO);
    println!("----------------------------------------");
    println!("{cantidad} llamadas · {total_ms} ms en total");
}
