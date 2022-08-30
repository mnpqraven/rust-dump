#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
struct IceWall {
    name: String,
}
struct Fireball {
    name: String,
}
// casting anything = a cast() method
pub trait Cast {
    fn cast(&self) {
        // anything inside this trait function will be the default
    }
}
// any spell would need to be casted
impl Cast for Fireball {}
impl Cast for IceWall {
    fn cast(&self) {
        // differen effects of casting if specified
    }
}
struct Spellbook {
    // INFO: any type that has the Cast trait
    // Box is a pointer to a value on the heap
    // dyn is a prefix that annotates that it's a trait object
    pub spells: Vec<Box<dyn Cast>>, // must point to a value that implement Cast
}

impl Spellbook {
    pub fn run(&self) {
        for spell in self.spells.iter() {
            spell.cast();
        }
    }
}

// INFO: trait bounds
struct Englishman {
    // speaks all 3 languages
    name: String,
}
struct German {
    // speaks german and english
    name: String,
}
struct Spanish {
    // speaks only spanish
    name: String,
}
pub trait English {}
pub trait Deutsch {}
pub trait Espanol {}
impl English for Englishman { }
impl Deutsch for Englishman { }
impl Espanol for Englishman { }
impl English for German { }
impl Deutsch for German { }
impl Espanol for Spanish { }

// <T: English> a trait bound, this function is only implemented for said
// generics with said traits
// (character: T): generic type for variable
pub fn speak_english<T: English>(character: T) -> String {
    String::from("yes")
}

fn main() {
    let _othi = Spanish {
        name: String::from("othi")
    };
    // this wouldn't compile
    // speak_english(_othi);
    let othi = Englishman {
        name: String::from("othi")
    };
    println!("{}", speak_english(othi));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playground() {
        main();
    }
}
