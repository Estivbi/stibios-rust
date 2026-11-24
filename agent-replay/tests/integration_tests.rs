// agent-replay · Tests de Integración
// Hito 7: Tests exhaustivos cubriendo ownership, filtros, cálculo de métricas y parsing

use agent_replay::{
    filtro::{filtrar_por_herramienta, filtrar_solo_errores, filtrar_solo_lentos},
    lector::sesion_demo,
    modelo::{EstadoEjecucion, Evento, Herramienta, NivelVelocidad, Sesion},
};

#[test]
fn test_creacion_evento_y_velocidad() {
    let ev_rapido = Evento::nuevo(
        1,
        0,
        Herramienta::Read,
        150,
        Some("file.rs".into()),
        EstadoEjecucion::Exito,
    );
    assert_eq!(ev_rapido.nivel_velocidad(), NivelVelocidad::Rapido);
    assert!(!ev_rapido.es_lento());

    let ev_lento = Evento::nuevo(
        2,
        2,
        Herramienta::Bash,
        3500,
        Some("cargo build".into()),
        EstadoEjecucion::Exito,
    );
    assert_eq!(ev_lento.nivel_velocidad(), NivelVelocidad::Lento);
    assert!(ev_lento.es_lento());
}

#[test]
fn test_estadisticas_sesion() {
    let sesion = sesion_demo();
    let stats = sesion.estadisticas();

    assert_eq!(stats.total_eventos, 9);
    assert!(stats.duracion_total_ms > 0);
    assert_eq!(stats.llamadas_con_error, 1);

    // Verificamos presencia en el desglose de HashMap
    assert!(stats.desglose.contains_key(&Herramienta::Bash));
    assert!(stats.desglose.contains_key(&Herramienta::Read));
    assert!(stats.desglose.contains_key(&Herramienta::Edit));

    let bash_metricas = stats.desglose.get(&Herramienta::Bash).unwrap();
    assert_eq!(bash_metricas.cantidad, 3);
    assert_eq!(bash_metricas.llamadas_con_error, 1);
}

#[test]
fn test_filtros_con_closures_e_iteradores() {
    let sesion = sesion_demo();
    let eventos = sesion.eventos();

    // Filtro por herramienta
    let solo_bash = filtrar_por_herramienta(eventos, &Herramienta::Bash);
    assert_eq!(solo_bash.len(), 3);
    for ev in solo_bash {
        assert_eq!(ev.herramienta, Herramienta::Bash);
    }

    // Filtro solo lentos
    let lentos = filtrar_solo_lentos(eventos);
    assert_eq!(lentos.len(), 1); // 2450 ms
    assert_eq!(lentos[0].duracion_ms, 2450);

    // Filtro solo errores
    let errores = filtrar_solo_errores(eventos);
    assert_eq!(errores.len(), 1);
    assert!(!errores[0].estado.es_exito());
}

#[test]
fn test_serializacion_deserializacion_json() {
    let sesion_orig = sesion_demo();
    let json_str = serde_json::to_string(&sesion_orig).expect("Debe serializar a JSON");
    let sesion_recup: Sesion = serde_json::from_str(&json_str).expect("Debe deserializar desde JSON");

    assert_eq!(sesion_orig.agente, sesion_recup.agente);
    assert_eq!(sesion_orig.eventos.len(), sesion_recup.eventos.len());
}
