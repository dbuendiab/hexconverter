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


// Esta función desempaqueta el Result<Numero, String> que
// genera el método `Numero::to_base()`. Devuelve el número
// o bien sale del programa. De esta forma me ahorro el discutir
// con el match del Rust, que me obliga a devolver el mismo 
// tipo en cada rama.

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
    // Primera prueba: dígitos
    //let n = run_digits();
    //println!("El último dígito es: {}", n);

    // Segunda prueba: numeros
    //run_number();

    // La función hace exit en caso de error
    let n: Numero = crate::get_numero("a02", 12);
    println!("Numero original: {}", n);

    // Prueba de cambio de base (directa)
    match n.to_base(10) {
        Ok(_n) => {}
        Err(e) => {
            crate::exit(e, 0);
        }
    };

    // Prueba de la función get_to_base(), que encapsula el match anterior
    let n8 = get_to_base(&n, 8);
    let n16 = get_to_base(&n, 16);

    println!("Los valores son {} y {}", n8.value(), n16.value());
    println!("Los faciales son {} y {}", n8.face(), n16.face());
    println!("Los números son {} y {}", n8, n16);
}


#[doc(hidden)]
// Función auxiliar para salir con mensaje
fn exit(msj: String, code: i32) {
    println!("{}", msj);
    std::process::exit(code);
}
/*
fn run_digits() -> usize {
    // Valor numérico de un dígito
    let c = 'x';
    let n = digitos::get_num(c).unwrap(); // Mejor match
    println!("{}", n);

    // Carácter para un valor numérico
    let n = 13;
    let c = digitos::get_char(n).unwrap();
    println!("{}", c);

    // Tratamiento del None (sin unwrap)
    let c = 'z';
    let n = digitos::get_num(c);
    match n {
        Some(n_out) => {
            println!("{}", n_out);
            n_out       // Devolver el u8
            },
        None => {
            println!("{}: Error dígito incorrecto", c);
            std::process::exit(1);
            }
    }
    //println!("{}", n);


}

fn run_number() {

    let n = Numero::new(String::from("A02"), 12);
    match n {
        Ok(num) => {
            let v = num.value();
            println!("El valor numérico es: {}", v);
            println!("El valor facial es: {}", num.face());

        },
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
 */
