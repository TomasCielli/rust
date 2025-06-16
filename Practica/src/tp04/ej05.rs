#![allow(dead_code)]
use std::collections::HashMap;
use std::fmt::Display;
use crate::tp03::ej03::Fecha;
use rand::Rng;
use rand::rng;

trait Igual{

    fn es_igual(&self, otro: &Self) -> bool;
}
    impl Igual for f64{

    fn es_igual(&self, otro: &f64) -> bool{
        *self == *otro
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MedioDePago{
    MercadoPago,
    TransferenciaBancaria,
}

#[derive(Debug)]
pub enum ErrorPlataforma {
    UsuarioNoValidado(String),
    BalanceInsuficiente(f64),
    TransaccionInvalido(String),
    BlockchainInvalida(String),
    MontoInvalido(f64),
}
    impl Display for ErrorPlataforma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorPlataforma::UsuarioNoValidado(nombre) => write!(f, "El usuario {} no esta validado.", nombre),
            ErrorPlataforma::BalanceInsuficiente(balance) => write!(f, "Balance insuficiente: {}.", balance),
            ErrorPlataforma::TransaccionInvalido(transaccion) => write!(f, "El tipo de transaccion '{}' no es valido.", transaccion),
            ErrorPlataforma::BlockchainInvalida(nombre) => write!(f, "La blockchain '{}' no es valida.", nombre),
            ErrorPlataforma::MontoInvalido(monto) => write!(f, "El monto {} no es valido.", monto),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blockchain{
    nombre: String,
    prefijo: String,
}
    impl Blockchain{

    fn new(nombre: String, prefijo: String) -> Blockchain {
        Blockchain { 
            nombre, 
            prefijo
        }
    }

    fn devolver_hash(&self) -> String{
        let mut generador = rng();
        let hash = format!("{}-{}", self.nombre, generador.random_range(10000..999999999));
        hash

    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cripto{
    nombre: String,
    prefijo: String,
    listado_blockchains: Vec<Blockchain>,
}
    impl Cripto {

    fn new(nombre: String, prefijo: String, listado_blockchains: Vec<Blockchain>) -> Cripto{
        Cripto{
            nombre,
            prefijo,
            listado_blockchains,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TipoTransaccion <'a>{
    Ingreso {monto: f64},
    CompraCripto {cripto: Cripto , monto_cripto: f64, cotizacion: f64},
    VentaCripto{cripto: Cripto,  monto_cripto: f64, cotizacion: f64},
    RetirarCripto{cripto: Cripto, monto_cripto: f64, cotizacion: f64, blockchain: &'a Blockchain, hash: String},
    RecepcionCripto{cripto: Cripto, monto_cripto: f64, cotizacion: f64, blockchain:&'a Blockchain},
    RetirarFiat{monto: f64, medio: MedioDePago}
}
    impl <'a> TipoTransaccion <'a>{
    fn to_string(&self) -> String{
        match self{
            TipoTransaccion::Ingreso {..} => "Ingreso".to_string(),
            TipoTransaccion::CompraCripto {..} => "CompraCripto".to_string(),
            TipoTransaccion::VentaCripto {..} => "VentaCripto".to_string(),
            TipoTransaccion::RetirarCripto {..} => "RetirarCripto".to_string(),
            TipoTransaccion::RecepcionCripto {..} => "RecepcionCripto".to_string(),
            TipoTransaccion::RetirarFiat {..} => "RetirarFiat".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Transaccion <'a>{
    fecha: Fecha,
    tipo: TipoTransaccion <'a>,
    usuario: Usuario,
}
    impl <'a> Transaccion<'a>{

    fn new (fecha: Fecha, tipo: TipoTransaccion, usuario: Usuario) -> Transaccion{
        Transaccion{fecha, tipo, usuario}
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Usuario{
    nombre: String,
    apellido: String,
    email: String,
    dni: u32,
    validado: bool,
    balance: HashMap<Cripto, f64>,
    balance_fiat: f64,
}
    impl Usuario{
    
    fn new(nombre: String, apellido: String, email: String, dni: u32) -> Usuario{
        Usuario{
            nombre,
            apellido,
            email,
            dni,
            validado: false,
            balance: HashMap::new(),
            balance_fiat: 0.0,
        }
    }

    fn validar_usuario(&mut self) -> Result<bool, ErrorPlataforma>{
        if self.validado{
            return Ok(true)
        }
        return Err(ErrorPlataforma::UsuarioNoValidado(format!("{} {}",self.apellido, self.nombre)))
    }

    fn verificar_balance_fiat(&mut self, monto_fiat: f64) -> Result<bool, ErrorPlataforma>{
        if self.balance_fiat >= monto_fiat {
            return Ok(true)
        }
        Err(ErrorPlataforma::BalanceInsuficiente(self.balance_fiat))
        
    }

    fn verificar_balance_cripto(&mut self, cripto: Cripto, monto_cripto: f64) -> Result<&mut f64, ErrorPlataforma> {
        if let Some(balance_cripto) = self.balance.get_mut(&cripto){
            if *balance_cripto  >= monto_cripto {
                return Ok(balance_cripto)
            }else {
                return Err(ErrorPlataforma::BalanceInsuficiente(*balance_cripto))
            }
        }else {
            return Err(ErrorPlataforma::BalanceInsuficiente(0.0))
        }
    }

    fn ingresar_dinero (&mut self, monto: f64, fecha_hoy: &Fecha) -> Result <Transaccion, ErrorPlataforma>{
        match self.validar_usuario(){
            Err(e) => Err(e),

            Ok(_) => {

                if monto > 0.0{
                    self.balance_fiat += monto;
                    return Ok(Transaccion::new(fecha_hoy.clone(), TipoTransaccion::Ingreso {monto}, self.clone()))
                }
                else{ return Err(ErrorPlataforma::MontoInvalido(monto))}
            }
        }
    }

    fn comprar_criptomoneda(&mut self, monto_fiat: f64, cripto: Cripto, cotizacion: f64, fecha_hoy: &Fecha) -> Result<Transaccion, ErrorPlataforma>{
        match self.validar_usuario(){
            Err(e) => Err(e),

            Ok(_) =>{
                if monto_fiat <= 0.0 {return Err(ErrorPlataforma::MontoInvalido(monto_fiat))}
                match self.verificar_balance_fiat(monto_fiat){
                    Err(e)=> Err(e),

                    Ok(_)=> {
                        self.balance_fiat -= monto_fiat;
                        let monto_cripto = monto_fiat / cotizacion;
                        *self.balance.entry(cripto.clone()).or_insert(0.0) += monto_cripto;
                        return Ok(Transaccion::new(fecha_hoy.clone(), TipoTransaccion::CompraCripto { cripto, monto_cripto, cotizacion }, self.clone()))
                    }
                }
            }
        }
    }

    fn vender_criptomoneda(&mut self, monto_cripto: f64, cripto: Cripto, cotizacion: f64, fecha_hoy: &Fecha) -> Result<Transaccion, ErrorPlataforma>{
        match self.validar_usuario(){
            Err(e) => Err(e),

            Ok(_) => {
                if monto_cripto <= 0.0 {return Err(ErrorPlataforma::MontoInvalido(monto_cripto))}
                match self.verificar_balance_cripto(cripto.clone(), monto_cripto) {
                    Err(e) => Err(e),

                    Ok(balance_cripto) => {
                        *balance_cripto -= monto_cripto;
                        self.balance_fiat += monto_cripto * cotizacion;
                        return Ok(Transaccion::new(fecha_hoy.clone(), TipoTransaccion::VentaCripto { cripto, monto_cripto, cotizacion}, self.clone()))
                    }
                }
            }
        }
    }

    fn retirar_criptomoneda<'a>(&mut self, monto_cripto: f64, cripto: Cripto, cotizacion: f64, fecha_hoy: &Fecha, blockchain: &'a Blockchain) -> Result<Transaccion<'a>, ErrorPlataforma>{
        match self.validar_usuario(){
            Err(e) => Err(e),

            Ok(_) =>{
                if !cripto.listado_blockchains.contains(blockchain) {return Err(ErrorPlataforma::BlockchainInvalida(blockchain.clone().nombre))}
                match self.verificar_balance_cripto(cripto.clone(), monto_cripto) {
                    Err(e) => Err(e),

                    Ok(balance_cripto)=>{
                        *balance_cripto -= monto_cripto;
                        let hash = blockchain.devolver_hash();
                        Ok(Transaccion::new(fecha_hoy.clone(),
                                            TipoTransaccion::RetirarCripto { cripto, monto_cripto, cotizacion, blockchain, hash},
                                            self.clone()))
                    }
                }
            }
        }
    }

    fn recibir_criptomoneda<'a>(&mut self, monto_cripto: f64, cripto: Cripto, cotizacion: f64, fecha_hoy: &Fecha, blockchain: &'a Blockchain) -> Result<Transaccion<'a>, ErrorPlataforma>{
        if cripto.listado_blockchains.contains(&blockchain){
            *self.balance.entry(cripto.clone()).or_insert(monto_cripto) += monto_cripto;
            Ok(Transaccion::new(fecha_hoy.clone(), TipoTransaccion::RecepcionCripto { cripto, monto_cripto, cotizacion, blockchain}, self.clone()))
        }else {
            Err(ErrorPlataforma::BlockchainInvalida(blockchain.clone().nombre))
        }
    }       

    fn retirar_fiat(&mut self, monto: f64, fecha_hoy: &Fecha, medio:MedioDePago ) -> Result <Transaccion, ErrorPlataforma>{
        match self.validar_usuario(){
            Err(e) => Err(e),

            Ok(_) => {
                if monto <= 0.0 {return Err(ErrorPlataforma::MontoInvalido(monto))}
                match self.verificar_balance_fiat(monto){
                    Err(e) => Err(e),

                    Ok(_) =>{
                        self.balance_fiat -= monto;
                        return Ok(Transaccion::new(fecha_hoy.clone(),
                                                TipoTransaccion::RetirarFiat { monto, medio},
                                                self.clone()))
                    }
                }
            }
        }
    }
}

fn cripto_con_mas_compras(transacciones: Vec<Transaccion>) -> Option<(Cripto, usize)> {
    let mut contador: HashMap<Cripto, usize> = HashMap::new();

    for transaccion in transacciones {
        if let TipoTransaccion::CompraCripto { cripto, .. } = transaccion.tipo {
            *contador.entry(cripto.clone()).or_insert(0) += 1;
        }
    }

    contador.into_iter().max_by_key(|(_, cantidad)| *cantidad)
}

fn cripto_con_mas_ventas(transacciones: Vec<Transaccion>) -> Option<(Cripto, usize)> {
    let mut contador: HashMap<Cripto, usize> = HashMap::new();

    for transaccion in transacciones {
        if let TipoTransaccion::VentaCripto { cripto, .. } = transaccion.tipo {
            *contador.entry(cripto.clone()).or_insert(0) += 1;
        }
    }

    contador.into_iter().max_by_key(|(_, cantidad)| *cantidad)
}

fn cripto_con_mas_volumen_ventas(transacciones: Vec<Transaccion>) -> Option<(Cripto, f64)> {
    if transacciones.is_empty(){
        return None
    } else{
        let mut volumenes: HashMap<Cripto, f64> = HashMap::new();

        for transaccion in transacciones{
            if let TipoTransaccion::VentaCripto { cripto, monto_cripto, .. } = transaccion.tipo{
                *volumenes.entry(cripto).or_insert(0.0) += monto_cripto
            }
        }

        volumenes.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }
}

fn cripto_con_mas_volumen_compras(transacciones: Vec<Transaccion>) -> Option<(Cripto, f64)> {
    if transacciones.is_empty(){
        return None
    } else{
        let mut volumenes: HashMap<Cripto, f64> = HashMap::new();

        for transaccion in transacciones{
            if let TipoTransaccion::CompraCripto { cripto, monto_cripto, .. } = transaccion.tipo{
                *volumenes.entry(cripto).or_insert(0.0) += monto_cripto
            }
        }

        volumenes.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tp03::ej03::Fecha;

    fn crar_usuario_valido() -> Usuario{
        let mut u= Usuario::new("Tomas".to_string(), "Cielli".to_string(), "tomas.cielli@gmail.com".to_string(), 44587046);
        u.validado = true;
        u.balance_fiat = 600.0;
        u
    }

    fn crear_usuario_sin_validar () -> Usuario{
        Usuario::new("Tomas".to_string(), "Cielli".to_string(), "tomas.cielli@gmail.com".to_string(), 44587046)
    }    

    fn fecha_cualquiera() -> Fecha{
        Fecha::new(1, 1, 2025)
    }

    fn crear_listado_blockchains() -> Vec<Blockchain>{
        let b1 = Blockchain::new("B1".to_string(), "1".to_string());
        let b2 = Blockchain::new("B2".to_string(), "2".to_string());
        let b3 = Blockchain::new("B3".to_string(), "3".to_string());

        let vector: Vec<Blockchain> = vec![b1, b2, b3];
        vector
    }

    fn crear_cripto () -> Cripto {
        let listado_blockchains = crear_listado_blockchains();
        Cripto::new("Bitcoin".to_string(), "BTC".to_string(), listado_blockchains)
    }

    #[test]
    fn test_ingresar_dinero(){
        let mut validado = crar_usuario_valido();
        let mut sin_validar = crear_usuario_sin_validar();
        let fecha_hoy= fecha_cualquiera();

        let ok = validado.ingresar_dinero(500.0, &fecha_hoy);
        assert!(ok.is_ok());

        assert!(validado.balance_fiat == 1100.0);

        let mut err = validado.ingresar_dinero(-1.0, &fecha_hoy);
        assert!(err.is_err());

        err = sin_validar.ingresar_dinero(500.0, &fecha_hoy);
        assert!(err.is_err());

        err = sin_validar.ingresar_dinero(-1.0, &fecha_hoy);
        assert!(err.is_err());
        
    }

    #[test]
    fn test_retirar_fiat(){
        let mut con_fiat = crar_usuario_valido();

        let mut sin_fiat = crar_usuario_valido();
        sin_fiat.balance_fiat = 0.0;

        let fecha_hoy = fecha_cualquiera();

        let ok = con_fiat.retirar_fiat(500.0, &fecha_hoy, MedioDePago::MercadoPago);
        assert!(ok.is_ok());

        assert!(con_fiat.balance_fiat == 100.0);

        let mut err = con_fiat.retirar_fiat(-100.0, &fecha_hoy, MedioDePago::MercadoPago);
        assert!(err.is_err());

        err = con_fiat.retirar_fiat(500.0, &fecha_hoy, MedioDePago::MercadoPago);
        assert!(err.is_err());

        err = sin_fiat.retirar_fiat(600.0, &fecha_hoy, MedioDePago::TransferenciaBancaria);
        assert!(err.is_err());


    }

    #[test]
    fn test_comprar_criptomoneda(){
        let mut con_fiat = crar_usuario_valido();

        let mut sin_fiat = crar_usuario_valido();
        sin_fiat.balance_fiat = 0.0;

        let fecha_hoy = fecha_cualquiera();

        let cripto = crear_cripto();

        let ok = con_fiat.comprar_criptomoneda(500.0, cripto.clone(), 500.0, &fecha_hoy);
        assert!(ok.is_ok());

        let balance = *con_fiat.balance.get(&cripto.clone()).unwrap();
        assert!(balance == 1.0);

        let mut err = con_fiat.comprar_criptomoneda(200.0, cripto.clone(), 1000.0, &fecha_hoy);
        assert!(err.is_err());

        err = con_fiat.comprar_criptomoneda(-100.0, cripto.clone(), 10.0, &fecha_hoy);
        assert!(err.is_err());

        err = sin_fiat.comprar_criptomoneda(200.0, cripto.clone(), 1000.0, &fecha_hoy);
        assert!(err.is_err());
    }

    #[test]
    fn test_vender_criptomoneda(){
        let fecha_hoy = fecha_cualquiera();
        let mut con_cripto = crar_usuario_valido();

        let cripto = crear_cripto();

        con_cripto.balance.entry(cripto.clone()).or_insert(3.0);

        let ok = con_cripto.vender_criptomoneda(2.0, cripto.clone(), 1000.0, &fecha_hoy);
        assert!(ok.is_ok());

        assert!(con_cripto.balance_fiat == 2600.0);

        let mut err = con_cripto.vender_criptomoneda(-1.0, cripto.clone(), 1000.0, &fecha_hoy);
        assert!(err.is_err());
        
        err = con_cripto.vender_criptomoneda(2.0, cripto.clone(), 1000.0, &fecha_hoy);
        assert!(err.is_err()); 
    }

    #[test]
    fn test_retirar_criptomoneda(){
        let fecha_hoy = fecha_cualquiera();
        let mut con_cripto = crar_usuario_valido();

        let cripto = crear_cripto();
        let blockchain = cripto.listado_blockchains.first().unwrap();

        con_cripto.balance.entry(cripto.clone()).or_insert(3.0);

        let ok = con_cripto.retirar_criptomoneda(2.0, cripto.clone(), 1000.0, &fecha_hoy, blockchain);
        assert!(ok.is_ok());

        assert!(con_cripto.balance.get(&cripto.clone()) == Some(&1.0));

        let blockchain2 = Blockchain::new("zzz".to_string(), "xd".to_string());
        let err = con_cripto.retirar_criptomoneda(1.0, cripto.clone(), 1000.0, &fecha_hoy, &blockchain2);
        assert!(err.is_err());
    }

    #[test]
    fn test_recibir_criptomoneda(){
        let fecha_hoy = fecha_cualquiera();
        let mut con_cripto = crar_usuario_valido();

        let cripto = crear_cripto();
        let blockchain = cripto.listado_blockchains.first().unwrap();

        con_cripto.balance.entry(cripto.clone()).or_insert(3.0);

        let ok = con_cripto.recibir_criptomoneda(2.0, cripto.clone(), 1000.0, &fecha_hoy, blockchain);
        assert!(ok.is_ok());

        assert!(con_cripto.balance.get(&cripto.clone()) == Some(&5.0));

        let blockchain2 = Blockchain::new("zzz".to_string(), "xd".to_string());
        let err = con_cripto.recibir_criptomoneda(1.0, cripto.clone(), 1000.0, &fecha_hoy, &blockchain2);
        assert!(err.is_err());
    }
}