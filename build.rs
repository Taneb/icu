extern crate gcc;

fn main() {
    let mut c = gcc::Config::new();
    c.file("src/icu.c");
    c.compile("libicucolrs.a");
}
