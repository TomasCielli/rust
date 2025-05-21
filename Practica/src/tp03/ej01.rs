struct Persona {
    nombre: String,
    edad: u8,
    direccion: Option<String>,
}

impl Persona {
    //"CONSTRUCTOR"

    fn new(nombre: String, edad: u8, direccion: Option<String>) -> Persona {
        let persona = Persona {
            nombre,
            edad,
            direccion,
        };
        persona
    }

    //METODOS

    fn to_string(&self) -> String {
        let direccion = match &self.direccion {
            Some(direccion) => direccion.clone(),
            None => "Sin direccion".to_string(),
        };
        format!(
            "Nombre: {} Edad: {} Direccion: {}",
            self.nombre, self.edad, direccion
        )
    }

    fn obtener_edad(&self) -> u8 {
        self.edad
    }

    fn actualizar_direccion(&mut self, nueva_direccion: Option<String>) {
        self.direccion = nueva_direccion;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persona_con_direccion() {
        let mut persona = Persona::new("Tomas".to_string(), 24, Some("Calle 73".to_string()));

        assert_eq!(
            persona.to_string(),
            "Nombre: Tomas Edad: 24 Direccion: Calle 73"
        );

        assert_eq!(persona.obtener_edad(), 24);

        persona.actualizar_direccion(Some("Calle 74".to_string()));
        assert_eq!(
            persona.to_string(),
            "Nombre: Tomas Edad: 24 Direccion: Calle 74"
        );
    }

    #[test]
    fn test_persona_sin_direccion() {
        let mut persona_sin_direccion = Persona::new("Juan".to_string(), 23, None);

        assert_eq!(
            persona_sin_direccion.to_string(),
            "Nombre: Juan Edad: 23 Direccion: Sin direccion"
        );

        assert_eq!(persona_sin_direccion.obtener_edad(), 23);

        persona_sin_direccion.actualizar_direccion(Some("Calle 75".to_string()));
        assert_eq!(
            persona_sin_direccion.to_string(),
            "Nombre: Juan Edad: 23 Direccion: Calle 75"
        );
    }
}
