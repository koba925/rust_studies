#[allow(dead_code)]
pub fn run() {
    let c = '\0';
    println!("null char `{}`", c);
    println!("null char debug `{:?}`", c);

    let s = "\taa\nbb\0";
    println!("default `{}`", s);
    println!("escape_default `{}`", s.escape_default());
    println!("escape_debug `{}`", s.escape_debug());
    println!("debug `{:?}`", s);
}
