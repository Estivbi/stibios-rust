// stibios-init · CLI Principal con Clap
// Hito 7 (Lecciones 24-26): Clap CLI moderno, subcomandos y manejo global de errores

use clap::{Parser, Subcommand, ValueEnum};
use stibios_init::{
    construir_estructura, fs_builder::ErrorInit, obtener_plantilla, validar_nombre_proyecto,
    ConfiguracionProyecto, ContextoEjecucion, Metodologia, Pipeline, VERSION_STIBIOS_INIT,
};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "stibios-init")]
#[command(author = "Carolina <carolinarb1994@gmail.com>")]
#[command(version = VERSION_STIBIOS_INIT)]
#[command(about = "Generador de scaffolding Stibios para proyectos con SDD/TDD y agentes", long_about = None)]
struct Cli {
    #[command(subcommand)]
    comando: Comandos,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum MetodologiaArg {
    Sdd,
    Tdd,
    Minimal,
}

impl From<MetodologiaArg> for Metodologia {
    fn from(arg: MetodologiaArg) -> Self {
        match arg {
            MetodologiaArg::Sdd => Metodologia::Sdd,
            MetodologiaArg::Tdd => Metodologia::Tdd,
            MetodologiaArg::Minimal => Metodologia::Minimal,
        }
    }
}

#[derive(Subcommand, Debug)]
enum Comandos {
    /// Crear un nuevo proyecto inicializado con la metodología elegida
    New {
        /// Nombre del proyecto (identificador válido en minúsculas y guiones)
        nombre: String,

        /// Metodología de desarrollo (sdd, tdd, minimal)
        #[arg(short, long, value_enum, default_value_t = MetodologiaArg::Sdd)]
        metodologia: MetodologiaArg,

        /// Ruta destino personalizada (por defecto crea una carpeta con el nombre en el dir actual)
        #[arg(short, long)]
        dir: Option<PathBuf>,

        /// Simular la creación sin escribir nada en disco
        #[arg(long, default_value_t = false)]
        dry_run: bool,

        /// Forzar sobrescritura si el directorio ya existe
        #[arg(short, long, default_value_t = false)]
        force: bool,
    },

    /// Listar las plantillas y metodologías disponibles
    List,

    /// Comprobar si un directorio cumple con la estructura Stibios
    Check {
        /// Directorio a comprobar (por defecto el actual '.')
        #[arg(default_value = ".")]
        dir: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.comando {
        Comandos::New {
            nombre,
            metodologia,
            dir,
            dry_run,
            force,
        } => {
            let met: Metodologia = metodologia.into();
            let destino = dir.unwrap_or_else(|| PathBuf::from(&nombre));

            let config = ConfiguracionProyecto {
                nombre: nombre.clone(),
                directorio_destino: destino,
                metodologia: met,
                dry_run,
                sobrescribir: force,
            };

            let mut ctx = ContextoEjecucion::nuevo(config);

            // Construimos el pipeline con closures (Hito 6)
            let mut pipeline = Pipeline::nuevo("Inicialización de Proyecto Stibios");

            pipeline
                .agregar_paso("Validación del nombre de proyecto", |ctx| {
                    validar_nombre_proyecto(&ctx.config.nombre)
                        .map(|_| ())
                        .map_err(ErrorInit::Validacion)
                })
                .agregar_paso("Carga de plantilla y elementos", |ctx| {
                    let plantilla = obtener_plantilla(ctx.config.metodologia);
                    ctx.elementos = plantilla.generar_elementos(&ctx.config.nombre);
                    Ok(())
                })
                .agregar_paso("Construcción en sistema de archivos", |ctx| {
                    construir_estructura(&ctx.config, &ctx.elementos)?;
                    Ok(())
                });

            pipeline.ejecutar(&mut ctx)?;

            println!(
                "\x1b[1;32mProyecto '{}' listo con metodología {} en '{}'!\x1b[0m\n",
                ctx.config.nombre,
                ctx.config.metodologia.como_str(),
                ctx.config.directorio_destino.display()
            );
        }

        Comandos::List => {
            println!("\n\x1b[1;36m======================================================================\x1b[0m");
            println!("\x1b[1;37m  PLANTILLAS Y METODOLOGÍAS DISPONIBLES (stibios-init v{})\x1b[0m", VERSION_STIBIOS_INIT);
            println!("\x1b[1;36m======================================================================\x1b[0m\n");

            let plantillas = [Metodologia::Sdd, Metodologia::Tdd, Metodologia::Minimal];
            for met in plantillas {
                let p = obtener_plantilla(met);
                println!("  \x1b[1;33m• {:<10}\x1b[0m {}", met.como_str(), p.nombre());
                println!("    \x1b[2m{}\x1b[0m", p.descripcion());
                println!("    Elementos generados: {}\n", p.total_elementos("demo"));
            }
        }

        Comandos::Check { dir } => {
            comprobar_directorio(&dir);
        }
    }

    Ok(())
}

fn comprobar_directorio(dir: &Path) {
    println!("\n\x1b[1;36mComprobando conformidad Stibios en: {}\x1b[0m", dir.display());

    let requeridos = [
        ("stibios.config.yml", "Configuración central del kit"),
        ("AGENTS.md", "Contrato del equipo de agentes"),
        ("CLAUDE.md", "Contexto de agentes de IA"),
        ("docs/specs", "Carpeta de especificaciones SDD"),
        ("docs/backlog", "Carpeta de backlog local de tickets"),
        ("tests", "Suite de pruebas TDD"),
        (".gitignore", "Configuración de exclusión git"),
    ];

    let mut conformes = 0;
    for (elem, desc) in requeridos {
        let ruta = dir.join(elem);
        if ruta.exists() {
            println!("  \x1b[32m✔ Presente:\x1b[0m  {:<18} ({desc})", elem);
            conformes += 1;
        } else {
            println!("  \x1b[33m⚠ Ausente:\x1b[0m   {:<18} ({desc})", elem);
        }
    }

    let porcentaje = (conformes as f64 / requeridos.len() as f64) * 100.0;
    println!("\n  Conformidad: \x1b[1m{conformes}/{}\x1b[0m ({:.0}%)\n", requeridos.len(), porcentaje);
}
