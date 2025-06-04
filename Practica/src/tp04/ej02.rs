/*

2- Dado el siguiente struct: 
 
struct Persona<'a>{ 
    nombre:&'a str, 
    apellido:&'a str, 
    direccion:&'a str, 
    ciudad:&'a str, 
    salario:f64, 
    edad:u8,  
} 
 
a- Escriba una función que reciba un vector de personas y otro parámetro que indica un 
salario y retorna un listado de personas donde el salario es mayor al parámetro recibido. 
 
b- Escriba una función que reciba un vector de personas, edad y el nombre de una  ciudad, 
y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro 
ciudad. 
 
c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y 
retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso 
contrario. 
 
d- Escriba una función que reciba un vector de personas y un nombre de una ciudad y 
retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso 
contrario.

e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la 
persona existe en el arreglo, false caso contrario 
 
f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las 
edades de las personas. 
 
g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor 
salario y la persona con el mayor salario, en caso de que haya más de una persona en cada 
categoría desempatar por la edad más grande. 
 
Nota: Implemente todos los métodos y traits que considere para resolver los ejercicios. 
Todos los ejercicios deben resolverse con iterator y closure

*/

#[derive(Clone)]

struct Persona<'a>{ 
    nombre:&'a str, 
    apellido:&'a str, 
    direccion:&'a str, 
    ciudad:&'a str, 
    salario:f64, 
    edad:u8,  
}
impl <'a> Persona <'a>{
    
    fn get_nombre (&self) -> String {
        self.nombre.to_string()
    }
    
    fn get_apellido(&self) -> String {
        self.apellido.to_string()
    } 

    fn get_direccion(&self) -> String {
        self.direccion.to_string()
    } 

    fn get_ciudad(&self) -> String {
        self.ciudad.to_string()
    } 

    fn get_salario(&self) -> f64 {
        self.salario
    }

    fn get_edad(&self) -> u8 {
        self.edad
    }

    fn igual(&self, otro: Persona) -> bool {
        self.get_nombre() == otro.get_nombre() &&
        self.get_apellido() == otro.get_apellido() &&
        self.get_direccion() == otro.get_direccion() &&
        self.get_ciudad() == otro.get_ciudad() &&
        self.get_salario() == otro.get_salario() &&
        self.get_edad() == otro.get_edad() 
    }


}

/* a- Escriba una función que reciba un vector de personas y otro parámetro que indica un 
salario y retorna un listado de personas donde el salario es mayor al parámetro recibido.  */

fn salario_mayor<'a> (personas: &'a Vec<Persona<'a>>, salario: f64) -> Vec<&'a Persona<'a>> {
    personas
        .iter()
        .filter(|x| x.get_salario() > salario)
        .collect()
}

/* b- Escriba una función que reciba un vector de personas, edad y el nombre de una  ciudad, 
y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro 
ciudad.  */

fn personas_mayores <'a> (personas: &'a Vec<Persona<'a>>, edad: u8, ciudad: String) -> Vec<&'a Persona<'a>> {
    personas
        .iter()
        .filter(|x| x.get_edad() == edad && x.get_ciudad() == ciudad)
        .collect()
}


/* c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y 
retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso 
contrario. */

fn todos_viven_en <'a>(personas: &'a Vec<Persona<'a>>, ciudad: String) -> bool {
    personas
        .iter()
        .all(|x| x.get_ciudad() == ciudad)
}

/* d- Escriba una función que reciba un vector de personas y un nombre de una ciudad y 
retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso 
contrario. */

fn alguno_vive_en <'a>(personas: &'a Vec<Persona<'a>>, ciudad: String) -> bool {
    personas
        .iter()
        .any(|x| x.get_ciudad() == ciudad)
} 

/* e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la 
persona existe en el arreglo, false caso contrario  */

fn esta_la_persona <'a>(personas: &'a Vec<Persona<'a>>, persona: &Persona) -> bool {
    personas.iter().any(|x| x.igual(persona.clone()))
}

/* f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las 
edades de las personas. */

fn contar_edades <'a>(personas: &'a Vec<Persona<'a>>) -> [u8; 130] {

    let mut edades: [u8; 130] = [0; 130];
    personas.iter().for_each(|x| {
        let i: usize = x.get_edad().into();
        edades[i] += 1;
    });

    edades
}

/* g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor 
salario y la persona con el mayor salario, en caso de que haya más de una persona en cada 
categoría desempatar por la edad más grande. */

