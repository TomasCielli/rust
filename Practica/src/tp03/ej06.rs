struct Examen {
    materia: String,
    nota: f32,
}
impl Examen {
    fn new(materia: String, nota: f32) -> Examen {
        Examen { materia, nota }
    }
}

struct Estudiante {
    nombre: String,
    id: u64,
    examenes: Option<Vec<Examen>>,
}

impl Estudiante {
    fn new(nombre: String, id: u64, examenes: Option<Vec<Examen>>) -> Estudiante {
        Estudiante {
            nombre,
            id,
            examenes,
        }
    }

    fn obtener_promedio(&self) -> f32 {
        let mut total: f32 = 0.0;
        let mut cant: f32 = 0.0;

        match &self.examenes {
            Some(examenes) => {
                for examen in examenes {
                    total += examen.nota;
                    cant += 1.0;
                }
            }

            None => {}
        }
        if total == 0.0 {
            return 0.0;
        }
        return total / cant;
    }

    fn obtener_calificacion_mas_alta(&self) -> f32 {
        let mut max: f32 = 0.0;

        match &self.examenes {
            Some(examenes) => {
                for examen in examenes {
                    if examen.nota > max {
                        max = examen.nota;
                    }
                }
            }
            None => (),
        }

        return max;
    }

    fn obtener_calificacion_mas_baja(&self) -> f32 {
        let mut min: f32 = 99999.0;

        match &self.examenes {
            Some(examenes) => {
                for examen in examenes {
                    if examen.nota < min {
                        min = examen.nota;
                    }
                }
            }
            None => (),
        }

        return min;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examen() {
        let examen = Examen::new("Matematica".to_string(), 8.5);
        assert_eq!(examen.materia, "Matematica");
        assert_eq!(examen.nota, 8.5);
    }

    #[test]
    fn test_estudiante() {
        let examen1 = Examen::new("Matematica".to_string(), 8.5);
        let examen2 = Examen::new("Historia".to_string(), 9.0);
        let examenes = Some(vec![examen1, examen2]);
        let estudiante = Estudiante::new("Juan".to_string(), 12345, examenes);

        assert_eq!(estudiante.nombre, "Juan");
        assert_eq!(estudiante.id, 12345);
        assert_eq!(estudiante.obtener_promedio(), 8.75);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(), 9.0);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), 8.5);
    }

    #[test]
    fn test_estudiante_sin_examenes() {
        let estudiante = Estudiante::new("Juan".to_string(), 12345, None); // <--- No hay vector de examenes

        assert_eq!(estudiante.nombre, "Juan");
        assert_eq!(estudiante.id, 12345);
        assert_eq!(estudiante.obtener_promedio(), 0.0);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(), 0.0);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), 99999.0);
    }

    #[test]
    fn test_estudiante_con_examenes_vacios() {
        let examenes = Some(vec![]); // <--- Hay un vector de examenes, pero esta vacio
        let estudiante = Estudiante::new("Juan".to_string(), 12345, examenes);

        assert_eq!(estudiante.nombre, "Juan");
        assert_eq!(estudiante.id, 12345);
        assert_eq!(estudiante.obtener_promedio(), 0.0);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(), 0.0);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), 99999.0);
    }
}
