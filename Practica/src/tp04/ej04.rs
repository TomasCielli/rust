/*

Se requiere implementar un sistema de ventas de productos. 

De cada producto se conoce el nombre, 
                              una categoría y 
                              un precio base, 
                              y algunos productos pueden tener descuentos aplicables dependiendo de la categoría. 
                              
Además, se debe registrar al vendedor que realizó la venta y al cliente. 

De ellos se conoce nombre, 
                   apellido, 
                   dirección, 
                   dni y 
                   del vendedor 
                        nro de legajo, 
                        antigüedad y 
                        salario. 

Los clientes pueden tener un beneficio de descuento si tienen suscripción al newsletter, de ser así se tiene el correo electrónico del 
mismo. 
 
El sistema debe permitir registrar las ventas realizadas y asociar el medio de pago utilizado. 

Los medios de pago aceptados son: tarjeta de crédito, 
                                  tarjeta de débito, 
                                  transferencia bancaria y 
                                  efectivo. 
 
 
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las 
siguientes acciones: 
 
    ➢   Crear una venta con: fecha, 
                              cliente, 
                              vendedor, 
                              medio de pago y 
                              un listado de productos con sus cantidades. 

    ➢   Calcular el precio final de una venta en base a los productos que hay en ella. Para 
    calcularlo tenga en cuenta que pueden haber determinados productos de alguna 
    categoría donde debería aplicarse un descuento. Tanto la categoría como el 
    porcentaje de descuento a aplicar son datos que le brinda el sistema. Es decir el 
    sistema tiene una lista de las categorías con el descuento a aplicar. Además se debe 
    aplicar un porcentaje de descuento general si el cliente tiene suscripción al 
    newsletter. 

    ➢  Para llevar un control de las ventas realizadas, se debe implementar un reporte que 
    permita visualizar las ventas totales por categoría de producto y otro por vendedor. 

*/

//USES
#![allow(dead_code)]
use std::{collections::HashMap, hash::Hash};
use crate::tp03::ej03::Fecha;


//TRAITS

pub trait ParaEnums {

    fn mostrar(&self) -> String;

    fn numero(&self) -> usize;
}


//CATEGORIAS

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Categoria {
    Alimento,
    Electronica,
    Ropa,
    Limpieza,
    Otro,
}


impl ParaEnums for Categoria {

    fn mostrar(&self) -> String {
        match self {
            Categoria::Alimento => "Alimento".to_string(),
            Categoria::Electronica =>"Electronica".to_string(),
            Categoria::Ropa =>"Ropa".to_string(),
            Categoria::Limpieza =>"Limpieza".to_string(),
            Categoria::Otro =>"Otro".to_string(),
        }
    }

    fn numero(&self) -> usize {
        match self {
            Categoria::Alimento => 0,
            Categoria::Electronica => 1,
            Categoria::Ropa => 2,
            Categoria::Limpieza => 3,
            Categoria::Otro => 4,
        }
    }
}



//MEDIOS DE PAGO

#[derive(Debug)]
enum MedioDePago {
    Credito,
    Debito,
    Transferencia,
    Efectivo,
}
impl ParaEnums for MedioDePago {

    fn mostrar(&self) -> String {
        match self {
            MedioDePago::Credito => "Credito".to_string(),
            MedioDePago::Debito =>"Debito".to_string(),
            MedioDePago::Transferencia =>"Transferencia".to_string(),
            MedioDePago::Efectivo =>"Efectivo".to_string(),
        }
    }

    fn numero(&self) -> usize {
        match self {
            MedioDePago::Credito => 0,
            MedioDePago::Debito => 1,
            MedioDePago::Transferencia => 2,
            MedioDePago::Efectivo => 3,
        }
    }
}



//PRODUCTOS

#[derive(Debug)]
struct Producto {
    nombre: String,
    categoria: Categoria,
    precio: f64,
}
impl Producto {

    fn new(nombre: String, categoria: Categoria, precio: f64) -> Producto {
        Producto {nombre, categoria, precio}
    }
    
    fn get_nombre(&self) -> String{
        self.nombre.clone()
    }

    fn get_categoria(&self) -> Categoria{
        self.categoria.clone()
    }

    fn get_precio(&self) -> f64 {
        self.precio
    }

    fn set_nombre(&mut self, nombre: String) -> bool{
        self.nombre = nombre;
        true
    }

    fn set_categoria(&mut self, categoria: Categoria) -> bool{
        self.categoria = categoria;
        true
    }

    fn set_precio(&mut self, precio: f64) -> bool{
        self.precio = precio;
        true
    }

