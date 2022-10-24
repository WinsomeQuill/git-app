struct User<'a> {
    name: &'a str,
    age: u16,
}

impl <'a>User<'a> {
    fn new(name: &str, age: u16) -> Result<User, Box<dyn std::error::Error>> { // создаем структуру
        let user = User { name, age };
        Ok(user)
    }

    fn about(&self) { // выводим формат информацию о структуре
        let formating = format!("My name: {}. Age: {}", &self.name, self.age);
        println!("{}", formating);
    }
}

fn main() {
    let user = User::new("Artem", 19).unwrap();
    user.about();
}
