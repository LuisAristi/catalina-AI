use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Tarea {
    pub id: i32,
    pub titulo: String,
    pub descripcion: String,
    pub fecha_creacion: String,
    pub fecha_vencimiento: String,
    pub prioridad: i32,
    pub lugar: String,
    pub completada: bool,
}

impl Tarea {
    pub fn new(
        id: i32,
        titulo: String,
        descripcion: String,
        fecha_creacion: String,
        fecha_vencimiento: String,
        prioridad: i32,
        lugar: String,
        completada: bool,
    ) -> Self {
        Tarea {
            id,
            titulo,
            descripcion,
            fecha_creacion,
            fecha_vencimiento,
            prioridad,
            lugar,
            completada,
        }
    }
}
