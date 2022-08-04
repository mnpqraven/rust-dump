use std::collections::HashMap;
struct User {
    id: i32,
    value: String
}
/// getting value without index
/// similar to vector
/// stored on the heap
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 30);

    scores.entry(String::from("Pink")).or_insert(40);

    println!("{:?}", scores);

    let text = "duplicate no yes maybe duplicate";
    let mut map: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // count is the value in the entry
        *count += 1;
        // duplicate 1
        // duplicate 1 no 1
        // duplicate 1 no 1 yes 1
        // duplicate 1 no 1 yes 1 maybe 1
        // duplicate 2 no 1 yes 1 maybe 1
    }
        println!("{:?}", map);
}
