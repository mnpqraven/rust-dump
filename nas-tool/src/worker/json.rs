use serde::Deserialize;
use serde::Serialize;
use serde_json::to_string;

#[derive(Serialize, Deserialize)]
struct user {
    name: String,
    pass: String,
    number: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_fn() {
        let t = user {
            name: String::from("test"),
            pass: String::from("12345"),
            number: 12,
        };
        let a = to_string(&t).unwrap();
        println!("{}", a)
    }
}
