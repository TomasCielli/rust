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

//FALTAN PUNTO G!!! Y TESTS