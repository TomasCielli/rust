/*

La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones 
(Basic, Clasic, Super) a sus usuarios. Cada suscripción tiene un costo mensual y una 
duración de meses y una fecha de inicio, además los usuarios pueden pagar por sus 
suscripciones con distintos medios de pago que son Efectivo, MercadoPago, Tarjeta de 
Crédito, Transferencia Bancaria, Cripto. Cada medio de pago tiene sus datos 
correspondientes a excepción de Efectivo. 
Los usuarios solo pueden tener una suscripción activa a la vez. 
 
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las 
siguientes acciones: 
 
➢  Crear un usuario con una determinada suscripción y medio de pago. 
➢  Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic 
pasa a Clasic y si está en Clasic pasa a Super. 
➢  Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la 
suscripción es del tipo Basic al hacerlo se cancelará la suscripción. 
➢  Dado un usuario cancelar la suscripción. 
➢  Saber el medio de pago que es más utilizado por los usuarios sobre las 
suscripciones activas 
➢  Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones 
activas. 
➢  Saber cuál fue el medio de pago más utilizado. 
➢  Saber cuál fue la suscripción más contratada.

*/
use crate::tp03::ej03::Fecha;

//Distintos tipos de suscripciones

#[derive(Debug, Clone, PartialEq, Eq)]
enum TipoSuscripcion {
    Basic,
    Clasic,
    Super,
}
impl TipoSuscripcion {
    fn buscar_tipo(&self) -> usize {
        match self {
            TipoSuscripcion::Basic => 1,
            TipoSuscripcion::Clasic => 2,
            TipoSuscripcion::Super => 3,
        }
    }
}

//Estructura de Suscripcion

#[derive(Debug, Clone)]
struct Suscripcion {
    tipo_suscripcion: TipoSuscripcion,
    costo: f64,
    duracion: u8,
    fecha_inicio: Fecha,
}

//Distintos medios de pago

#[derive(Debug, Clone, PartialEq, Eq)]
enum MedioDePago {
    Efectivo,
    MercadoPago{cvu: String},
    TarjetaCredito{numero: String, vencimiento: String},
    TransferenciaBancaria{cbu: String},
    Cripto{direccion: String},
}
impl MedioDePago {
    fn buscar_tipo (&self) -> usize{
        match self {
            MedioDePago::Efectivo => 1,
            MedioDePago::MercadoPago { cvu } => 2,
            MedioDePago::TarjetaCredito { numero, vencimiento } => 3,
            MedioDePago::TransferenciaBancaria { cbu } => 4,
            MedioDePago::Cripto { direccion } => 5,

        }
    }
    
}

//Estructura de Usuario

#[derive(Debug, Clone)]
struct Usuario {
    nombre: String,
    suscripcion: Option<Suscripcion>,
    medio_de_pago: Option<MedioDePago>,
}

impl Usuario {

    /*➢  Crear un usuario con una determinada suscripción y medio de pago.  */

    fn new(nombre: String, suscripcion: Suscripcion, medio_de_pago: MedioDePago) -> Usuario{
        Usuario{
            nombre,
            suscripcion: Some(suscripcion),
            medio_de_pago: Some(medio_de_pago),
        }
    }

    

    fn upgrade(&mut self) -> bool{
        if let Some(sus) = &mut self.suscripcion {
            match sus.tipo_suscripcion {
                TipoSuscripcion::Basic => {sus.tipo_suscripcion = TipoSuscripcion::Clasic; return true},
                TipoSuscripcion::Clasic => {sus.tipo_suscripcion = TipoSuscripcion::Super; return true},
                TipoSuscripcion::Super => {println!("No se puede hacer upgrade: - Ya es un usuario Super."); return false}
            }
        } else {println!("No se puede hacer upgrade: -No tiene una suscripcion activa."); return false}
    }

    /*➢  Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la 
    suscripción es del tipo Basic al hacerlo se cancelará la suscripción. */

    fn downgrade(&mut self) -> bool{
        if let Some(sus) = &mut self.suscripcion {
            match sus.tipo_suscripcion {
                TipoSuscripcion::Super => {sus.tipo_suscripcion = TipoSuscripcion::Clasic; return true},
                TipoSuscripcion::Clasic => {sus.tipo_suscripcion = TipoSuscripcion::Basic; return true},
                TipoSuscripcion::Basic => {self.suscripcion = None; return true}
            }
        } else {println!("No se puede hacer downgrade: - No tiene una suscripcion activa."); return true}
    }

    /*➢  Dado un usuario cancelar la suscripción.  */

    fn cancelar(&mut self) -> bool {
        if self.suscripcion.is_some() {
            self.suscripcion = None;
            return true
        }
        else { println!("No se puede cancelar: - No tiene una suscripcion activa."); return false}
    }
}

struct Plataforma {
    usuarios: Vec<Usuario>,
}
impl Plataforma {

    fn new() -> Plataforma {
        Plataforma { usuarios: Vec::new() }
    }

    fn agregar_usuario(&mut self, usuario: Usuario) {
        self.usuarios.push(usuario);
    }

    fn devolver_tipo_pago(num: usize) -> String {
        match num {
            1 => "Efectivo".to_string(),
            2 => "MercadoPago".to_string(),
            3 => "TarjetaCredito".to_string(),
            4 => "TransferenciaBancaria".to_string(),
            5 => "Cripto".to_string(),
            _ => "Tipo no valido.".to_string(),
        }
    }

    fn devolver_tipo_suscripcion(num: usize) -> String {
        match num {
            0 => "Basic".to_string(),
            1 => "Clasic".to_string(),
            2 => "Super".to_string(),
            _ => "Suscripcion no valida.".to_string(),
        }
    }

    /*➢  Saber el medio de pago que es más utilizado por los usuarios sobre las 
suscripciones activas  */
    fn medio_mas_usado_activos (&self) -> String {
        let mut contadores = [0, 0, 0, 0, 0]; 
        for usuario in self.usuarios.clone() {
            if usuario.suscripcion.is_some() {
                if let Some(mdp) = usuario.medio_de_pago {
                    contadores[mdp.buscar_tipo() - 1] += 1;
                }
            }
        }

        let mut max_i = 0;
        let mut max = contadores[0];
        for (i, &valor) in contadores.iter().enumerate() {
            if valor > max {
                max = valor;
                max_i = i;
            }
        }

        Self::devolver_tipo_pago(max_i)
    }

    /*➢  Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones 
    activas. */

    fn suscripcion_mas_usada_activos (&self) -> String {
        let mut contadores = [0, 0, 0]; 
        for usuario in self.usuarios.clone() {
            if let Some(sus) = usuario.suscripcion {
                contadores[sus.tipo_suscripcion.buscar_tipo() - 1] += 1;
            }
        }
        let mut max_i = 0;
        let mut max = contadores[0];
        for (i, &value) in contadores.iter().enumerate() {
            if value > max {
                max = value;
                max_i = i;
            }   
        }

        Self::devolver_tipo_suscripcion(max_i)
    }


    fn medio_mas_usado (&self) -> String {
        let mut contadores = [0, 0, 0, 0, 0]; 
        for usuario in self.usuarios.clone() {
            if usuario.suscripcion.is_some() {
                if let Some(mdp) = usuario.medio_de_pago {
                    contadores[mdp.buscar_tipo() - 1] += 1;
                }
            }
        }

        let mut max_i = 0;
        let mut max = contadores[0];
        for (i, &valor) in contadores.iter().enumerate() {
            if valor > max {
                max = valor;
                max_i = i;
            }
        }

        Self::devolver_tipo_pago(max_i)
    }
}