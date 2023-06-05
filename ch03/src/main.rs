fn main() {
    let heart_eyed_cat = '😻';

    println!("{heart_eyed_cat}");
    let tup = ('a', 100);
    let (x, y) = tup;
    println!("x: {x}, y:{y}");

    let v = [1, 2, 3, 5, 10];
    for number in v {
        println!("{number}");
    }
    let num = plus_one(100);
    println!("{num}");

    if num > 100 {
        println!("Over 100");
    } else {
        println!("Less than 100");
    }
}

fn plus_one(x: i32) -> i32 {
    // number plus one
    x + 1
}
