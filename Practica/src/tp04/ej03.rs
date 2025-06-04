/*

La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones (Basic, Clasic, Super) a sus usuarios.
 
Cada suscripción tiene un costo mensual y una duración de meses y una fecha de inicio, 
además los usuarios pueden pagar por sus suscripciones con distintos medios de pago que son Efectivo, 
                                                                                            MercadoPago, 
                                                                                            Tarjeta de Crédito, 
                                                                                            Transferencia Bancaria, 
                                                                                            Cripto. 
Cada medio de pago tiene sus datos correspondientes a excepción de Efectivo. 

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

const COSTO_BASIC: f64 = 10000.0;
const COSTO_CLASIC: f64 = 20000.0;
const COSTO_SUPER: f64 = 40000.0;

#[derive(Clone)]
#[allow(dead_code)]

struct Suscripcion {
    costo_mensual: f64,
    duracion_meses: u8,
    inicio: Fecha,
}
impl Suscripcion {

    //CONSTRUCTOR
    fn new(costo_mensual: f64, duracion_meses: u8, inicio: Fecha) -> Suscripcion{
        Suscripcion{
            costo_mensual,
            duracion_meses,
            inicio,
        }
    }

    //GETTERS

        //Costo Mensual
        fn get_costo_mensual(&self) -> f64 {
            self.costo_mensual
        }

        //Duracion Meses
        fn get_duracion_meses(&self) -> u8 {
            self.duracion_meses
        }
        //Inicio
        fn get_inicio(&self) -> Fecha {
            self.inicio.clone()
        }
    //SETTERS

        //Costo Mensual
        fn set_costo_mensual(&mut self, costo: f64) -> bool {
            if costo > 0.0 {
                self.costo_mensual = costo;
                return true
            }
            false
        }

        //Duracion Meses
        fn set_duracion_meses(&mut self, duracion: u8) -> bool {
            if duracion > 0 {
                self.duracion_meses = duracion;
                return true
            }
            false
        }

        //Inicio
        fn set_inicio(&mut self, inicio: Fecha) -> bool {
            if inicio.es_fecha_valida() {
                self.inicio = inicio;
                return true
            }
            false
        }

}
enum TipoSuscripcion {
    Basic {suscripcion_basic: Suscripcion},
    Clasic {suscripcion_clasic: Suscripcion},
    Super {suscripcion_super: Suscripcion},
}
impl TipoSuscripcion{
    
    fn get_tipo_suscripcion(&self) -> (String, &Suscripcion) {
        match self {
            TipoSuscripcion::Basic { suscripcion_basic } => ("Basic".to_string(), suscripcion_basic),
            TipoSuscripcion::Clasic { suscripcion_clasic } => ("Clasic".to_string(), suscripcion_clasic),
            TipoSuscripcion::Super { suscripcion_super } => ("Super".to_string(), suscripcion_super)
        }
    }
}

enum MedioDePago {
    Efectivo,
    MercadoPago { cvu: String },
    TarjetaCredito { numero: String, vencimiento: Fecha },
    TransferenciaBancaria { cbu: String },
    Cripto { direccion: String },
}
impl MedioDePago {
    fn get_medio_de_pago(&self) -> String {
        match self {
            MedioDePago::Efectivo => "Efectivo".to_string(),
            MedioDePago::MercadoPago{cvu} => format!("MercadoPago | CVU: {}", cvu),
            MedioDePago::TarjetaCredito{numero, vencimiento} => format!("Tarjeta de Credito | Numero: {}, Vencimiento: {}", numero, vencimiento.mostrar()),
            MedioDePago::TransferenciaBancaria{cbu} => format!("Transferencia Bancaria | CBU: {}", cbu),
            MedioDePago::Cripto {direccion} => format!("Cripto | Direccion: {}", direccion),
        }
    }
}

struct Usuario {
    suscripcion: Option<TipoSuscripcion>,
    medio_de_pago: MedioDePago,
}

impl Usuario {

    //➢  Crear un usuario con una determinada suscripción y medio de pago. 
    fn new(suscripcion: Option<TipoSuscripcion>, medio_de_pago: MedioDePago) -> Usuario {
        Usuario {
            suscripcion, 
            medio_de_pago
        }
    }
    

    // ➢  Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic pasa a Clasic y si está en Clasic pasa a Super. 
    fn upgrade(&mut self) -> bool {
        if let Some(ref sus) = self.suscripcion {
            let duracion = sus.get_tipo_suscripcion().1.get_duracion_meses();
            let inicio = sus.get_tipo_suscripcion().1.get_inicio();

            if sus.get_tipo_suscripcion().0  == "Basic".to_string() {
                let suscripcion_clasic = Suscripcion::new(COSTO_CLASIC, duracion, inicio);
                self.suscripcion = Some(TipoSuscripcion::Clasic {suscripcion_clasic});
                return true
            } 
            else if sus.get_tipo_suscripcion().0 == "Clasic".to_string() {
                let suscripcion_super = Suscripcion::new(COSTO_SUPER, duracion, inicio);
                self.suscripcion = Some(TipoSuscripcion::Super {suscripcion_super});
                return true
            }
            println!("No se puede mejorar la suscripcion porque esta ya esta en Super.");
            return false
            }
        else {
            println!("El usuario no tiene una suscripcion.");
            return false
        }
        }
        

        /*➢  Dado un usuario cancelar la suscripción.  */

        fn cancelar(&mut self) -> bool {
            if let Some(ref sus) = self.suscripcion {
                self.suscripcion = None;
                return true
            }
            else {
                println!("El usuario no tiene una suscripcion.");
                return false
            }
        }

        /*➢  Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la 
        suscripción es del tipo Basic al hacerlo se cancelará la suscripción.  */

        fn downgrade(&mut self) -> bool {
            if let Some(ref sus) = self.suscripcion {
                let duracion = sus.get_tipo_suscripcion().1.get_duracion_meses();
                let inicio = sus.get_tipo_suscripcion().1.get_inicio();

                if sus.get_tipo_suscripcion().0  == "Super".to_string() {
                    let suscripcion_clasic = Suscripcion::new(COSTO_CLASIC, duracion, inicio);
                    self.suscripcion = Some(TipoSuscripcion::Clasic {suscripcion_clasic});
                    return true
                } 
                else if sus.get_tipo_suscripcion().0 == "Clasic".to_string() {
                    let suscripcion_basic = Suscripcion::new(COSTO_BASIC, duracion, inicio);
                    self.suscripcion = Some(TipoSuscripcion::Basic {suscripcion_basic});
                    return true
                }
                self.cancelar()
            }
            else {
                println!("El usuario no tiene una suscripcion.");
                return false
            }
        }

    }

    /*➢  Saber el medio de pago que es más utilizado por los usuarios sobre las 
    suscripciones activas */ 
    
    /*➢  Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones 
    activas. */
    
    /*➢  Saber cuál fue el medio de pago más utilizado. */
    
    /*➢  Saber cuál fue la suscripción más contratada. */






