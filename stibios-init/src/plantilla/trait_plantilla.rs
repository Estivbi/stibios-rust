// stibios-init · Trait para Generación de Plantillas
// Hito 5 (Lecciones 17-20): Traits, Métodos por Defecto e Implementaciones

use crate::modelo::ElementoEstructura;

/// Lección 17: Definición de un trait que establece el contrato que cualquier plantilla debe cumplir.
pub trait GeneradorPlantilla {
    /// Nombre descriptivo de la plantilla
    fn nombre(&self) -> &'static str;

    /// Descripción breve de la metodología
    fn descripcion(&self) -> &'static str;

    /// Lista de elementos (carpetas y archivos con contenido) que componen la plantilla
    fn generar_elementos(&self, nombre_proyecto: &str) -> Vec<ElementoEstructura>;

    /// Lección 17: Método con implementación por defecto en el trait
    fn total_elementos(&self, nombre_proyecto: &str) -> usize {
        self.generar_elementos(nombre_proyecto).len()
    }
}
