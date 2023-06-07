pub mod garden;
use std::collections::HashMap;

use front_of_house::hosting;
use garden::vegetables::Asparagus;

fn main() {
    let plant = Asparagus {};
    let rose = flowers::Rose { color: "red" };
    println!("I'm growing {:?}", plant);
    println!("And rose with {}, too", rose.color);

    let mut map = HashMap::new();
    map.insert("choi", 22);
    map.insert("lora", 20);

    for (key, value) in map {
        println!("{key}, {value}");
    }
}
use crate::garden::flowers;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    customer::eat_at_restaurant();
}
use crate::front_of_house::hosting::add_to_waitlist;

mod customer {
    pub fn eat_at_restaurant() {
        super::add_to_waitlist();
    }
}
