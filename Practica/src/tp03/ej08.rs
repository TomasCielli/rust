#[derive(Clone)]
enum Genero {
    ROCK,
    POP,
    RAP,
    JAZZ,
    OTROS,
}

impl Genero {
    fn devolver_genero(&self) -> String {
        match self {
            Genero::ROCK => return "Rock".to_string(),
            Genero::POP => return "Pop".to_string(),
            Genero::RAP => return "Rap".to_string(),
            Genero::JAZZ => return "Jazz".to_string(),
            Genero::OTROS => return "Otros".to_string(),
        }
    }
}

#[derive(Clone)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}
impl Cancion {
    fn new(titulo: String, artista: String, genero: Genero) -> Cancion {
        Cancion {
            titulo,
            artista,
            genero,
        }
    }
}

#[derive(Clone)]
struct Playlist {
    nombre: String,
    canciones: Vec<Cancion>,
}

impl Playlist {
    fn new(nombre: String) -> Playlist {
        Playlist {
            nombre,
            canciones: Vec::new(), // <--- Empieza vacia
        }
    }

    fn agregar_cancion(&mut self, cancion: Cancion) -> bool {
        if !self.canciones.iter().any(|x| {
            // Busco si ya existe la cancion
            cancion.titulo == x.titulo
                && cancion.artista == x.artista
                && cancion.genero.devolver_genero() == x.genero.devolver_genero()
        }) {
            // Si no existe la agrego
            self.canciones.push(cancion);
            return true;
        }
        return false;
    }

    fn eliminar_cancion(&mut self, cancion: Cancion) -> bool {
        if let Some(pos) = self.canciones.iter().position(|x| {
            cancion.titulo == x.titulo
                && cancion.artista == x.artista
                && cancion.genero.devolver_genero() == x.genero.devolver_genero()
        }) {
            self.canciones.remove(pos);
            return true;
        }
        return false;
    }

    fn mover_cancion(&mut self, cancion: Cancion, posicion: usize) -> bool {
        if posicion < self.canciones.len() {
            //Si la posicion es valida
            if let Some(pos) = self.canciones.iter().position(|x| {
                // Busco la cancion
                cancion.titulo == x.titulo
                    && cancion.artista == x.artista
                    && cancion.genero.devolver_genero() == x.genero.devolver_genero()
            }) {
                // Si la encuentro
                let cancion_mover = self.canciones.remove(pos); // La saco de la lista

                self.canciones.insert(posicion, cancion_mover); // La inserto en la nueva posicion
                return true;
            }
        }
        return false; // Si no la encuentro o la posicion no es valida
    }

    fn buscar_cancion(&self, nombre: String) -> Option<&Cancion> {
        self.canciones.iter().find(|x| x.titulo == nombre)
    }

    fn obtener_canciones_genero(&self, genero: Genero) -> Vec<&Cancion> {
        self.canciones
            .iter()
            .filter(|x| x.genero.devolver_genero() == genero.devolver_genero())
            .collect() // Armo una coleccion de las canciones que tengan el mismo genero
        // El collect infiere el tipo ¿?
    }

    fn obtener_canciones_artista(&self, artista: String) -> Vec<&Cancion> {
        self.canciones
            .iter()
            .filter(|x| x.artista == artista)
            .collect()
    }

    fn modificar_titulo(&mut self, titulo: String) {
        self.nombre = titulo;
    }

