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

    let mut new_value = vec![1, 2, 3, 4, 5];
    let my_pointer = &new_value[2];
    //new_value.push(6);
    println!("The value is: {}", &my_pointer);

    for i in &mut new_value {
        *i += 50;
    }
}
