fn main() {
    let mut index = 0;
    loop {
        index += 1;
        println!("Write {index}");
        println!("{:?}", std::fs::read_dir("/mnt").unwrap().map(|p| p.unwrap().path()).collect::<Vec<_>>());
        std::fs::write(format!("/mnt/file-{index}.txt"), b"123123123123123").unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
