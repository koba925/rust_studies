#[allow(dead_code)]
pub fn run() {
    let s1 = Some(1);
    let n: Option<i32> = None;

    // Error!
    //
    // if let Some(i) = s1
    //     && i == 1
    // {
    //     println!("some 1");
    // } else {
    //     println!("not some 1");
    // }

    match s1 {
        Some(i) if i == 1 => println!("Some 1"),
        _ => println!("not Some 1"),
    }

    match n {
        Some(i) if i == 1 => println!("Some 1"),
        _ => println!("not Some 1"),
    }
}
