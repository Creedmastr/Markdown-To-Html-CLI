fn main() {
    mdtohtml::load(std::fs::File::open("./test.md").unwrap(), true, true);
}