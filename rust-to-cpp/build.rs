extern crate gcc;

fn main() {
    gcc::Build::new()
        .file("src/xyz/triple.cpp")
        .include("src") // NOTE: inclusion basedir(s) can be everywhere
        .cpp(true)
        .compile("libtriple_WHATEVER.a");  // NOTE: name of the created library does not matter
}
