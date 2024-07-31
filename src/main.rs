fn main() {
    loop {
        let Ok(dir) = std::fs::read_dir("/mnt") else {
            continue
        };
        let mut count = 0;

        for entry in dir {
            let Ok(path) = entry.map(|p| p.path()) else {
                continue
            };
            let _ = std::fs::remove_file(&path);
            let _ = std::fs::remove_dir(path);
            count += 1;
        }
        println!("Found {count}");
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
