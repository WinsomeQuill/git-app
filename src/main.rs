struct User<'a> {
    name: &'a str,
    age: u16,
    course: u16,
}

impl <'a>User<'a> {
    fn new(name: &str, age: u16, course: u16) -> Result<User, Box<dyn std::error::Error>> { // создаем структуру
        let user = User { name, age, course };
        Ok(user)
    }

    fn about(&self) { // выводим формат информацию о структуре
        let formating = format!("My name: {}. Age: {}. Course of study: {}.",
            &self.name, self.age, self.course);
        println!("{}", formating);
    }
}

fn main() {
    let user = User::new("Artem", 19, 4).unwrap();
    user.about();
}
