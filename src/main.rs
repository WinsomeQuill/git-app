struct User {
    name: String,
    age: u16,
    course: u16,
}

impl User {
    fn new(name: String, age: u16, course: u16) -> Result<User, Box<dyn std::error::Error>> { // создаем структуру
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
    let user = User::new(String::from("Artem"), 19, 4).unwrap();
    user.about();
}
