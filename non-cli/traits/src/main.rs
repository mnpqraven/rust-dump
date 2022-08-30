use std::cmp::PartialOrd;

// <T> generic type constraint
// [T] vector of generic
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub struct Point<T > {
    x: T, y: T
}

// needs to specify generics
impl<T> Point<T> {
    fn x(&self) -> &T{
        &self.x
    }
}

// just for specific types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float: Point<f32> = Point { x: 3.12, y: 6.32 };

    println!("{}", float.distance_from_origin());
}
