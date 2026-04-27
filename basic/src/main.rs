fn main() {
    use std::collections::HashMap;

    let mut my_map = HashMap::new();
    my_map.insert("one", 1);

    let data = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let values = vec![1, 2, 3];
    let my_point = "one".to_string();

    let points: HashMap<_, _> = data.into_iter().zip(values.into_iter()).collect();
    println!("The value is: {:?}", &points);
    let better = points.get(&my_point).unwrap();
    println!("The value is: {:?}", &better);

    let name = String::from("Alice");

    let name_value = String::from("Alice2");

    let mut my_map2 = HashMap::new();
    my_map2.insert(name, name_value);
}
