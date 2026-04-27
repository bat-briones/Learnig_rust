fn main() {
    let mensaje = "hola mundo".to_string();
    let slice = &mensaje[5..].to_string(); // "hola "
    println!("Mensaje completo: {}", &slice);

    primera_palabra(&mensaje);
    primera_palabra(&slice);
}

/*
!IMPORTANT: La RAM al Desnudo (String vs Bytes)

Texto:   "H  o  l  a     🦀"
chars():  1  2  3  4  5   6    (La CPU trabaja para agruparlos)
bytes(): [72,111,108,97, 32, 240,159,166,128] (Lo que lee la CPU realmente)

- ¿Por qué as_bytes() encuentra el espacio (' ') tan rápido?
  Porque simplemente busca el número 32 en el arreglo.
- ¿Qué pasa si busco el byte 32 dentro del cangrejo por error?
  UTF-8 es tan brillante que garantiza que ningún byte de un emoji
  jamás será igual a 32. Es matemáticamente seguro.
*/
fn primera_palabra(cadena: &String) {
    let bytes = cadena.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // b' ' es la forma de escribir el byte para el espacio
            println!("Primera palabra termina en el índice: {}", i);
            break;
        } else {
            println!("Byte actual: {}", item);
        }
    }
}
