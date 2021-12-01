use crate::lesson3::ownership;
use core::cmp::Ordering;
use std::cmp::Ord;
mod lesson3;
fn main() {
    say_hello();
    say_my_name();
    class_example();
    arrays();
    tuples();
    vectors();
    enums();
    generics();
    option();
    result();
    ownership();
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

//Array
fn arrays() {
    let arr1: [u64; 4] = [1, 2, 3, 4];
    let arr2: [Foo; 4] = [Foo {}; 4];
    print!("item: {}", arr1[3]);
}

#[derive(Clone, Copy, Debug)]
struct Foo {}

//Tuple
fn tuples() {
    let tuple1 = (123, true, Foo {});
    let tuple2 = Bar(123, true, Foo {});
    println!("item {}", tuple1.1);
}

struct Bar(u64, bool, Foo);

//Vec ~= ArrayList
fn vectors() {
    let mut arr3: Vec<u64> = Vec::new();
    arr3.push(123);
    println!("item {}", arr3[0]);
    println!("item {:?}", arr3.get(0)); //Option, does not crash when out of bounds, but returns None
}

//enum
fn enums() {
    let activity = Activity::Skiing("Jodel".to_string());
    let message = match activity {
        Activity::Sleeping(hours) if hours > 8 => format!("Wake up! {}", hours),
        Activity::Sleeping(_) => format!("Shhh"),
        Activity::Skiing(resort) => format!("Awesome! {}", resort),
    };
    println!("The message is: {}", message)
}
enum Activity {
    Sleeping(u8),
    Skiing(String),
}

//generics
fn generics() {
    //Creates seperate containers with a hardcoded type
    let c1 = Container { item: 123 };
}
struct Container<T: Ord> {
    item: T,
}

impl<T: Ord> Container<T> {
    fn compare_item(&self, other_item: &T) -> Ordering {
        self.item.cmp(other_item)
    }
}

//Option
//No Null type
fn option() {
    //Option has values of Some and None
    let foo: Option<Foo> = Option::None;
    println!("foo: {:?}", foo)
}
//This is the same as option
enum Nullable<T> {
    Null,
    Value(T),
}

//Result
fn floor_divide(num: f32, by: f32) -> Result<i32, String> {
    if by == 0. {
        //panic!("cannot divide by 0");
        //You cannot escape this
        Err("Cannot divide by 0".to_string())
    } else {
        Ok((num / by).floor() as i32)
    }
}
fn result() {
    let result = floor_divide(10., 0.);
    println!("results{:?}", result);
    //catch error by match
}
