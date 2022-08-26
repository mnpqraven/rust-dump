pub mod exclude {
    use std::fs::{File, self};
    use std::io;
    use std::io::Write;

    static TMP_FILE: &'static str = "/tmp/exclude.txt";

    // NOTE: threading ?
    fn default_excludes() -> Vec<String> {
        vec![
            String::from("node_modules"),
            String::from("bin"),
            String::from("debug"),
            String::from("obj"),
            String::from("dist"),
            String::from("target"),
        ]
    }

    fn create_tmp_exclude() {
        let mut file = File::create(&TMP_FILE).expect("can't create file, check perms");
        for line in default_excludes() {
            file.write(line.as_bytes()).unwrap();
            file.write(b"\n").unwrap();
        }
    }
    /// TODO: clean up fn
    fn clear_tmp_exclude() -> Result<(), io::Error> {
        fs::remove_file(&TMP_FILE).expect("can't delete file");
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::{create_tmp_exclude, clear_tmp_exclude};

        #[test]
        fn io_create() {
            create_tmp_exclude();
        }
        #[test]
        fn io_clear() {
            clear_tmp_exclude().unwrap();
        }
    }
}
