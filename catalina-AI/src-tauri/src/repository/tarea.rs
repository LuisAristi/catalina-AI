use std::path::PathBuf;
use rusqlite::{params, Connection, OptionalExtension, Result};

use crate::models::tarea::Tarea;

pub struct TareaRepository {
    db_path: PathBuf,
}

impl TareaRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let repo = Self {
            db_path: PathBuf::from(db_path),
        };
        repo.init_db()?;
        Ok(repo)
    }

    fn connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }

    fn init_db(&self) -> Result<()> {
        let conn = self.connection()?;

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS tareas (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                titulo TEXT NOT NULL,
                descripcion TEXT NOT NULL,
                fecha_creacion TEXT NOT NULL,
                fecha_vencimiento TEXT NOT NULL,
                prioridad INTEGER NOT NULL,
                lugar TEXT NOT NULL,
                completada INTEGER NOT NULL
            )
            ",
            (),
        )?;

        Ok(())
    }

    /* aqui aprendo que &self es una referencia a la instancia actual de 
    la estructura TareaRepository, lo que permite acceder a sus métodos y 
    propiedades sin tomar posesión de la instancia. Esto es útil para mantener 
    la eficiencia y evitar mover o copiar la estructura completa cada vez que se 
    llama a un método*/
    pub fn crear(&self, tarea: &Tarea) -> Result<i64> {
        let conn = self.connection()?;

        conn.execute(
            "
            INSERT INTO tareas (
                titulo,
                descripcion,
                fecha_creacion,
                fecha_vencimiento,
                prioridad,
                lugar,
                completada
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ",
            params![
                tarea.titulo,
                tarea.descripcion,
                tarea.fecha_creacion,
                tarea.fecha_vencimiento,
                tarea.prioridad,
                tarea.lugar,
                if tarea.completada { 1 } else { 0 }
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn listar(&self) -> Result<Vec<Tarea>> {
        let conn = self.connection()?;
        let mut stmt = conn.prepare(
            "
            SELECT id, titulo, descripcion, fecha_creacion, fecha_vencimiento, prioridad, lugar, completada
            FROM tareas
            ORDER BY id DESC
            ",
        )?;

        let tareas = stmt
            .query_map([], |row| {
                Ok(Tarea {
                    id: row.get(0)?,
                    titulo: row.get(1)?,
                    descripcion: row.get(2)?,
                    fecha_creacion: row.get(3)?,
                    fecha_vencimiento: row.get(4)?,
                    prioridad: row.get(5)?,
                    lugar: row.get(6)?,
                    completada: row.get::<_, i32>(7)? != 0,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(tareas)
    }

    pub fn obtener_por_id(&self, id: i32) -> Result<Option<Tarea>> {
        let conn = self.connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, titulo, descripcion, fecha_creacion, fecha_vencimiento, prioridad, lugar, completada FROM tareas WHERE id = ?1",
        )?;

        let tarea = stmt
            .query_row([id], |row| {
                Ok(Tarea {
                    id: row.get(0)?,
                    titulo: row.get(1)?,
                    descripcion: row.get(2)?,
                    fecha_creacion: row.get(3)?,
                    fecha_vencimiento: row.get(4)?,
                    prioridad: row.get(5)?,
                    lugar: row.get(6)?,
                    completada: row.get::<_, i32>(7)? != 0,
                })
            })
            .optional()?;

        Ok(tarea)
    }

    pub fn actualizar(&self, tarea: &Tarea) -> Result<usize> {
        let conn = self.connection()?;

        conn.execute(
            "
            UPDATE tareas
            SET titulo = ?1,
                descripcion = ?2,
                fecha_creacion = ?3,
                fecha_vencimiento = ?4,
                prioridad = ?5,
                lugar = ?6,
                completada = ?7
            WHERE id = ?8
            ",
            params![
                tarea.titulo,
                tarea.descripcion,
                tarea.fecha_creacion,
                tarea.fecha_vencimiento,
                tarea.prioridad,
                tarea.lugar,
                if tarea.completada { 1 } else { 0 },
                tarea.id,
            ],
        )
    }

    pub fn eliminar(&self, id: i32) -> Result<usize> {
        let conn = self.connection()?;
        conn.execute("DELETE FROM tareas WHERE id = ?1", [id])
    }

    pub fn eliminar_todas(&self) -> Result<usize> {
        let conn = self.connection()?;
        conn.execute("DELETE FROM tareas", [])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puede_crear_y_listar_tareas() {
        let repo = TareaRepository::new("test_tareas.db").unwrap();
        repo.eliminar_todas().unwrap();

        let tarea = Tarea::new(
            0,
            "Estudiar SQLite".to_string(),
            "Revisar el flujo de la base de datos".to_string(),
            "2026-08-16T09:00:00Z".to_string(),
            "2026-08-18T09:00:00Z".to_string(),
            2,
            "Casa".to_string(),
            false,
        );

        let id = repo.crear(&tarea).unwrap();
        let tareas = repo.listar().unwrap();

        assert!(id > 0);
        assert_eq!(tareas.len(), 1);
        assert_eq!(tareas[0].titulo, "Estudiar SQLite");
    }
}