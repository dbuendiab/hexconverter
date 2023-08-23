//! # Módulo dígitos
//! 
//! Proporciona las funciones para convertir caracteres en índices y viceversa
//! Los caracteres están en el rango 0..0a..z, y los índices entre 0..35
//! 
//! ## Ejemplos

// Un problema con los doctest, que no se ejecutan aquí
// Por lo visto, solo se ejecutan si están en lib.rs, ya que se supone
// que esta documentación ha de estar en una librería, para linkarla con
// los otros binarios. Podría montarlo así, pero ahora no me apetece
// Lo dejo como está (igual a cambiar otra vez)

pub mod digitos {

    const DIGITOS: &[(usize, char)] = &[(0, '0'), (1, '1'), (2, '2'), (3, '3'), (4, '4'), 
                                     (5, '5'), (6, '6'), (7, '7'), (8, '8'), (9, '9'),
                                (10, 'a'), (11, 'b'), (12, 'c'), (13, 'd'), (14, 'e'), 
                                (15, 'f'), (16, 'g'), (17, 'h'), (18, 'i'), (19, 'j'), 
                                (20, 'k'), (21, 'l'), (22, 'm'), (23, 'n'), (24, 'o'), 
                                (25, 'p'), (26, 'q'), (27, 'r'), (28, 's'), (29, 't'), 
                                (30, 'u'), (31, 'v'), (32, 'w'), (33, 'x'), (34, 'y'), 
                                (35, 'z'),];

    /// ```rust
    /// let n = 15;
    /// let c = digit::get_char(n);
    /// assert_eq!(c, 'f');
    /// ```
    /// 
    /// ```rust
    /// let c = 'z';
    /// let n = digit::get_num(c);
    /// assert_eq!(n, 35);
    /// ```
    /// 
    pub fn get_char(numero: usize) -> Option<char> {
        DIGITOS
        .iter()
        .find(|&&(n, _)| n == numero)
        .map(|&(_, c)| c)
    }
    
    pub fn get_num(caracter: char) -> Option<usize> {
        DIGITOS
        .iter()
        .find(|&&(_, c)| c == caracter)
        .map(|&(n, _)| n)
    }
}

#[cfg(test)]
mod tests {

    use super::digitos::{get_char, get_num};

    #[test]
    fn get_num_a() {
        let c = 'a';
        let n = get_num(c);
        assert_eq!(n, Some(10));
    }

    #[test]
    fn get_char_26() {
        let n = 26;
        let c = get_char(n);
        assert_eq!(c, Some('q'));
    }

    #[test]
    fn get_num_may_a_none() {
        let c = 'A';
        let n = get_num(c);
        assert_eq!(n, None);
    }

    #[test]
    fn get_char_40_none() {
        let n = 40;
        let c = get_char(n);
        assert_eq!(c, None);
    }

}