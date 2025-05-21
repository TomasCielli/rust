/*

📘 TP3 - Ejercicio 6 - Funcionalidad adicional

Deberán agregar una funcionalidad al ejercicio que permita retornar un informe detallado del rendimiento académico de un estudiante.

Este informe debe incluir:
Nombre e identificación del estudiante.,
Cantidad total de exámenes rendidos.,
Promedio general de notas.,
Nota más alta y la materia correspondiente.,
Nota más baja y la materia correspondiente.,

🔧 La funcionalidad deberá implementarse como un método asociado del estudiante llamado generar_informe.
En caso de que el estudiante no haya rendido ningún examen, no debe retornarse ningún informe.

📌 Requisitos:
La funcionalidad debe integrarse naturalmente con las estructuras ya definidas.,
Se espera una solución robusta ante distintas situaciones, incluyendo estudiantes sin exámenes.,
Se debe acompañar con al menos dos tests unitarios que validen su correcto funcionamiento.,

🛠️ Durante la evaluación:
Deberán presentar una estrategia clara para encarar esta funcionalidad, explicando cómo organizarán los datos y qué métodos planean utilizar.
La versión final (V2) deberá ajustarse fielmente a la estrategia presentada.

*/
#[derive(Clone)]
struct Informe {
    nombre: String,
    id: u64,
    cant_examenes: usize,
    promedio: f32,
    materia_alta: Examen, //NO SON OPTION XQ SI SE CREO EL INFORME SIGNIFICA QUE COMO
    materia_baja: Examen, //MINIMO HAY UN EXAMEN
}
impl Informe {
    fn new(
        nombre: String,
        id: u64,
        cant_examenes: usize,
        promedio: f32,
        materia_alta: Examen,
        materia_baja: Examen,
    ) -> Informe {
        Informe {
            nombre,
            id,
            cant_examenes,
            promedio,
            materia_alta,
            materia_baja,
        }
    }
}

#[derive(Clone)]
struct Examen {
    materia: String,
    nota: f32,
}
impl Examen {
    fn new(materia: String, nota: f32) -> Examen {
        Examen { materia, nota }
    }
}

#[derive(Clone)]
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

    /*

    ACTUALIZACIONES DE V.2

       • Meti todas la variables para crear el informe dentro del if, para que no sean declaradas innecesariamente en caso de no haber examenes

       • Borre el struct "Materia" ya que es exactamente el mismo que el de "Examen"

       • Borre variables nota_alta y nota_baja, ya que ahora se calculan directamente dentro del .find() para ahorrar lineas

       • Las variables materia_alta y materia_baja dejaron de ser Option, ya que al estar dentro del scope del if si o si deberia haber un examen cargado al momento de declararlo
         y por lo tanto, nunca van a ser None

       • Borre el "if let Some(materia) = alta/baja {...}" porque siempre va a encontrarse una nota minima y una maxima

       • Emprolije el codigo, devolviendo directamente un Informe sin necesidad de declarar una nueva variable

       • Agregue los test

    */
    fn entregar_informe(&self) -> Option<Informe> {
        if let Some(examenes) = &self.examenes {
            //Todo dentro del if, asi si esta vacio no crea variables inutiles
            let nombre = &self.nombre;
            let id = self.id;
            let promedio = self.obtener_promedio();
            let cant_examenes = examenes.len();
            let materia_alta = examenes
                .iter()
                .find(|x| x.nota == self.obtener_calificacion_mas_alta())
                .expect("Deberia tener un examen cargado como minimo."); //Se sabe que como minimo hay 1 examen xq entro al if
            let materia_baja = examenes
                .iter()
                .find(|x| x.nota == self.obtener_calificacion_mas_baja())
                .expect("Deberia tener un examen cargado como minimo."); //Lo mismo

            return Some(Informe::new(
                //CREA EL INFORME Y LO DEVUELVE
                nombre.to_string(),
                id,
                cant_examenes,
                promedio,
                materia_alta.clone(),
                materia_baja.clone(),
            ));
        }
        None //DEVUELVE NONE
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

    //TESTS V.2

    #[test]
    fn test_entregar_informe_con_datos_validos() {
        //Creo los examenes
        let examen1 = Examen::new("Matematica".to_string(), 10.0);
        let examen2 = Examen::new("Historia".to_string(), 9.0);
        let examen3 = Examen::new("Literatura".to_string(), 7.0);
        //Los cargo en un vector
        let examenes = Some(vec![examen1.clone(), examen2.clone(), examen3.clone()]);
        //Creo un estudiante con los examenes cargados
        let estudiante = Estudiante::new("Tomas".to_string(), 12345, examenes);

        let informe = estudiante
            .entregar_informe()
            .expect("Deberia haber un informe");

        assert_eq!(informe.nombre, "Tomas");
        assert_eq!(informe.id, 12345);
        assert_eq!(informe.cant_examenes, 3);
        assert_eq!(informe.promedio, (10.0 + 9.0 + 7.0) / 3.0);
        assert_eq!(informe.materia_alta.materia, "Matematica"); // Nota mas alta: Matematica (10.0)
        assert_eq!(informe.materia_alta.nota, 10.0);
        assert_eq!(informe.materia_baja.materia, "Literatura"); // Nota mas baja: Literatura (7.0)
        assert_eq!(informe.materia_baja.nota, 7.0);
    }

    #[test]
    fn test_entregar_informe_sin_examenes() {
        let estudiante = Estudiante::new("Tomas".to_string(), 12345, None);
        let informe = estudiante.entregar_informe();

        assert!(informe.is_none()); //No se genera el informe xq no hay examenes para crearlo
    }

    #[test]
    fn test_entregar_informe_notas_repetidas() {
        //Se crean los examenes (con las mimas notas maximas y minimas)
        let examen1 = Examen::new("Fisica".to_string(), 10.0);
        let examen2 = Examen::new("Quimica".to_string(), 10.0);
        let examen3 = Examen::new("Biologia".to_string(), 4.0);
        let examen4 = Examen::new("Geografia".to_string(), 4.0);

        //Se cargan 4 examenes al vector
        let examenes = Some(vec![
            examen1.clone(),
            examen2.clone(),
            examen3.clone(),
            examen4.clone(),
        ]);
        //Se crea al estudiante con el vector cargado
        let estudiante = Estudiante::new("Tomas".to_string(), 12345, examenes);

        //Se crea el informe
        let informe = estudiante
            .entregar_informe()
            .expect("Deberia haber un informe");

        assert_eq!(informe.nombre, "Tomas");
        assert_eq!(informe.id, 12345);
        assert_eq!(informe.cant_examenes, 4);
        assert_eq!(informe.promedio, 7.0); // Promedio: (10 + 10 + 4 + 4) /4 = 7
        assert!(informe.materia_alta.nota == 10.0); //La nota maxima de cualquiera de las dos materias (Fisica / Quimica)
        assert!(informe.materia_baja.nota == 4.0); //La nota minima de cualquiera de las dos materias (Biologia / Geografia)
    }
}
