extern crate gcc;

fn main() {
    gcc::Config::new()
        .flag("-fPIC")
        .flag("-std=c99")
        .define("_GNU_SOURCE", None)
        .file("container.c")
        .compile("libcontainer.a")
}
