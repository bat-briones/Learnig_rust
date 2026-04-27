enum Tiempo {
    Segundo,
    Minuto,
    Hora,
    Dia,
}

fn main() {
    let tiempo = Tiempo::Hora;
    let segundos = valor_en_segundos(tiempo);
    println!("El valor en segundos es: {}", segundos);

    let cinco = Some(5);
    let seis = incrementar_tiempo(cinco);
    println!("El valor incrementado es: {:?}", seis);
}

fn valor_en_segundos(tiempo: Tiempo) -> u32 {
    match tiempo {
        Tiempo::Dia => 24 * 60 * 60,
        Tiempo::Hora => 60 * 60,
        Tiempo::Minuto => 60,
        Tiempo::Segundo => 1,
    }
}

fn incrementar_tiempo(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(valor) => Some(valor + 1),
    }
}
