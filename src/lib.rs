use std::{
    collections::HashSet,
    fs::{self, File},
    io::{self, prelude::*},
    path::{MAIN_SEPARATOR_STR, PathBuf},
};

pub const DASH_PADDING: usize = 10 * 4;

pub struct SyncIt {
    src_dir: String,
    dest_dir: String,

    /// relative paths which are visted
    visited: HashSet<String>,
}

impl SyncIt{
    pub fn new(src_dir: &str, dest_dir: &str) -> Self {
        Self {
            src_dir: src_dir.to_string(),
            dest_dir: dest_dir.to_string(),

            visited: HashSet::new(),
        }
    }

    pub fn sync(&mut self) {
        let src_dir = self.src_dir.to_string();
        let dest_dir = self.dest_dir.to_string();

        self.one_directional_sync(&src_dir, &src_dir, &dest_dir);
        self.one_directional_sync(&dest_dir, &dest_dir, &src_dir);
    }

    /// One Way Sync
    ///
    fn one_directional_sync(&mut self, base_dir: &str, src: &str, dest: &str) {
        for entry in fs::read_dir(&src).unwrap().flatten() {
            let entry_path = entry.path();

            // relative path based on src_dir
            let relative_path = entry_path
                .to_str()
                .and_then(|path| path.strip_prefix(&base_dir))
                .and_then(|path| path.strip_prefix(MAIN_SEPARATOR_STR));

            if relative_path.is_none() {
                println!("-- Continue");
                continue;
            }

            // Caching for Dobule Check
            if entry_path.is_file() {
                self.visited.insert(relative_path.unwrap().to_string());
            }

            let dest_path = PathBuf::from(&dest);
            let dest_path = dest_path.join(relative_path.unwrap());

            // create
            if !dest_path.exists() {
                if entry_path.is_dir() {
                    let _ = fs::create_dir_all(dest_path);
                } else if entry_path.is_file() {
                    let _ = fs::copy(&entry_path, &dest_path);
                }

                continue;
            }

            // comparing
            if entry_path.is_dir() {
                self.one_directional_sync(
                    &base_dir,
                    &entry_path.to_str().unwrap(),
                    &dest_path.to_str().unwrap(),
                );
                continue;
            }

            self.file_comparision(&entry_path, &dest_path);
        }
    }

    fn file_comparision(&self, src: &PathBuf, dest: &PathBuf) {
        if self.is_equal_files(&src.to_str().unwrap(), &dest.to_str().unwrap()) {
            return;
        }

        let src_meta = fs::metadata(&src).unwrap(); // TODO: handle latter
        let dest_meta = fs::metadata(&dest).unwrap(); // TODO: handle latter

        let src_date = src_meta.modified().unwrap(); // TODO: handle
        let dest_date = dest_meta.modified().unwrap(); // TODO: handle

        println!("|{}", "-".repeat(DASH_PADDING));
        println!("| Src : {src:?}");
        println!("| Dest: {dest:?}",);
        println!(
            "| \t Size: {:>10} - {:>10}",
            src_meta.len(),
            dest_meta.len()
        );
        println!("| \t Date: {src_date:?} - {dest_date:?}");

        let input = get_input("| > What to keep Src (Y/N)? ").unwrap();

        match &input[..] {
            "y" | "Y" | "" => {
                let _ = fs::remove_file(&dest); // TODO: handle
                let _ = fs::copy(&src, &dest); // TODO: handle
            }
            "n" | "N" => {
                let _ = fs::remove_file(&src); // TODO: handle
                let _ = fs::copy(&dest, &src); // TODO: handle
            }
            _ => {

                // TODO: invalid input
            }
        }

        println!("{}", "-".repeat(DASH_PADDING));
    }

    fn is_equal_files(&self, a: &str, b: &str) -> bool {
        let mut file_a = File::open(&a).unwrap();
        let mut file_b = File::open(&b).unwrap();

        if file_a.metadata().unwrap().len() != file_b.metadata().unwrap().len() {
            return false;
        }

        let mut buffer_a = [0u8; 8192];
        let mut buffer_b = [0u8; 8192];

        loop {
            let n_a = file_a.read(&mut buffer_a).unwrap();
            let n_b = file_b.read(&mut buffer_b).unwrap();

            if n_a != n_b {
                return false;
            }

            if n_a == 0 {
                break;
            }

            if buffer_a[..n_a] != buffer_b[..n_b] {
                return false;
            }
        }

        true
    }
}

fn get_input(question: &str) -> io::Result<String> {
    print!("{question}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}
