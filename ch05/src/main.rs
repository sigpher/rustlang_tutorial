use ch05::{Area, Color, Point, Rectangle, User};

fn main() {
    let choi = User::new("choi", 22);
    println!("{:?}", choi);
    let lora = User {
        username: "lora",
        ..choi
    };
    println!("{:?}", lora);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", black);
    println!("{:?}", origin);

    let white = Color(255, 255, 255);
    println!("{:?}", white);

    let rect1 = Rectangle::new(10, 20);
    println!("{:?}", rect1);
    println!("{:?}", rect1.area());
}
