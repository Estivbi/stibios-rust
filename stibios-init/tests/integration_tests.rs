// stibios-init · Tests de Integración
// Hito 7: Tests de validación de nombres, plantillas y pipeline

use stibios_init::{
    construir_estructura,
    modelo::{ConfiguracionProyecto, Metodologia},
    obtener_plantilla,
    pipeline::{ContextoEjecucion, Pipeline},
    validador::{validar_nombre_proyecto, ErrorValidacion},
};
use std::path::PathBuf;

#[test]
fn test_validacion_nombres_correctos() {
    assert!(validar_nombre_proyecto("mi-proyecto").is_ok());
    assert!(validar_nombre_proyecto("agente_rust_v2").is_ok());
    assert!(validar_nombre_proyecto("stibios-app").is_ok());
}

#[test]
fn test_validacion_nombres_incorrectos() {
    assert_eq!(validar_nombre_proyecto(""), Err(ErrorValidacion::Vacio));
    assert_eq!(validar_nombre_proyecto("a"), Err(ErrorValidacion::DemasiadoCorto));
    assert_eq!(
        validar_nombre_proyecto("123-proyecto"),
        Err(ErrorValidacion::EmpiezaPorNumeroOGuion)
    );
    assert!(matches!(
        validar_nombre_proyecto("proyecto con espacios"),
        Err(ErrorValidacion::CaracterInvalido(' '))
    ));
    assert_eq!(
        validar_nombre_proyecto("cargo"),
        Err(ErrorValidacion::PalabraReservada("cargo"))
    );
    assert_eq!(
        validar_nombre_proyecto("let"),
        Err(ErrorValidacion::PalabraReservada("let"))
    );
}

#[test]
fn test_plantillas_elementos() {
    let p_sdd = obtener_plantilla(Metodologia::Sdd);
    let elems_sdd = p_sdd.generar_elementos("test-sdd");
    assert!(elems_sdd.iter().any(|e| e.ruta_relativa == "CLAUDE.md"));
    assert!(elems_sdd.iter().any(|e| e.ruta_relativa == "DESIGN.md"));
    assert!(elems_sdd.iter().any(|e| e.ruta_relativa == "specs"));

    let p_tdd = obtener_plantilla(Metodologia::Tdd);
    let elems_tdd = p_tdd.generar_elementos("test-tdd");
    assert!(elems_tdd.iter().any(|e| e.ruta_relativa.starts_with("tests")));

    let p_min = obtener_plantilla(Metodologia::Minimal);
    let elems_min = p_min.generar_elementos("test-min");
    assert!(elems_min.len() < elems_sdd.len());
}

#[test]
fn test_pipeline_dry_run() {
    let config = ConfiguracionProyecto {
        nombre: "proyecto-prueba".to_string(),
        directorio_destino: PathBuf::from("target/test_dry_run"),
        metodologia: Metodologia::Sdd,
        dry_run: true,
        sobrescribir: true,
    };

    let mut ctx = ContextoEjecucion::nuevo(config);
    let mut pipeline = Pipeline::nuevo("Test Pipeline");

    pipeline
        .agregar_paso("Validar", |ctx| {
            validar_nombre_proyecto(&ctx.config.nombre).map(|_| ()).map_err(Into::into)
        })
        .agregar_paso("Generar", |ctx| {
            let plantilla = obtener_plantilla(ctx.config.metodologia);
            ctx.elementos = plantilla.generar_elementos(&ctx.config.nombre);
            Ok(())
        })
        .agregar_paso("Construir dry-run", |ctx| {
            construir_estructura(&ctx.config, &ctx.elementos)?;
            Ok(())
        });

    let resultado = pipeline.ejecutar(&mut ctx);
    assert!(resultado.is_ok());
    assert_eq!(ctx.pasos_completados.len(), 3);
}
