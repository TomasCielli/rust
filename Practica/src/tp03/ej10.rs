use crate::tp03::ej03::Fecha;

//ENUMS

#[derive(Clone)]
enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}
impl Genero {
    fn mostrar(&self) -> String {
        match self {
            Genero::Novela => "Novela".to_string(),
            Genero::Infantil => "Infantil".to_string(),
            Genero::Tecnico => "Tecnico".to_string(),
            Genero::Otros => "Otros".to_string(),
        }
    }
}

#[derive(Clone)]
enum Estado {
    Devuelto,
    EnPrestamo,
}
impl Estado {
    fn mostrar(&self) -> String {
        match self {
            Estado::Devuelto => "Devuelto".to_string(),
            Estado::EnPrestamo => "En Prestamo".to_string(),
        }
    }
}

//STRUCTS

#[derive(Clone)]
struct Cliente {
    nombre: String,
    telefono: String,
    mail: String,
}
impl Cliente {
    fn new(nombre: String, telefono: String, mail: String) -> Cliente {
        Cliente {
            nombre,
            telefono,
            mail,
        }
    }

    fn iguales(&self, cliente: &Cliente) -> bool {
        self.nombre == cliente.nombre && self.telefono == cliente.telefono
    }
}

#[derive(Clone)]
struct Libro {
    isbn: u32,
    titulo: String,
    autor: String,
    paginas: u16,
    genero: Genero,
}
impl Libro {
    fn new(isbn: u32, titulo: String, autor: String, paginas: u16, genero: Genero) -> Libro {
        Libro {
            isbn,
            titulo,
            autor,
            paginas,
            genero,
        }
    }
}

#[derive(Clone)]
struct Libros(u32, Libro); //Tuple Struct de copias y libro
impl Libros {
    fn new(copias: u32, libro: Libro) -> Libros {
        Libros(copias, libro)
    }
}

#[derive(Clone)]
struct Prestamo {
    libro: u32, //Clave unica
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: Estado,
}
impl Prestamo {
    fn new(libro: u32, cliente: Cliente, fecha_vencimiento: Fecha, estado: Estado) -> Prestamo {
        Prestamo {
            libro,
            cliente,
            fecha_vencimiento,
            fecha_devolucion: None,
            estado,
        }
    }
}

struct Biblioteca {
    nombre: String,
    direccion: String,
    libros: Vec<Libros>, //Isbn como clave //SE PODIA USAR UN HASHMAP
    prestamos: Vec<Prestamo>,
}
impl Biblioteca {
    fn new(nombre: String, direccion: String) -> Biblioteca {
        Biblioteca {
            nombre,
            direccion,
            libros: Vec::new(), //Empiezan vacios
            prestamos: Vec::new(),
        }
    }
    fn cargar_libro(&mut self, libro: Libros) -> bool {
        if self.libros.iter().any(|x| x.1.isbn == libro.1.isbn) {
            //Si existe algun libro con la misma clave
            return false; //No lo carga, y devuelve false
        } else {
            self.libros.push(libro); //Si no existe, lo carga y devuelve true
            return true;
        }
    }

    fn buscar_libro(&mut self, isbn: u32) -> Option<usize> {
        //Devuelve la posicion (si existe) del libro en el vector
        if let Some(pos) = self.libros.iter().position(|x| x.1.isbn == isbn) {
            //Compara claves
            return Some(pos);
        } else {
            return None;
        }
    }

    fn cant_copias(&mut self, isbn: u32) -> Option<u32> {
        if let Some(pos) = self.buscar_libro(isbn) {
            return Some(self.libros[pos].0); //Se fija en el primer campo del tuple struct, que tiene la cantidad de copias
        } else {
            return None;
        }
    }

    fn dec_copias(&mut self, isbn: u32) -> bool {
        if let Some(pos) = self.buscar_libro(isbn) {
            //Busca el libro
            if self.libros[pos].0 > 0 {
                //Si quedan copias para prestar
                self.libros[pos].0 -= 1; //Decrementa la cantidad de copias y devuelve true
                return true;
            }
        }
        return false; //Si no se cumple alguna de las condiciones, devuelve false
    }

    fn inc_copias(&mut self, isbn: u32) -> bool {
        //Lo mismo que el anterior, pero incrementando las copias (y sin revisar la cant. copias)
        if let Some(pos) = self.buscar_libro(isbn) {
            self.libros[pos].0 += 1;
            return true;
        }
        return false;
    }

    fn contar_prestamos(&self, cliente: Cliente) -> u8 {
        let mut total = 0; //Empieza el contador en 0
        for prestamo in &self.prestamos {
            //Recorre los prestamos
            if prestamo.cliente.iguales(&cliente) //Si el cliente (del prestamo) es igual al buscado
                && prestamo.estado.mostrar() == "En Prestamo"
            //Y el estado es "En Prestamo"
            {
                total += 1; //Suma al contador
            }
        }
        return total;
    }

