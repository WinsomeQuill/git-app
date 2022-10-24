struct User<'a> {
    name: &'a str
}

fn main() {
    let user = User { name: "Artem" };
}
