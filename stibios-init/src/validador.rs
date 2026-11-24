// stibios-init · Validador de Nombres de Proyecto
// Hito 2 (Lecciones 6-8): Ownership, referencias y slices (&str)
// Hito 4 (Lección 16): Result<T, E> para validación idiomática sin panics

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorValidacion {
    Vacio,
    DemasiadoCorto,
    DemasiadoLargo,
    EmpiezaPorNumeroOGuion,
    CaracterInvalido(char),
    PalabraReservada(&'static str),
}

impl fmt::Display for ErrorValidacion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorValidacion::Vacio => write!(f, "El nombre no puede estar vacío"),
            ErrorValidacion::DemasiadoCorto => write!(f, "El nombre debe tener al menos 2 caracteres"),
            ErrorValidacion::DemasiadoLargo => write!(f, "El nombre no puede superar los 64 caracteres"),
            ErrorValidacion::EmpiezaPorNumeroOGuion => {
                write!(f, "El nombre debe comenzar por una letra (a-z o A-Z)")
            }
            ErrorValidacion::CaracterInvalido(c) => write!(
                f,
                "El carácter '{c}' no está permitido. Solo se permiten letras, números, '-' y '_'"
            ),
            ErrorValidacion::PalabraReservada(palabra) => {
                write!(f, "'{palabra}' es una palabra reservada del sistema o de Rust")
            }
        }
    }
}

impl std::error::Error for ErrorValidacion {}

/// Lección 7 & 8: Recibe una referencia a slice `&str` y devuelve el mismo slice prestado si es válido.
/// Cero asignaciones en memoria heap.
pub fn validar_nombre_proyecto(nombre: &str) -> Result<&str, ErrorValidacion> {
    if nombre.is_empty() {
        return Err(ErrorValidacion::Vacio);
    }

    if nombre.len() < 2 {
        return Err(ErrorValidacion::DemasiadoCorto);
    }

    if nombre.len() > 64 {
        return Err(ErrorValidacion::DemasiadoLargo);
    }

    // Lección 8: Obtenemos el primer carácter usando iterador sobre chars
    let primer_char = match nombre.chars().next() {
        Some(c) => c,
        None => return Err(ErrorValidacion::Vacio),
    };

    if !primer_char.is_ascii_alphabetic() {
        return Err(ErrorValidacion::EmpiezaPorNumeroOGuion);
    }

    // Lección 5 & 6: Iteramos sobre los caracteres restantes
    for c in nombre.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
            return Err(ErrorValidacion::CaracterInvalido(c));
        }
    }

    // Comprobamos palabras reservadas de Rust y paquetes comunes
    const PALABRAS_RESERVADAS: &[&str] = &[
        "self", "super", "crate", "fn", "let", "mut", "test", "cargo", "main", "core", "std",
    ];

    for &reservada in PALABRAS_RESERVADAS {
        if nombre.eq_ignore_ascii_case(reservada) {
            return Err(ErrorValidacion::PalabraReservada(reservada));
        }
    }

    Ok(nombre)
}
