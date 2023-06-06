#[derive(Debug)]
pub struct User<'a> {
    pub username: &'a str,
    pub age: u8,
}

impl<'a> User<'a> {
    pub fn new(username: &'a str, age: u8) -> Self {
        Self { username, age }
    }
}

#[derive(Debug)]
pub struct Color(pub i32, pub i32, pub i32);
#[derive(Debug)]
pub struct Point(pub i32, pub i32, pub i32);

#[derive(Debug)]
pub struct Rectangle<T>
where
    T: PartialOrd + Copy + Clone + std::ops::Mul<Output = T>,
{
    pub width: T,
    pub height: T,
}

pub trait Area<T>
where
    T: PartialOrd + Copy + Clone + std::ops::Mul<Output = T>,
{
    fn area(&self) -> T;
}

impl<T> Area<T> for Rectangle<T>
where
    T: PartialOrd + Copy + Clone + std::ops::Mul<Output = T>,
{
    fn area(&self) -> T {
        self.width * self.height
    }
}

impl<T> Rectangle<T>
where
    T: PartialOrd + Copy + Clone + std::ops::Mul<Output = T>,
{
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}
