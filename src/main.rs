fn main() {
    loop {
        let dir = std::fs::read_dir("/mnt").unwrap().map(|p| p.unwrap().path()).collect::<Vec<_>>();
        let index = dir.len();
        println!("Write {index}");
        println!("{:?}", dir);
        std::fs::write(format!("/mnt/file-{index}.txt"), b"123123123123123").unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
