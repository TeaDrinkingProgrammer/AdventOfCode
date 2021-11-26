fn main() {
    say_hello();
    say_my_name();
    class_example();
}

fn say_hello() {
    //isize/usize is the integer length of the platform e.g 64 bit processor gives 64 int
    let coding: bool = true;
    //not mutable, it runs the ifstatement
    let mood = if coding { "happy" } else { "sad" };
    println!("Hello World {}", mood);
}

fn say_my_name() {
    //String struct, equivelent to StringBuilder
    let mut full_name = String::from("John");
    full_name.push_str(" Doe");
    println!("Hello World {}", full_name);
}

pub struct Contact {
    full_name: String,
    since: u16,
}
impl Contact {
    pub fn from_info(full_name: String, since: u16) -> Contact {
        return Contact { full_name, since };
    }
    fn info(&self) -> String {
        //no semicolon, last statement will return automatically
        format!("{} since: {}", self.full_name, self.since)
    }
}
fn class_example() {
    let c1 = Contact::from_info("John Doe".to_string(), 2005);
    println!("Hello {} since: {}", c1.full_name, c1.since);
    println!("Hello {}", c1.info());
    println!("Hello {}", c1.card());
}

trait BusinessCard {
    fn card(&self) -> String;
}
impl BusinessCard for Contact {
    fn card(&self) -> String {
        format!("Business Card: {}", self.full_name)
    }
}