    fn precio_con_descuento(&self, descuentos_por_categoria: &HashMap<Categoria, f64>) -> f64{
        if let Some(descuento) = descuentos_por_categoria.get(&self.categoria) {
            return (self.get_precio() / 100.0) * descuento
        }
        return self.get_precio()

    }
}


//PERSONAS

#[derive(Debug, Clone, PartialEq)]
struct Persona {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: u32,
}
impl Persona {

    fn new(nombre: String, apellido: String, direccion: String, dni: u32) -> Persona {
        Persona { nombre, apellido, direccion, dni }
    }

    fn get_nombre(&self) -> String{
        self.nombre.clone()
    }

    fn get_apellido(&self) -> String{
        self.apellido.clone()
    }

    fn get_direccion(&self) -> String{
        self.direccion.clone()
    }

    fn get_dni(&self) -> u32{
        self.dni
    }

    fn set_nombre(&mut self, nombre: String) -> bool{
        self.nombre = nombre;
        true
    }

    fn set_apellido(&mut self, apellido: String) -> bool{
        self.apellido = apellido;
        true
    }

    fn set_direccion(&mut self, direccion: String) -> bool{
        self.direccion = direccion;
        true
    }
    
    fn set_dni(&mut self, dni: u32) -> bool{
        self.dni = dni;
        true
    }
}



//CLIENTES

#[derive(Debug, Clone)]
struct Cliente {
    persona: Persona,
    email: Option<String>, // Option, porq puede o no estar suscripto
}
impl Cliente {

    fn new(nombre: String, apellido: String, direccion: String, dni: u32, email: Option<String>) -> Cliente {
        let persona = Persona::new(nombre, apellido, direccion, dni);
        Cliente{persona, email}
    }

    fn get_nombre(&self) -> String{
        self.persona.get_nombre()
    }

    fn get_apellido(&self) -> String{
        self.persona.get_apellido().clone()
    }

    fn get_direccion(&self) -> String{
        self.persona.get_direccion().clone()
    }

    fn get_dni(&self) -> u32{
        self.persona.get_dni()
    }

    fn get_email(&self) -> Option<String>{
        if let Some(email) = self.email.clone() {
            return Some(email)
        } else {return None}
    }

    fn set_nombre(&mut self, nombre: String) -> bool{
        self.persona.set_nombre(nombre)
    }

    fn set_apellido(&mut self, apellido: String) -> bool{
        self.persona.set_apellido(apellido)
    }

    fn set_direccion(&mut self, direccion: String) -> bool{
        self.persona.set_direccion(direccion)
    }
    
    fn set_dni(&mut self, dni: u32) -> bool{
        self.persona.set_dni(dni)
    }

    fn set_email(&mut self, email: Option<String>) -> bool{
        self.email = email;
        true
    }

    fn tiene_descuento(&self) -> bool{
        if self.get_email().is_some() {
            return true
        }
        false
    }
}



//VENDEDORES

#[derive(Debug, Clone)]
struct Vendedor {
    persona: Persona,
    legajo: u32,
    antiguedad: u8,
    salario: f64,
}
impl Vendedor{
    fn new(nombre: String, apellido: String, direccion: String, dni: u32, legajo: u32, antiguedad: u8, salario: f64) -> Vendedor {
        let persona = Persona::new(nombre, apellido, direccion, dni);
        Vendedor{persona, legajo, antiguedad, salario}
    }

    fn get_nombre(&self) -> String{
        self.persona.get_nombre()
    }

    fn get_apellido(&self) -> String{
        self.persona.get_apellido().clone()
    }

    fn get_direccion(&self) -> String{
        self.persona.get_direccion().clone()
    }

    fn get_dni(&self) -> u32{
        self.persona.get_dni()
    }

    fn get_legajo(&self) -> u32{
        self.legajo
    }

    fn get_antiguedad(&self) -> u8{
        self.antiguedad
    }

    fn get_salario(&self) -> f64{
        self.salario
    }

    fn set_nombre(&mut self, nombre: String) -> bool{
        self.persona.set_nombre(nombre)
    }

    fn set_apellido(&mut self, apellido: String) -> bool{
        self.persona.set_apellido(apellido)
    }

    fn set_direccion(&mut self, direccion: String) -> bool{
        self.persona.set_direccion(direccion)
    }
    
    fn set_dni(&mut self, dni: u32) -> bool{
        self.persona.set_dni(dni)
    }

    fn set_legajo(&mut self, legajo: u32) -> bool{
        self.legajo = legajo;
        true
    }

    fn set_antiguedad(&mut self, antiguedad: u8) -> bool{
        self.antiguedad = antiguedad;
        true
    }

    fn set_salario(&mut self, salario: f64) -> bool{
        self.salario = salario;
        true
    }

