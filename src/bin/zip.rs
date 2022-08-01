fn main() {
    let filename = std::env::args_os().skip(1).next().unwrap();
    let handle = File::open(filename);
    
}