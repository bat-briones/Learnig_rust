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

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    fn builder(name: String, age: u8) -> User {
        User { name, age }
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

    let is_adult: bool = my_user.is_adult();
    if is_adult {
        println!("{} is an adult", my_user.name);
    } else {
        println!("{} is not an adult", my_user.name);
    }

    let my_user2 = User::builder(String::from("Jane"), 25);
    my_user2.print_data("2024-06-01".to_string());
}
