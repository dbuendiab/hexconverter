//! # Aplicación hexconverter
//! 
//! Para poder usar otros ficheros .rs en este main.rs, mod incluye el código del fichero entero
//! digit contiene el módulo digit, con las funciones get_char() y get_num()
//! numero contiene la estructura Numero y su implementación
//! A pesar de ser cosas distintas, el tratamiento es homogéneo.
mod digit;
mod numero;

// Asigna una ruta completa a un nombre
// En este caso, digitos es un módulo, y Numero es la estructura Numero
use digit::digitos;
use numero::Numero;

// Crate para tratamiento de argumentos
use std::env;

/// Esta función recibe los parámetros para un número `face` en base `base`
/// 
/// Genera un Número con el constructor `new()`, pero este constructor no es infalible
/// así que devuelve un `Result<Numero, String>` según la operación  haya funcionado o no
/// 
/// El caso `Ok()` es el que devuelve el `Numero n`, el otro `Err()` un mensaje de error
/// 
/// El problema es que yo estoy acostumbrado a 'acabar' con el error vía `try-catch` o
/// similar, de modo que saldría de esta función limpiamente, devolviendo solo `n`
/// 
/// El match debe devolver el mismo tipo en ambos brazos (es cierto?), pero veo que
/// si salgo del programa vía `exit()`, el compilador me lo perdona
/// 
fn get_numero(face: &str, base: usize) -> Numero {
    let r = Numero::new(String::from(face), base);
    match r {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            std::process::exit(0);
        }
    }
}


/// Esta función desempaqueta el Result<Numero, String> que
/// genera el método `Numero::to_base()`. Devuelve el número
/// o bien sale del programa. De esta forma me ahorro el discutir
/// con el match del Rust, que me obliga a devolver el mismo 
/// tipo en cada rama.
///
fn get_to_base(n: &Numero, base_dest: usize) -> Numero {
    match n.to_base(base_dest) {
        Ok(n) => {
            n
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(0);
        }
    }
}

#[doc(hidden)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!(r#"
        HexConverter: utilidad de conversión de bases numéricas

        Uso: hexconverter <numero> <base1> <base2>");
"#);
        std::process::exit(1);
    }

    let numero = &args[1];
    let base1: usize = args[2].parse().expect("El segundo argumento debe ser un número entero");
    let base2: usize = args[3].parse().expect("El tercer argumento debe ser un número entero");

    // La función hace exit en caso de error
    let n: Numero = crate::get_numero(numero, base1);
    let n2: Numero = crate::get_to_base(&n, base2);
    println!("Conversión {} -> {}", n, n2);

}

/* 
#[doc(hidden)]
// Función auxiliar para salir con mensaje
fn exit(msj: String, code: i32) {
    println!("{}", msj);
    std::process::exit(code);
}
 */