    fn realizar_prestamo(&mut self, isbn: u32, cliente: Cliente, fecha_vencimiento: Fecha) -> bool {
        if self.contar_prestamos(cliente.clone()) < 5 {
            //Si el cliente tiene menos de 5 prestamos
            if let Some(pos) = self.buscar_libro(isbn) {
                //Busca el libro
                if self.libros[pos].0 > 0 {
                    //Si quedan copias para prestar
                    let prestamo =
                        Prestamo::new(isbn, cliente, fecha_vencimiento, Estado::EnPrestamo); //Crea el prestamo
                    self.prestamos.push(prestamo); //Lo carga al vector de prestamos
                    self.dec_copias(isbn); //Decrementa la cantidad de copias
                    return true;
                }
            }
        }
        return false;
    }

    fn prestamos_por_vencer(&self, dias: u8, hoy: Fecha) -> Option<Vec<Prestamo>> {
        let mut listado = Vec::new();
        for prestamo in &self.prestamos {
            let mut fecha = prestamo.fecha_vencimiento.clone();
            let mut fecha_hoy = hoy.clone();
            if fecha_hoy.sumar_dias(dias) {
                //Si la fecha de hoy + los dias es una fecha valida
                if fecha.es_mayor_2(hoy.clone())
                    && !fecha.es_mayor_2(fecha_hoy.clone())
                    && prestamo.estado.mostrar() == "En Prestamo".to_string()
                {
                    //y si esta entre la fecha de hoy y la de hoy+dias, y el estado es "En Prestamo"
                    listado.push(prestamo.clone()); //Lo carga al listado
                }
            }
        }
        if !listado.is_empty() {
            //Si el listado no esta vacio, lo devuelve
            return Some(listado);
        }
        return None;
    }

    fn prestamos_vencidos(&self, hoy: Fecha) -> Option<Vec<Prestamo>> {
        let mut listado = Vec::new();
        for prestamo in &self.prestamos {
            if hoy.es_mayor_2(prestamo.clone().fecha_vencimiento) //Si la fecha de vencimiento es < a la de hoy (se vencio el prestamo)
                && prestamo.estado.mostrar() == "En Prestamo".to_string()
            //y el prestamo esta en estado "En Prestamo"
            {
                listado.push(prestamo.clone()); //Lo carga al listado
            }
        }
        if !listado.is_empty() {
            return Some(listado); //Si el listado no esta vacio, lo devuelve
        }
        return None;
    }

    fn buscar_prestamo(&self, isbn: u32, cliente: Cliente) -> Option<&Prestamo> {
        if let Some(prestamo) = self
            .prestamos
            .iter()
            .find(|x| x.libro == isbn && x.cliente.iguales(&cliente))
        //Busca por clave (isbn) y compara clientes
        {
            return Some(prestamo); //Si lo encuentra lo devuelve
        }
        return None; //Sino devuelve None
    }

    fn devolver_libro(&mut self, isbn: u32, cliente: Cliente, hoy: Fecha) -> bool {
        if let Some(prestamo) = self.prestamos.iter_mut().find(|prestamo| {
            //Si existe un prestamo con:
            prestamo.cliente.iguales(&cliente) //El mismo cliente, el mismo libro, y que tengan el estado "En Prestamo"
                && prestamo.libro == isbn
                && prestamo.estado.mostrar() == "En Prestamo".to_string()
        }) {
            prestamo.estado = Estado::Devuelto; //Lo actualiza
            prestamo.fecha_devolucion = Some(hoy);
            self.inc_copias(isbn);
            return true; //Y devuelve true
        }
        return false; //Sino devuelve false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargar_libros_repetidos() {
        let mut biblio = Biblioteca::new("Biblioteca".to_string(), "Calle 73".to_string());
        let libro = Libros::new(
            1,
            Libro::new(
                100,
                "Libro1".to_string(),
                "Autor1".to_string(),
                300,
                Genero::Tecnico,
            ),
        );
        assert_eq!(biblio.cargar_libro(libro.clone()), true); // Carga el libro
        assert_eq!(biblio.cargar_libro(libro), false); // No se puede cargar un libro con el mismo isbn
    }

    #[test]
    fn test_prestamos_maximos() {
        let mut biblio = Biblioteca::new("Biblioteca".to_string(), "Direccion".to_string());
        let libro = Libros::new(
            10,
            Libro::new(
                1,
                "Libro1".to_string(),
                "Autor1".to_string(),
                100,
                Genero::Otros,
            ),
        );
        biblio.cargar_libro(libro);
        let cliente = Cliente::new(
            "Tomas".to_string(),
            "123".to_string(),
            "messiteamo@gmail.com".to_string(),
        );
        let fecha = Fecha::new(10, 10, 2025);

        for _ in 0..5 {
            assert_eq!(
                biblio.realizar_prestamo(1, cliente.clone(), fecha.clone()),
                true
            );
        }
        // No se pueden realizar más de 5 préstamos
        assert_eq!(
            biblio.realizar_prestamo(1, cliente.clone(), fecha.clone()),
            false
        );
    }

