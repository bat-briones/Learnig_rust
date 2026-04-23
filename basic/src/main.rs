/*
!IMPORTANT: Mentalidad de Structs en Rust

1. DATOS vs COMPORTAMIENTO:
   - struct: Define qué datos guardamos (La estructura física en RAM).
   - impl: Define qué funciones tiene (La lógica).

2. COMPOSICIÓN > HERENCIA:
   - No pienses: "Un Perro es un Animal".
   - Piensa: "Un Perro tiene DatosDeAnimal y se comporta como el Trait Caminar".

3. ZERO-COST:
   - Usar un struct no hace el programa más lento que usar variables sueltas.
     El compilador lo optimiza para que en el binario final sean solo offsets de memoria.
*/

struct User {
    name: String,
    age: u8, // 0..=255 you cant have more than 255 years old
}

fn main() {
    let mut user1 = User {
        name: "bat-briones".to_string(),
        age: 30,
    };
    println!("User: {}, Age: {}", user1.name, user1.age);
    user1.age += 1; // Cumpleaños feliz
    println!(
        "Happy Birthday! {} is now {} years old.",
        user1.name, user1.age
    );
    let user2 = User {
        age: 25,
        ..user1 // Clonamos user1 pero con un nuevo nombre
    };
    // like js you can use the spread operator but in rust is
    //
    println!("User: {}, Age: {}", user2.name, user2.age);

    // when you define one struct of mutable type rememenber
    // you can not choise some fields to mut, all the struct  is mutable

    // warinig remembrer the ownership of the struct
    // if you MOVE the struct now the original variable is not valid anymore

    //println!("User1: {}, Age: {}", user1.name, user1.age); // this is invalid because user1 was moved to user2
    // disclaimer:
    // ints, floats, bools, chars, tuples and arrays are Copy types
    // String, Vec, and custom structs are Move types
}
