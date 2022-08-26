pub mod tmp_worker {
    use std::fs::{self, File};
    use std::io;
    use std::io::Write;

    #[allow(dead_code)]
    static TMP_EXCLUDE: &'static str = "/tmp/exclude";

    const DEFAULT_EXCLUDES: [&'static str; 6] =
        ["node_modules", "bin", "debug", "obj", "dist", "target"];

    /// Creates the exclude file in /tmp, containing folders to exclude
    /// (binanies in repos, etc.)
    fn create_tmp_exclude() -> Result<(), io::Error> {
        let mut file = File::create(&TMP_EXCLUDE).expect("can't create file, check perms");
        for line in DEFAULT_EXCLUDES {
            file.write(line.as_bytes()).unwrap();
            file.write(b"\n").unwrap();
        }
        Ok(())
    }

    /// Deletes the exclude file in /tmp
    fn clear_tmp_exclude() -> Result<(), io::Error> {
        fs::remove_file(&TMP_EXCLUDE).expect("can't delete file, check perms");
        Ok(())
    }
}