    fn igual(&self, otro: &Self) -> bool {
        self.get_dni() == otro.get_dni() &&
        self.get_legajo() == otro.get_legajo() &&
        self.get_nombre() == otro.get_nombre() &&
        self.get_apellido() == otro.get_apellido() &&
        self.get_direccion() == otro.get_direccion()
    }
}



//VENTAS

#[derive(Debug)]
struct Venta {
    fecha: Fecha,
    cliente: Cliente,
    vendedor: Vendedor,
    medio_de_pago: MedioDePago,
    productos: HashMap<Producto, u32>// (producto , cantidad)
}
impl Venta{

    fn new(fecha: Fecha, cliente: Cliente, vendedor: Vendedor, medio_de_pago: MedioDePago, productos:HashMap<Producto, u32>) -> Venta{
        Venta{fecha, cliente, vendedor, medio_de_pago, productos}
    }

    fn buscar_descuento(producto: Producto, descuentos_por_categoria: &HashMap<Categoria, f64>) -> f64{
        if let Some(descuento) = descuentos_por_categoria.get(&producto.categoria) {
            return *descuento
        }
        return 0.0
    }

    fn get_fecha(&self) -> &Fecha{
        &self.fecha
    }
    fn get_cliente(&self) -> &Cliente{
        &self.cliente
    }
    fn get_vendedor(&self) -> Vendedor{
        self.vendedor.clone()
    }
    fn get_medio_de_pago(&self) -> &MedioDePago{
        &self.medio_de_pago
    }
    fn get_productos(&self) -> &HashMap<Producto, u32>{
        &self.productos
    }
    fn set_fecha(&mut self, fecha: Fecha) -> bool{
        self.fecha = fecha;
        true
    }
    fn set_cliente(&mut self, cliente: Cliente) -> bool{
        self.cliente = cliente;
        true
    }
    fn set_vendedor(&mut self, vendedor: Vendedor) -> bool{
        self.vendedor = vendedor;
        true
    }
    fn set_medio_de_pago(&mut self, medio_de_pago: MedioDePago) -> bool{
        self.medio_de_pago = medio_de_pago;
        true
    }
    fn set_productos(&mut self, productos: HashMap<Producto, u32>) -> bool{
        self.productos = productos;
        true
    }



    fn precio_final(&self, descuentos_por_categoria: &HashMap<Categoria, f64>, descuento_suscripto: f64) -> f64{
        let mut total = 0.0;
        for producto in self.get_productos() {
            let cant: u32 = *producto.1;
            total += producto.0.precio_con_descuento(descuentos_por_categoria) * (cant as f64)
        }
        if self.get_cliente().tiene_descuento(){
            total -= (total/100.0) * descuento_suscripto;
        }
        total
    }


}
/*    ➢  Para llevar un control de las ventas realizadas, se debe implementar un reporte que 
    permita visualizar las ventas totales por categoría de producto y otro por vendedor.  */

struct Reporte {
    ventas: Vec<Venta>,
}
impl Reporte {
    
    fn new(ventas: Vec<Venta>) -> Reporte {
        Reporte{ventas}
    }

    fn ventas_por_categoria(&self, descuentos_por_categoria: &HashMap<Categoria, f64>, descuento_suscripto: f64) -> HashMap<Categoria, f64> {
        let mut reporte: HashMap<Categoria, f64> = HashMap::new();
        for venta in &self.ventas {
            for (producto, cantidad) in venta.get_productos() {
                let categoria = producto.get_categoria();
                let precio_con_descuentos = if venta.get_cliente().tiene_descuento() {
                    (producto.precio_con_descuento(descuentos_por_categoria) / 100.0) * descuento_suscripto
                } else {
                    producto.precio_con_descuento(descuentos_por_categoria)
                };
                let total_producto = precio_con_descuentos * (*cantidad as f64);
                *reporte.entry(categoria).or_insert(0.0) += total_producto;
            }
        }
        reporte
    }

    fn  ventas_por_vendedor(&self, descuentos_por_categoria: &HashMap<Categoria, f64>, descuento_suscripto: f64) -> Vec<(Vendedor, f64)> {
        let mut reporte: Vec<(Vendedor, f64)> = Vec::new();
        for venta in &self.ventas {
            let vendedor = venta.get_vendedor();
            let total_venta = venta.precio_final(descuentos_por_categoria, descuento_suscripto);
            if let Some((_, total)) = reporte.iter_mut().find(|(v, _)| v.igual(&vendedor)) {
                *total += total_venta;
            } else {
                reporte.push((vendedor, total_venta));
            }
        }
        reporte
    }
}
