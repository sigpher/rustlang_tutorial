use ch06::Coin;
use rand::Rng;

fn main() {
    let coin = Coin::Quarter(ch06::UsState::Alaska);
    println!("{:?}", coin);
    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    let mut count = 0;
    loop {
        count += 1;
        let dice_roll = dice_roll();
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }
        if count == 100 {
            break;
        }
    }
}

fn plus_one(x: Option<u32>) -> Option<u32> {
    if let Some(number) = x {
        Some(number)
    } else {
        None
    }
}

fn add_fancy_hat() {
    println!("add fancy hat")
}
fn remove_fancy_hat() {
    println!("remove fancy hat")
}
fn move_player(num_spaces: u8) {
    println!("Move {num_spaces} step(s)")
}

fn dice_roll() -> u8 {
    rand::thread_rng().gen_range(1..10u8)
}
