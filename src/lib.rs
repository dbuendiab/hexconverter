//! # Aplicación hexconverter
//!
//! **HexConverter** es una aplicación de línea de comandos para Windows
//! 
//! ```text
//! C:\> hexconverter <numero> <base1> <base2>
//! ```
//! ## Ejemplo
//! ```text
//! C:\> hexconverter 1001010 2 16
//! Conversión 1001010:2 -> 74 -> 4a:16
//! ```
//! 
//! La notación de la salida significa `<numero>:<base1> -> <numero base 10> -> <numero:base2>`
//! 
//! Los parámetros de HexConverter son 
//! * **numero**: La cantidad numérica a convertir
//! * **base1**: La base numérica en la que viene expresado el número
//! * **base2**: La base numérica de conversión
//! 
//! Las bases numéricas permitidas van de `2` a `35`, ambas inclusive
//! 
//! Los dígitos admisibles son estos: `0123456789abcdefghijklmnopqrstuvwxyz`
//! 
//! Pueden usarse mayúsculas, pero el programa las convierte a minúsculas
//! 

pub mod digit;
pub mod numero;