fn min_max_salario<'a>(personas: &'a Vec<Persona<'a>>) -> (Option<&'a Persona<'a>>, Option<&'a Persona<'a>>) { 

    if personas.is_empty() {
        return (None, None);
    }

    let mut min = &personas[0];
    let mut max = &personas[0];  

    personas.iter().for_each(|x| {
        if x.get_salario() < min.get_salario() || 
           (x.get_salario() == min.get_salario() && x.get_edad() > min.get_edad()) {
            min = x;
        }
        if x.get_salario() > max.get_salario() || 
           (x.get_salario() == max.get_salario() && x.get_edad() > max.get_edad()) {
            max = x;
        }
    });

    (Some(min), Some(max))
}

//Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_personas<'a>() -> Vec<Persona<'a>> {
        vec![
            Persona {
                nombre: "Ana",
                apellido: "Gomez",
                direccion: "Calle 1",
                ciudad: "La Plata",
                salario: 80000.0,
                edad: 30,
            },
            Persona {
                nombre: "Luis",
                apellido: "Martinez",
                direccion: "Calle 2",
                ciudad: "La Plata",
                salario: 120000.0,
                edad: 40,
            },
            Persona {
                nombre: "Maria",
                apellido: "Perez",
                direccion: "Calle 3",
                ciudad: "Cordoba",
                salario: 90000.0,
                edad: 35,
            },
            Persona {
                nombre: "Juan",
                apellido: "Lopez",
                direccion: "Calle 4",
                ciudad: "Cordoba",
                salario: 90000.0,
                edad: 50,
            },
        ]
    }

    #[test]
    fn test_salario_mayor() {
        let personas = crear_personas();
        let result = salario_mayor(&personas, 85000.0);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_personas_mayores() {
        let personas = crear_personas();
        let result = personas_mayores(&personas, 35, "Cordoba".to_string());
        assert_eq!(result.len(), 1); // Solo Juan (50) cumple ambas condiciones
    }

    #[test]
    fn test_todos_viven_en_true() {
        let personas = vec![
            Persona {
                nombre: "Ana",
                apellido: "Gomez",
                direccion: "Calle 1",
                ciudad: "La Plata",
                salario: 50000.0,
                edad: 28,
            },
            Persona {
                nombre: "Carlos",
                apellido: "Diaz",
                direccion: "Calle 2",
                ciudad: "La Plata",
                salario: 55000.0,
                edad: 32,
            },
        ];
        assert!(todos_viven_en(&personas, "La Plata".to_string()));
    }

    #[test]
    fn test_todos_viven_en_false() {
        let personas = crear_personas();
        assert!(!todos_viven_en(&personas, "La Plata".to_string()));
    }

    #[test]
    fn test_alguno_vive_en_true() {
        let personas = crear_personas();
        assert!(alguno_vive_en(&personas, "Cordoba".to_string()));
    }

    #[test]
    fn test_alguno_vive_en_false() {
        let personas = crear_personas();
        assert!(!alguno_vive_en(&personas, "Rosario".to_string()));
    }

    #[test]
    fn test_esta_la_persona_true() {
        let personas = crear_personas();
        let persona = personas[0].clone();
        assert!(esta_la_persona(&personas, &persona));
    }

    #[test]
    fn test_esta_la_persona_false() {
        let personas = crear_personas();
        let persona = Persona {
            nombre: "Otro",
            apellido: "Otro",
            direccion: "Otra calle",
            ciudad: "Otro lugar",
            salario: 100.0,
            edad: 1,
        };
        assert!(!esta_la_persona(&personas, &persona));
    }

    #[test]
    fn test_contar_edades() {
        let personas = crear_personas();
        let edades = contar_edades(&personas);
        assert_eq!(edades[30], 1);
        assert_eq!(edades[40], 1);
        assert_eq!(edades[35], 1);
        assert_eq!(edades[50], 1);
    }

    #[test]
    fn test_min_max_salario() {
        let personas = mock_personas();
        let (min, max) = min_max_salario(&personas);
        assert_eq!(min.unwrap().nombre, "Ana"); // 80000.0
        assert_eq!(max.unwrap().nombre, "Luis"); // 120000.0
    }

    #[test]
    fn test_min_max_empate_por_edad() {
        let personas = vec![
            Persona {
                nombre: "P1",
                apellido: "A",
                direccion: "D1",
                ciudad: "X",
                salario: 100.0,
                edad: 40,
            },
            Persona {
                nombre: "P2",
                apellido: "B",
                direccion: "D2",
                ciudad: "X",
                salario: 100.0,
                edad: 45,
            },
        ];
        let (min, max) = min_max_salario(&personas);
        assert_eq!(min.unwrap().nombre, "P2");
        assert_eq!(max.unwrap().nombre, "P2");
    }

    #[test]
    fn test_min_max_salario_lista_vacia() {
        let personas: Vec<Persona> = vec![];
        let (min, max) = min_max_salario(&personas);
        assert!(min.is_none());
        assert!(max.is_none());
    }
}

