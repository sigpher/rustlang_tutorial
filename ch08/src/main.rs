use std::collections::HashMap;

fn main() {
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    //
    // let third = &v[2];
    // println!("THe third element is {third}");
    // let hundredth = &v.get(99);
    // if let Some(number) = hundredth {
    //     println!("The third element is {number}");
    // } else {
    //     println!("There is no hundredth element");
    // }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("{first}");
    v.push(10);

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);

    let mut names: Vec<String> = vec!["will".into(), "lora".into(), "troy".into()];
    for name in &mut names {
        *name = format!("{name} choi");
    }
    println!("{:?}", names);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(2.0),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    println!("{:?}", row);

    let text = String::from("中国");

    for ch in text.chars() {
        println!("{ch}");
    }

    for byte in text.bytes() {
        println!("{byte}");
    }

    let mut score = HashMap::new();

    score.insert(String::from("Blue"), 10);
    score.insert(String::from("Red"), 50);
    score.insert(String::from("Yellow"), 20);
    score.insert(String::from("Pink"), 100);

    println!("{:?}", score);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