    #[test]
    fn test_sin_copias_disponibles() {
        let mut biblio = Biblioteca::new("Biblioteca".to_string(), "Direccion".to_string());
        let libro = Libros::new(
            1,
            Libro::new(
                2,
                "Libro1".to_string(),
                "Autor1".to_string(),
                150,
                Genero::Novela,
            ),
        );
        biblio.cargar_libro(libro);

        let cliente1 = Cliente::new(
            "Tomas".to_string(),
            "123".to_string(),
            "messiteamo@gmail.com".to_string(),
        );
        let cliente2 = Cliente::new(
            "Juan".to_string(),
            "321".to_string(),
            "messiteodio@gmail.com".to_string(),
        );
        let fecha = Fecha::new(5, 5, 2025);

        assert_eq!(
            biblio.realizar_prestamo(2, cliente1.clone(), fecha.clone()),
            true
        ); // Presta el libro
        assert_eq!(
            biblio.realizar_prestamo(2, cliente2.clone(), fecha.clone()),
            false
        ); // No hay más copias
    }

    #[test]
    fn test_devolver_libro() {
        let mut biblio = Biblioteca::new("Biblioteca".to_string(), "Direccion".to_string());
        let libro = Libros::new(
            1,
            Libro::new(
                3,
                "Libro1".to_string(),
                "Autor1".to_string(),
                100,
                Genero::Infantil,
            ),
        );
        biblio.cargar_libro(libro);
        let cliente = Cliente::new(
            "Juan".to_string(),
            "321".to_string(),
            "messiteodio@gmail.com".to_string(),
        );
        let fecha_vto = Fecha::new(1, 1, 2025);
        let hoy = Fecha::new(2, 1, 2025);

        assert_eq!(
            biblio.realizar_prestamo(3, cliente.clone(), fecha_vto.clone()),
            true
        );
        assert_eq!(biblio.cant_copias(3), Some(0));
        assert_eq!(biblio.devolver_libro(3, cliente.clone(), hoy.clone()), true);
        assert_eq!(biblio.cant_copias(3), Some(1));

        // No quedan mas prestamos
        assert_eq!(biblio.devolver_libro(3, cliente.clone(), hoy), false);
    }

    #[test]
    fn test_prestamos_vencidos() {
        let mut biblio = Biblioteca::new("Biblioteca".to_string(), "Direccion".to_string());
        let libro = Libros::new(
            2,
            Libro::new(
                4,
                "Libro1".to_string(),
                "Autor1".to_string(),
                200,
                Genero::Novela,
            ),
        );
        biblio.cargar_libro(libro);
        let cliente = Cliente::new(
            "Tomas".to_string(),
            "123".to_string(),
            "messiteamo@gmail.com".to_string(),
        );
        let vencido = Fecha::new(1, 1, 2024);
        let hoy = Fecha::new(1, 1, 2025);

        biblio.realizar_prestamo(4, cliente.clone(), vencido.clone());

        let vencidos = biblio.prestamos_vencidos(hoy).unwrap();
        assert_eq!(vencidos.len(), 1);
    }

    #[test]
    fn test_prestamos_por_vencer() {
        let mut biblio = Biblioteca::new("Biblioteca".to_string(), "Direccion".to_string());
        let libro = Libros::new(
            3,
            Libro::new(
                5,
                "Libro1".to_string(),
                "Autor1".to_string(),
                120,
                Genero::Tecnico,
            ),
        );
        biblio.cargar_libro(libro);
        let cliente = Cliente::new(
            "Tomas".to_string(),
            "123".to_string(),
            "messiteamo@gmail.com".to_string(),
        );

        let vencimiento = Fecha::new(15, 5, 2025);
        let hoy = Fecha::new(13, 5, 2025);

        biblio.realizar_prestamo(5, cliente.clone(), vencimiento.clone());

        let por_vencer = biblio.prestamos_por_vencer(2, hoy.clone());
        if let Some(prestamos) = por_vencer {
            assert_eq!(prestamos.len(), 1);
            assert_eq!(prestamos[0].libro, 5);
            assert_eq!(prestamos[0].cliente.nombre, "Tomas");
        }
        let por_vencer_nada = biblio.prestamos_por_vencer(1, hoy.clone());
        assert!(por_vencer_nada.is_none());
    }

    #[test]
    fn test_buscar_prestamo() {
        let mut biblio = Biblioteca::new("Biblioteca".to_string(), "Direccion".to_string());
        let libro = Libros::new(
            1,
            Libro::new(
                6,
                "Libro1".to_string(),
                "Autor1".to_string(),
                80,
                Genero::Otros,
            ),
        );
        biblio.cargar_libro(libro);
        let cliente = Cliente::new(
            "Leo".to_string(),
            "231".to_string(),
            "leomessi@gmail.com".to_string(),
        );
        let fecha = Fecha::new(20, 6, 2025);

        biblio.realizar_prestamo(6, cliente.clone(), fecha.clone());

        let encontrado = biblio.buscar_prestamo(6, cliente.clone());
        assert!(encontrado.is_some());

        let otro = Cliente::new(
            "Tomas".to_string(),
            "123".to_string(),
            "messiteamo@gmail.com".to_string(),
        );
        assert!(biblio.buscar_prestamo(6, otro).is_none());
    }
}
