fn main() {
    let mensaje = "hola mundo".to_string(); // this is a string literal that we can convert to a String type using the to_string() method

    let y = toma_propiedad(mensaje);
    println!("El mensaje es: {}", y);
    //println!("El mensaje es: {}", mensaje); // this will cause a compile-time error because the ownership of the string has been moved to the toma_propiedad function

    let mensaje2 = "hola mundo".to_string();
    let (mensaje_calculado, longitud) = calcular_cadena(mensaje2);
    println!(
        "El mensaje calculado es: {}, y su longitud es: {}",
        mensaje_calculado, longitud
    );
    //println!("El mensaje calculado es: {}, y su longitud es:
}

fn toma_propiedad(cadena: String) -> String {
    println!("La cadena es: {}", cadena);
    cadena
}

fn calcular_cadena(cadena: String) -> (String, usize) {
    let size = cadena.len();
    (cadena, size)
}
