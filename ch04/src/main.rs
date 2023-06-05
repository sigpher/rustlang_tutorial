fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2}");
    // println!("{s1}");

    let text = "how do you do";
    let first_word = first_word(text);
    println!("{first_word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}
