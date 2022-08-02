/// A t-shirt company gives away a free shirt to someone on their mailing list
/// every so often. People on the mailing list can optionally add their favorite
/// color to their profile. If the person chosen to get the free shirt has their
/// favorite color in their profile, they get that color shirt. If the person
/// hasn’t specified a favorite color, they get the color that the company
/// currently has the most of.
#[derive(Debug, Clone, Copy, PartialEq)]
enum ShirtColor {
    Red,
    Blue,
}

struct Stock {
    shirts: Vec<ShirtColor>,
}
impl Stock {
    fn giveaway_match(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        match user_preference {
            Some(color) => color,
            _ => self.most_in_stock(),
        }
    }

    /// same functionality as the above function but using closure instead of
    /// normal match patterns.
    /// Is like a callback
    fn giveaway_closure(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_in_stock())
    }
    fn most_in_stock(&self) -> ShirtColor {
        let mut red = 0;
        let mut blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => red += 1,
                ShirtColor::Blue => blue += 1,
            }
        }
        if red > blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let add_hello = || println!("hello");

    add_hello();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_match() {
        let stock = Stock {
            shirts: vec![
                ShirtColor::Red,
                ShirtColor::Red,
                ShirtColor::Red,
                ShirtColor::Blue,
                ShirtColor::Blue,
            ],
        };
        let user1_likes = Some(ShirtColor::Blue);
        let user2_likes = Some(ShirtColor::Red);
        assert_eq!(stock.giveaway_match(None), ShirtColor::Red);
        assert_eq!(stock.giveaway_match(user1_likes), ShirtColor::Blue);
        assert_eq!(stock.giveaway_match(user2_likes), ShirtColor::Red);
    }
    #[test]
    fn color_closure() {
        let stock = Stock {
            shirts: vec![
                ShirtColor::Red,
                ShirtColor::Red,
                ShirtColor::Red,
                ShirtColor::Blue,
                ShirtColor::Blue,
            ],
        };
        let user1_likes = Some(ShirtColor::Blue);
        let user2_likes = Some(ShirtColor::Red);
        assert_eq!(stock.giveaway_closure(None), ShirtColor::Red);
        assert_eq!(stock.giveaway_closure(user1_likes), ShirtColor::Blue);
        assert_eq!(stock.giveaway_closure(user2_likes), ShirtColor::Red);
    }
}
