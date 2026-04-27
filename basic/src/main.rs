fn main() {
    let mut value: Vec<i32> = Vec::new();

    value.push(32);
    value.push(33);

    for i in &value {
        println!("The value is: {}", i);
    }

    let mut count = 0;

    loop {
        if count >= value.len() {
            break;
        }
        println!("The value is: {}", value[count]);
        count += 1;
    }
    count = 0;
    while count < value.len() {
        println!("The value is: {}", value[count]);
        count += 1;
    }

    println!("The value is: {:?}", &value);
}