    fn eliminar_todas(&mut self) -> bool {
        if self.canciones.is_empty() {
            return false; // Si ya estaba vacia
        }
        self.canciones.clear(); // El clear borra todos los elementos en el vector
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playlist() {
        let mut playlist = Playlist::new("Mi Playlist".to_string());

        let cancion1 = Cancion::new(
            "Cancion 1".to_string(),
            "Artista 1".to_string(),
            Genero::ROCK,
        );
        let cancion2 = Cancion::new(
            "Cancion 2".to_string(),
            "Artista 2".to_string(),
            Genero::POP,
        );
        let cancion3 = Cancion::new(
            "Cancion 3".to_string(),
            "Artista 1".to_string(),
            Genero::RAP,
        );

        //Agrego canciones
        playlist.agregar_cancion(cancion1);
        playlist.agregar_cancion(cancion2);
        playlist.agregar_cancion(cancion3);

        assert_eq!(playlist.canciones.len(), 3);

        // Elimino una cancion
        playlist.eliminar_cancion(Cancion::new(
            "Cancion 2".to_string(),
            "Artista 2".to_string(),
            Genero::POP,
        ));
        assert_eq!(playlist.canciones.len(), 2);

        // Muevo una cancion a la posicion 0
        playlist.mover_cancion(
            Cancion::new(
                "Cancion 3".to_string(),
                "Artista 1".to_string(),
                Genero::RAP,
            ),
            0,
        );
        assert_eq!(playlist.canciones[0].titulo, "Cancion 3");

        // Busco una cancion por nombre
        let cancion_encontrada = playlist.buscar_cancion("Cancion 1".to_string());
        assert!(cancion_encontrada.is_some());

        // Busco una cancion que no existe
        let cancion_no_encontrada = playlist.buscar_cancion("Cancion 4".to_string());
        assert!(cancion_no_encontrada.is_none());

        // Busco las canciones de un genero
        let canciones_rock = playlist.obtener_canciones_genero(Genero::ROCK);
        assert_eq!(canciones_rock.len(), 1);

        // Busco las canciones de un artista
        let canciones_artista1 = playlist.obtener_canciones_artista("Artista 1".to_string());
        assert_eq!(canciones_artista1.len(), 2);

        // Modifico el titulo de la playlist
        playlist.modificar_titulo("Messi te amo".to_string());
        assert_eq!(playlist.nombre, "Messi te amo");

        // Elimino todas las canciones de la playlist
        playlist.eliminar_todas();
        assert_eq!(playlist.canciones.len(), 0);
    }

    //TEST NUEVOS

    #[test]
    fn test_playlist_vacia() {
        let playlist = Playlist::new("Vacia".to_string());
        assert!(playlist.canciones.is_empty());
        assert!(playlist.buscar_cancion("Nada".to_string()).is_none());
        assert!(playlist.obtener_canciones_genero(Genero::ROCK).is_empty());
        assert!(
            playlist
                .obtener_canciones_artista("Tomas".to_string())
                .is_empty()
        );
    }

    #[test]
    fn test_agregar_y_eliminar_cancion() {
        let mut playlist = Playlist::new("Rock Nacional".to_string());
        let c1 = Cancion::new("Cancion".to_string(), "Tomas".to_string(), Genero::ROCK);
        assert_eq!(playlist.canciones.len(), 0);
        playlist.agregar_cancion(c1.clone());
        assert_eq!(playlist.canciones.len(), 1);
        assert!(playlist.eliminar_cancion(c1.clone()));
        assert_eq!(playlist.canciones.len(), 0);
        // Eliminar una canción que no existe
        assert!(!playlist.eliminar_cancion(c1));
    }

    #[test]
    fn test_mover_canciones() {
        let mut playlist = Playlist::new("Rock Nacional".to_string());
        let c1 = Cancion::new("Cancion 1".to_string(), "Tomas".to_string(), Genero::ROCK);
        let c2 = Cancion::new("Cancion 2".to_string(), "Tomas".to_string(), Genero::ROCK);
        let c3 = Cancion::new("Cancion 3".to_string(), "Tomas".to_string(), Genero::ROCK);
        playlist.agregar_cancion(c1.clone());
        playlist.agregar_cancion(c2.clone());
        playlist.agregar_cancion(c3.clone());
        assert!(playlist.mover_cancion(c2.clone(), 0));
        assert_eq!(playlist.canciones[0].titulo, "Cancion 2");
        assert_eq!(playlist.canciones[1].titulo, "Cancion 1");
        assert_eq!(playlist.canciones[2].titulo, "Cancion 3");
        // Mover una canción que no existe
        assert!(!playlist.mover_cancion(
            Cancion::new("Cancion 4".to_string(), "Tomas".to_string(), Genero::ROCK),
            0
        ));
        // Mover a una posición inválida
        assert!(!playlist.mover_cancion(c1.clone(), 5));
    }
}
