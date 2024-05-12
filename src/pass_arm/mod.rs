// パターンマッチの左辺を関数に渡すことはできるか？

#[derive(PartialEq)]
enum ABC {
    A,
    B,
    C,
}

// fn match_fn(arm: ABC) -> i32 {
//     let b = ABC::B;
//     match b {
//         arm => 3,
//         _ => 4,
//     }
// }

fn match_fn(val: ABC, matches: Vec<ABC>) -> i32 {
    if matches.contains(&val) {
        1
    } else {
        0
    }
    // for m in matches {
    //     if m == val {
    //         return 1;
    //     }
    // }
    // 0
}

#[allow(dead_code)]
pub fn run() {
    assert_eq!(
        match ABC::A {
            ABC::A | ABC::B => 1,
            _ => 0,
        },
        1
    );
    assert_eq!(match_fn(ABC::A, vec![ABC::A, ABC::B]), 1);
    assert_eq!(match_fn(ABC::B, vec![ABC::A, ABC::B]), 1);
    assert_eq!(match_fn(ABC::C, vec![ABC::A, ABC::B]), 0);
}
