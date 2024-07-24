fn main() {
    let content = "123123123123123".repeat(10_000_000);
    loop {
        let Ok(dir) = std::fs::read_dir("/mnt") else {
            continue
        };
        let dir = dir.map(|p| p.unwrap().path()).collect::<Vec<_>>();
        let index = dir.len();
        println!("Write {index}");
        if let Err(err) = std::fs::write(format!("/mnt/file-{index}.txt"), content.as_bytes()) {
            println!("Error: {err}");
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
