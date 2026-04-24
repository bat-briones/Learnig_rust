struct User {
    name: String,
    age: u8, // 0..=255 you cant have more than 255 years old
}

impl User {
    fn print_data(&self, date: String) {
        println!(
            "{} is {} years old and the date is {}",
            self.name, self.age, date
        );
    }
}

/*
methods: the methods are functions that are associated with a struct,
they are defined inside an impl block, and they can be called on an instance of the struct, for example:
*/

fn main() {
    let my_user = User {
        name: String::from("John"),
        age: 30,
    };
    my_user.print_data("skjda".to_string());
}
