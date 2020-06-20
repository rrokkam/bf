fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("args should have a filename as the first argument");
    let program = std::fs::read_to_string(filename)
        .expect("first argument in args should point to a valid utf8-encoded file");
    print!("{}", program);
}
