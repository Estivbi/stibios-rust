// stibios-init · Pipeline de Ejecución con Closures
// Hito 6 (Lecciones 21-23): Closures dinámicos, Higher-order functions y Box<dyn Fn>

use crate::fs_builder::ErrorInit;
use crate::modelo::{ConfiguracionProyecto, ElementoEstructura};

/// Contexto mutable que viaja a lo largo del pipeline
pub struct ContextoEjecucion {
    pub config: ConfiguracionProyecto,
    pub elementos: Vec<ElementoEstructura>,
    pub pasos_completados: Vec<String>,
}

impl ContextoEjecucion {
    pub fn nuevo(config: ConfiguracionProyecto) -> Self {
        Self {
            config,
            elementos: Vec::new(),
            pasos_completados: Vec::new(),
        }
    }
}

/// Tipo alias para un paso del pipeline implementado mediante un Closure
type PasoPipeline = Box<dyn Fn(&mut ContextoEjecucion) -> Result<(), ErrorInit>>;

/// Pipeline desacoplado que encadena tareas secuenciales
pub struct Pipeline {
    nombre: String,
    pasos: Vec<(String, PasoPipeline)>,
}

impl Pipeline {
    pub fn nuevo(nombre: impl Into<String>) -> Self {
        Self {
            nombre: nombre.into(),
            pasos: Vec::new(),
        }
    }

    /// Lección 21: Acepta un closure genérico `F` y lo almacena dentro de un `Box`
    pub fn agregar_paso<F>(&mut self, descripcion: impl Into<String>, paso: F) -> &mut Self
    where
        F: Fn(&mut ContextoEjecucion) -> Result<(), ErrorInit> + 'static,
    {
        self.pasos.push((descripcion.into(), Box::new(paso)));
        self
    }

    /// Ejecuta todos los pasos secuencialmente; se detiene ante el primer error
    pub fn ejecutar(&self, ctx: &mut ContextoEjecucion) -> Result<(), ErrorInit> {
        println!("\n\x1b[1;36m▶ Iniciando Pipeline: {}\x1b[0m", self.nombre);

        for (desc, paso) in &self.pasos {
            print!("  • {desc} ... ");
            match paso(ctx) {
                Ok(()) => {
                    println!("\x1b[32mOK\x1b[0m");
                    ctx.pasos_completados.push(desc.clone());
                }
                Err(err) => {
                    println!("\x1b[31mFALLÓ\x1b[0m");
                    return Err(err);
                }
            }
        }

        println!("\x1b[1;32m✔ Pipeline finalizado con éxito.\x1b[0m\n");
        Ok(())
    }
}
