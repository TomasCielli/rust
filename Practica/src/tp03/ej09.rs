use crate::tp03::ej03::Fecha;
use std::collections::VecDeque;

enum TipoAnimal {
    PERRO,
    GATO,
    CABALLO,
    OTROS,
}

impl TipoAnimal {
    //devuelvo un string con el tipo de animal
    fn mostrar_tipo(&self) -> String {
        match self {
            TipoAnimal::PERRO => "Perro".to_string(),
            TipoAnimal::GATO => "Gato".to_string(),
            TipoAnimal::CABALLO => "Caballo".to_string(),
            TipoAnimal::OTROS => "Otros".to_string(),
        }
    }
}

struct Duenio {
    nombre: String,
    direccion: String,
    telefono: String,
}

impl Duenio {
    fn new(nombre: String, direccion: String, telefono: String) -> Duenio {
        Duenio {
            nombre,
            direccion,
            telefono,
        }
    }

    fn son_distintos(&self, otro: &Duenio) -> bool {
        self.nombre != otro.nombre
            || self.telefono != otro.telefono
            || self.direccion != otro.direccion
    }
}

struct Mascota {
    nombre: String,
    edad: u32,
    tipo: TipoAnimal,
    duenio: Duenio,
}

struct Atencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    fecha_prox_visita: Fecha,
}

impl Atencion {
    fn new(
        mascota: Mascota,
        diagnostico: String,
        tratamiento: String,
        fecha_prox_visita: Fecha,
    ) -> Atencion {
        Atencion {
            mascota,
            diagnostico,
            tratamiento,
            fecha_prox_visita,
        }
    }
}

struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    cola_atencion: VecDeque<Mascota>,
    atenciones: Vec<Atencion>,
}

impl Veterinaria {
    fn new(nombre: String, direccion: String, id: u32) -> Veterinaria {
        Veterinaria {
            nombre,
            direccion,
            id,
            cola_atencion: VecDeque::new(),
            atenciones: Vec::new(),
        }
    }

    // si pusheo atras, entonces la mascota va al fondo de la cola
    fn agregar_atras(&mut self, mascota: Mascota) {
        self.cola_atencion.push_back(mascota);
    }

    //si pusheo adelante, la mascota pasa a ser el primero en la cola
    fn agregar_primero(&mut self, mascota: Mascota) {
        self.cola_atencion.push_front(mascota);
    }

    // si popeo adelante, saco "atiendo" la mascota que esta primera en  cola (devuelvo el pop)
    fn atender_primero(&mut self) -> Option<Mascota> {
        self.cola_atencion.pop_front()
    }

    fn eliminar_de_la_cola(&mut self, mascota: Mascota) -> bool {
        if let Some(pos) = self.cola_atencion.iter().position(|x| {
            x.nombre == mascota.nombre
                && x.edad == mascota.edad
                && x.tipo.mostrar_tipo() == mascota.tipo.mostrar_tipo()
                && !x.duenio.son_distintos(&mascota.duenio)
        }) {
            self.cola_atencion.remove(pos);
            return true;
        } else {
            return false;
        }

        // RETIENE todas las mascotas que no son la que quiero borrar (las que tengan distintos datos internos)
        /* self.cola_atencion.retain(|m| {
            m.nombre != mascota.nombre
                || m.duenio.son_distintos(&mascota.duenio)
                || m.edad != mascota.edad
                || m.tipo.mostrar_tipo() != mascota.tipo.mostrar_tipo()
        }); */
    }

    //pushea (registra) una atencion
    fn registrar_atencion(&mut self, atencion: Atencion) {
        self.atenciones.push(atencion);
    }

    fn buscar_atencion(
        &mut self,
        nombre: String,
        duenio: String,
        telefono: String,
    ) -> Option<&mut Atencion> {
        self.atenciones.iter_mut().find(|x| {
            x.mascota.nombre == nombre
                && x.mascota.duenio.nombre == duenio
                && x.mascota.duenio.telefono == telefono
        })
    }

    fn modificar_diagnostico(
        &mut self,
        nombre: String,
        duenio: String,
        telefono: String,
        diagnostico: String,
    ) {
        if let Some(atencion) = self.buscar_atencion(nombre, duenio, telefono) {
            atencion.diagnostico = diagnostico.to_string();
        }
    }

    fn modificar_la_fecha(
        &mut self,
        nombre: String,
        duenio: String,
        telefono: String,
        fecha: Fecha,
    ) {
        if let Some(atencion) = self.buscar_atencion(nombre, duenio, telefono) {
            atencion.fecha_prox_visita = fecha;
        }
    }

