fn main() {
    let v = vec![1, 2, 10, 5];
    let largest_number = largest(&v);
    println!("{largest_number}");
    let v2 = vec!['a', 'z', 'c'];
    println!("{}", largest(&v2));

    let point = Point { x: 10.0, y: 5.0 };

    println!("{}", point.x());
}
fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
