fn main() {
    let content = "123123123123123".repeat(1_000_000);
    loop {
        let dir = std::fs::read_dir("/mnt")
            .unwrap()
            .map(|p| p.unwrap().path())
            .collect::<Vec<_>>();
        let index = dir.len();
        println!("Write {index}");
        std::fs::write(format!("/mnt/file-{index}.txt"), content.as_bytes()).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
