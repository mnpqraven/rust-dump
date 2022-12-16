enum Browser {
    Firefox,
    Chrome,
}

fn browser_open(selection: Browser, link: &str) -> Result<String, &'static str> {
    let mut br = String::new();
    match selection {
        Browser::Firefox => br.push_str("firefox"),
        Browser::Chrome => br.push_str("google-chrome-stable"),
    }
    if !link.is_empty() {
        let f = format!("{} {}", br, link);
        Ok(f)
    } else {
        Err("empty link, aborting")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_link() {
        let res = browser_open(Browser::Firefox, "192.168.1.14:5000").unwrap();
        assert_eq!(res, "firefox 192.168.1.14:5000");
    }

    #[test]
    #[should_panic]
    fn no_link() {
        let _res = browser_open(Browser::Chrome, "").unwrap();
    }
}