    fn eliminar_atencion(&mut self, nombre: String, duenio: Duenio) -> bool {
        //Devuelve true si lo encuentra/borra, false si no
        if let Some(pos) = self
            .atenciones
            .iter()
            .position(|x| x.mascota.nombre == nombre && !x.mascota.duenio.son_distintos(&duenio))
        //Busca e/ todas las atenciones
        {
            self.atenciones.remove(pos); //Si encontro una con esos datos, la borra
            return true; //y devuelve true
        } else {
            return false; //sino, devuelve false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_veterinaria() {
        let mut veterinaria =
            Veterinaria::new("Lo de Cacho".to_string(), "Calle 74".to_string(), 10);
        assert_eq!(veterinaria.direccion, "Calle 74".to_string());
        assert_eq!(veterinaria.nombre, "Lo de Cacho".to_string());
        assert_eq!(veterinaria.id, 10);
        assert_eq!(veterinaria.cola_atencion.len(), 0);
        assert_eq!(veterinaria.atenciones.len(), 0);
    }

    #[test]
    fn test_agregar_atras() {
        let mut veterinaria =
            Veterinaria::new("Lo de Cacho".to_string(), "Calle 74".to_string(), 10);
        let duenio = Duenio::new(
            "Tomas".to_string(),
            "Calle 73".to_string(),
            "123456789".to_string(),
        );
        let mascota = Mascota {
            nombre: "Helena".to_string(),
            edad: 5,
            tipo: TipoAnimal::PERRO,
            duenio,
        };
        veterinaria.agregar_atras(mascota);
        assert_eq!(veterinaria.cola_atencion.len(), 1);
    }

    #[test]
    fn test_agregar_primero() {
        let mut veterinaria =
            Veterinaria::new("Lo de Cacho".to_string(), "Calle 74".to_string(), 10);
        let duenio = Duenio::new(
            "Tomas".to_string(),
            "Calle 73".to_string(),
            "123456789".to_string(),
        );
        let mascota = Mascota {
            nombre: "Helena".to_string(),
            edad: 5,
            tipo: TipoAnimal::PERRO,
            duenio,
        };
        veterinaria.agregar_primero(mascota);
        assert_eq!(veterinaria.cola_atencion.len(), 1);
    }

    #[test]
    fn test_atender_primero() {
        let mut veterinaria =
            Veterinaria::new("Lo de Cacho".to_string(), "Calle 74".to_string(), 10);
        let duenio = Duenio::new(
            "Tomas".to_string(),
            "Calle 73".to_string(),
            "123456789".to_string(),
        );
        let mascota = Mascota {
            nombre: "Helena".to_string(),
            edad: 5,
            tipo: TipoAnimal::PERRO,
            duenio,
        };
        veterinaria.agregar_primero(mascota);
        assert_eq!(veterinaria.cola_atencion.len(), 1);

        let atendida = veterinaria.atender_primero();
        assert_eq!(veterinaria.cola_atencion.len(), 0);

        let atencion = Atencion::new(
            atendida.unwrap(),
            "Un Diagnostico".to_string(),
            "Un Tratamiento".to_string(),
            Fecha::new(10, 10, 2023),
        );
        veterinaria.registrar_atencion(atencion);
        assert_eq!(veterinaria.atenciones.len(), 1);
        assert_eq!(
            veterinaria.atenciones[0].mascota.nombre,
            "Helena".to_string()
        );
    }

    #[test]
    fn test_eliminar_de_la_cola() {
        let mut veterinaria =
            Veterinaria::new("Lo de Cacho".to_string(), "Calle 74".to_string(), 10);
        let duenio = Duenio::new(
            "Tomas".to_string(),
            "Calle 73".to_string(),
            "123456789".to_string(),
        );
        let mascota = Mascota {
            nombre: "Helena".to_string(),
            edad: 5,
            tipo: TipoAnimal::PERRO,
            duenio,
        };
        veterinaria.agregar_primero(mascota);
        assert_eq!(veterinaria.cola_atencion.len(), 1);
        assert_eq!(
            veterinaria.eliminar_de_la_cola(Mascota {
                nombre: "Helena".to_string(),
                edad: 5,
                tipo: TipoAnimal::PERRO,
                duenio: Duenio::new(
                    "Tomas".to_string(),
                    "Calle 73".to_string(),
                    "123456789".to_string(),
                ),
            }),
            true
        );
        assert_eq!(veterinaria.cola_atencion.len(), 0);
        assert_eq!(
            veterinaria.eliminar_de_la_cola(Mascota {
                nombre: "Helena".to_string(),
                edad: 5,
                tipo: TipoAnimal::PERRO,
                duenio: Duenio::new(
                    "Tomas".to_string(),
                    "Calle 73".to_string(),
                    "123456789".to_string(),
                ),
            }),
            false
        );
    }

    #[test]
    fn test_registrar_atencion() {
        let mut veterinaria =
            Veterinaria::new("Lo de Cacho".to_string(), "Calle 74".to_string(), 10);
        let duenio = Duenio::new(
            "Tomas".to_string(),
            "Calle 73".to_string(),
            "123456789".to_string(),
        );
        let mascota = Mascota {
            nombre: "Helena".to_string(),
            edad: 5,
            tipo: TipoAnimal::PERRO,
            duenio,
        };
        veterinaria.agregar_primero(mascota);
        assert_eq!(veterinaria.cola_atencion.len(), 1);

        let atendida = veterinaria.atender_primero();
        assert_eq!(veterinaria.cola_atencion.len(), 0);

        let atencion = Atencion::new(
            atendida.unwrap(),
            "Un Diagnostico".to_string(),
            "Un Tratamiento".to_string(),
            Fecha::new(10, 10, 2023),
        );
        veterinaria.registrar_atencion(atencion);
        assert_eq!(veterinaria.atenciones.len(), 1);
    }

    #[test]

    fn test_buscar_atencion() {
        let mut veterinaria =
            Veterinaria::new("Lo de Cacho".to_string(), "Calle 74".to_string(), 10);
        let duenio = Duenio::new(
            "Tomas".to_string(),
            "Calle 73".to_string(),
            "123456789".to_string(),
        );
        let mascota = Mascota {
            nombre: "Helena".to_string(),
            edad: 5,
            tipo: TipoAnimal::PERRO,
            duenio,
        };
        veterinaria.agregar_primero(mascota);

        let atendida = veterinaria.atender_primero();

        let atencion = Atencion::new(
            atendida.unwrap(),
            "Un Diagnostico".to_string(),
            "Un Tratamiento".to_string(),
            Fecha::new(10, 10, 2023),
        );
        veterinaria.registrar_atencion(atencion);

        assert_eq!(
            veterinaria
                .buscar_atencion(
                    "Helena".to_string(),
                    "Tomas".to_string(),
                    "123456789".to_string()
                )
                .is_some(),
            true
        );

        assert_eq!(
            veterinaria
                .buscar_atencion(
                    "Elena".to_string(),
                    "Tomas".to_string(),
                    "123456789".to_string()
                )
                .is_some(),
            false
        );
    }

    #[test]
    fn test_modificar_diagnostico() {
        let mut veterinaria =
            Veterinaria::new("Lo de Cacho".to_string(), "Calle 74".to_string(), 10);
        let duenio = Duenio::new(
            "Tomas".to_string(),
            "Calle 73".to_string(),
            "123456789".to_string(),
        );
        let mascota = Mascota {
            nombre: "Helena".to_string(),
            edad: 5,
            tipo: TipoAnimal::PERRO,
            duenio,
        };
        veterinaria.agregar_primero(mascota);

        let atendida = veterinaria.atender_primero();

        let atencion = Atencion::new(
            atendida.unwrap(),
            "Un Diagnostico".to_string(),
            "Un Tratamiento".to_string(),
            Fecha::new(10, 10, 2023),
        );
        veterinaria.registrar_atencion(atencion);

        veterinaria.modificar_diagnostico(
            "Helena".to_string(),
            "Tomas".to_string(),
            "123456789".to_string(),
            "Un nuevo diagnostico".to_string(),
        );

        assert_eq!(
            veterinaria
                .buscar_atencion(
                    "Helena".to_string(),
                    "Tomas".to_string(),
                    "123456789".to_string()
                )
                .unwrap()
                .diagnostico,
            "Un nuevo diagnostico".to_string()
        );

        veterinaria.modificar_diagnostico(
            "Elena".to_string(),
            "Tomas".to_string(),
            "123456789".to_string(),
            "Un nuevo diagnostico".to_string(),
        );

        assert_eq!(
            veterinaria
                .buscar_atencion(
                    "Helena".to_string(),
                    "Tomas".to_string(),
                    "123456789".to_string()
                )
                .unwrap()
                .diagnostico,
            "Un nuevo diagnostico".to_string() // No se modifica porque no existe la atencion
        );
    }

    #[test]
    fn test_modificar_fecha() {
        let mut veterinaria =
            Veterinaria::new("Lo de Cacho".to_string(), "Calle 74".to_string(), 10);
        let duenio = Duenio::new(
            "Tomas".to_string(),
            "Calle 73".to_string(),
            "123456789".to_string(),
        );
        let mascota = Mascota {
            nombre: "Helena".to_string(),
            edad: 5,
            tipo: TipoAnimal::PERRO,
            duenio,
        };
        veterinaria.agregar_primero(mascota);

        let atendida = veterinaria.atender_primero();

        let atencion = Atencion::new(
            atendida.unwrap(),
            "Un Diagnostico".to_string(),
            "Un Tratamiento".to_string(),
            Fecha::new(10, 10, 2023),
        );
        veterinaria.registrar_atencion(atencion);

        veterinaria.modificar_la_fecha(
            "Helena".to_string(),
            "Tomas".to_string(),
            "123456789".to_string(),
            Fecha::new(20, 10, 2023),
        );

        assert_eq!(veterinaria.atenciones[0].fecha_prox_visita.dia, 20);

        veterinaria.modificar_la_fecha(
            "Elena".to_string(),
            "Tomas".to_string(),
            "123456789".to_string(),
            Fecha::new(10, 10, 2023),
        );

        assert_eq!(veterinaria.atenciones[0].fecha_prox_visita.dia, 20); // No se modifica porque no existe la atencion
    }

    #[test]
    fn test_eliminar_atencion() {
        let mut veterinaria =
            Veterinaria::new("Lo de Cacho".to_string(), "Calle 74".to_string(), 10);
        let duenio = Duenio::new(
            "Tomas".to_string(),
            "Calle 73".to_string(),
            "123456789".to_string(),
        );
        let mascota = Mascota {
            nombre: "Helena".to_string(),
            edad: 5,
            tipo: TipoAnimal::PERRO,
            duenio,
        };
        veterinaria.agregar_primero(mascota);

        let atendida = veterinaria.atender_primero();

        let atencion = Atencion::new(
            atendida.unwrap(),
            "Un Diagnostico".to_string(),
            "Un Tratamiento".to_string(),
            Fecha::new(10, 10, 2023),
        );
        veterinaria.registrar_atencion(atencion);
        assert_eq!(veterinaria.atenciones.len(), 1);

        veterinaria.eliminar_atencion(
            "Helena".to_string(),
            Duenio::new(
                "Tomas".to_string(),
                "Calle 73".to_string(),
                "123456789".to_string(),
            ),
        );
        assert_eq!(veterinaria.atenciones.len(), 0);

        assert_eq!(
            veterinaria.eliminar_atencion(
                "Helena".to_string(),
                Duenio::new(
                    "Helena".to_string(),
                    "Calle 73".to_string(),
                    "123456789".to_string(),
                ),
            ),
            false
        );
    }

    //TEST NUEVOS

    #[test]
    fn test_atender_cola_vacia() {
        let mut veterinaria = Veterinaria::new("Cachito".to_string(), "Calle 73".to_string(), 11);
        assert!(veterinaria.atender_primero().is_none()); // Intenta atender cuando la cola está vacía, devuelve None
    }

    #[test]
    fn test_eliminar_mascota_inexistente() {
        let mut veterinaria = Veterinaria::new("Cachito".to_string(), "Calle 73".to_string(), 12);
        let duenio = Duenio::new(
            "Tomas".to_string(),
            "Calle 73".to_string(),
            "321".to_string(),
        );
        let mascota = Mascota {
            nombre: "Helena".to_string(),
            edad: 3,
            tipo: TipoAnimal::GATO,
            duenio,
        };
        assert!(!veterinaria.eliminar_de_la_cola(mascota)); // Intenta eliminar una mascota que no está en la cola, devuelve false
    }

    #[test]
    fn test_buscar_atencion_inexistente() {
        let mut veterinaria = Veterinaria::new("Cachito".to_string(), "Calle 73".to_string(), 14);
        assert!(
            veterinaria
                .buscar_atencion(
                    // Busca una atención que no existe
                    "Helena".to_string(),
                    "Tomas".to_string(),
                    "123".to_string()
                )
                .is_none() // Devuelve None
        );
    }

    #[test]
    fn test_modificar_diagnostico_inexistente() {
        let mut veterinaria = Veterinaria::new("Cachito".to_string(), "Calle 73".to_string(), 15);
        veterinaria.modificar_diagnostico(
            "NoExiste".to_string(),
            "Juan".to_string(),
            "321".to_string(),
            "Nuevo Diagnóstico".to_string(), // Intenta modificar el diagnóstico de una atención inexistente
        );
        assert!(veterinaria.atenciones.is_empty());
    }

    #[test]
    fn test_eliminar_atencion_inexistente() {
        let mut veterinaria = Veterinaria::new("Cachito".to_string(), "Calle 73".to_string(), 15);
        let duenio = Duenio::new(
            "Pedro".to_string(),
            "Calle 81".to_string(),
            "312".to_string(),
        );
        assert!(!veterinaria.eliminar_atencion("NoExiste".to_string(), duenio)); // Intenta eliminar una atención que no existe, devuelve false
    }
}
