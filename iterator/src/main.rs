#[allow(dead_code)]
struct Progress {
    length: i32,
    fill: char,
    delims: (char, char),
}
impl IntoIterator for Progress {
    type Item = i32;
    type IntoIter = ProgressIterator; // -----v

    fn into_iter(self) -> Self::IntoIter {
        ProgressIterator {
            // ------v
            length: self.length,
        }
    }
}
struct ProgressIterator {
    // -------^---------^----v
    length: i32,
}
impl Iterator for ProgressIterator {
    // -----------^
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = 1;
        for _ in 0..self.length {
            result *= 2;
        }
        Some(result)
    }
}

fn main() {
    // (b'a' + rand::thread_rng().gen_range(0..=(b'z' - b'a')))

    let mut iter = vec!["1", "2", "3"].into_iter();
    while let Some(e) = iter.next() {
        println!("{}", e);
    }

    let vs = vec![1, 2, 3];
    for _v in vs.iter() {
        // borrows vs, & to v
    }
    for _v in &vs {
        // equiv to vs.iter()
    }
    for _v in vs {
        // consumes vs, owned v
    }
}

struct Counter {
    count: usize,
    bound: usize,
}

impl Counter {
    fn new(bound: usize) -> Self {
        Self { count: 0, bound }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < self.bound {
            Some(self.count)
        } else {
            None
        }
    }
}
// TODO: collection of key value pairs, e.g json
#[derive(Debug)]
struct UserDatabase {
    user_collection: Vec<User>,
}
impl UserDatabase {
    fn new() -> Self {
        Self {
            user_collection: Vec::new(),
        }
    }
    fn add(&mut self, element: User) {
        self.user_collection.push(element);
    }
}
#[derive(Debug)]

struct User {
    id: i32,
    username: String,
    age: i32,
}
impl User {
    // NOTE: starts id at 0 ??
    fn new(username: String, age: i32) -> Self {
        User {
            id: 0,
            username,
            age,
        }
    }
}
impl Iterator for User {
    // is this i32 for the interating index or for the whole struct item ?
    // probably the former
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.id += 1;
        Some(self.id)
    }
}
impl IntoIterator for UserDatabase {
    // type for runner
    type Item = User;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.user_collection.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_empty() {
        let mut count_iter = Counter::new(20);
        // for index in count_iter.into_iter() {
        //     println!("{}", index)
        // }
        assert_eq!(count_iter.next(), Some(1));
    }

    #[test]
    fn user_then_vec() {
        //all individual, not iterable
        let _user1 = User::new("othi".to_string(), 29);
        let _user2 = User::new("aarst".to_string(), 12);
        let _user3 = User::new("yuql".to_string(), 38);
        let _user4 = User::new("z1vm".to_string(), 28);
        let _user5 = User::new("ausw".to_string(), 73);
        let _user6 = User::new("su28".to_string(), 44);

        // vec, should be iterable
        let list = vec![_user1, _user2, _user3, _user4, _user5, _user6];
        let mut iter = list.iter();
        println!("{:?}", iter.next().unwrap());
        println!("{:?}", iter.next().unwrap());
        println!("{:?}", iter.next().unwrap());
        println!("{:?}", iter.next().unwrap());
        println!("{:?}", iter.next().unwrap());
    }

    #[test]
    fn vec_then_user() {
        //all individual, not iterable
        let _user1 = User::new("1111".to_string(), 29);
        let _user2 = User::new("2222".to_string(), 12);
        let _user3 = User::new("3333".to_string(), 38);
        let _user4 = User::new("4444".to_string(), 28);
        let _user5 = User::new("5555".to_string(), 73);
        let _user6 = User::new("6666".to_string(), 44);

        // vec, should be iterable
        let list = vec![_user1, _user2, _user3, _user4, _user5, _user6];

        let mut collection = UserDatabase {
            user_collection: list,
        };
        collection.add(User::new("text".to_uppercase(), 22));

        let mut iter = collection.into_iter();
        // TODO: see ref mut
        while let Some(ref mut x) = iter.next() {
            println!("{:?}", x);
        }
        // println!("{:?}", collection.into_iter().next());
    }

    #[test]
    fn collection_edit_by_instantiating() {
        //all individual, not iterable
        let user1 = User::new("1111".to_string(), 29);
        let user2 = User::new("2222".to_string(), 12);
        let user3 = User::new("3333".to_string(), 38);
        let user4 = User::new("4444".to_string(), 28);
        let user5 = User::new("5555".to_string(), 73);
        let user6 = User::new("6666".to_string(), 44);
        // vec, should be iterable
        let list = vec![user1, user2, user3, user4, user5];

        // NOTE: this works
        let mut collection1 = UserDatabase {
            user_collection: list,
        };
        collection1.add(user6);
        let mut iter1 = collection1.into_iter();
        while let Some(ref mut x) = iter1.next() {
            println!("{:?}", x);
        }

    }

    #[test]
    fn collection_edit_by_new() {
        let user1 = User::new("1111".to_string(), 29);
        let user2 = User::new("2222".to_string(), 12);
        let user3 = User::new("3333".to_string(), 38);
        let user4 = User::new("4444".to_string(), 28);
        let user5 = User::new("5555".to_string(), 73);
        let user6 = User::new("6666".to_string(), 44);

        let list = vec![user1, user2, user3, user4, user5];
        let mut collection2 = UserDatabase::new();
        for person in list {
          collection2.add(person);
        }
        collection2.add(user6);

        let mut iter2 = collection2.into_iter();
        while let Some(ref mut x) = iter2.next() {
            println!("{:?}", x);
        }

    }
}
