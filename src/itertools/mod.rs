use itertools::{Itertools, MultiPeek};
use std::str::Chars;

struct Scanner<'a> {
    source: &'a str,
    chars: MultiPeek<Chars<'a>>,
    start: usize,
    current: usize,
}

impl<'a> Scanner<'a> {
    fn new(source: &str) -> Scanner {
        Scanner {
            source,
            chars: source.chars().multipeek(),
            start: 0,
            current: 0,
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
    fn advance(&mut self) -> char {
        let c = self.chars.next().expect("end of source");
        self.current += c.len_utf8();
        c
    }

    fn lexeme(&mut self) -> &str {
        &self.source[self.start..self.current]
    }

    // previous的なものはなさそう やるなら自分で覚えておく？
    // それなら自分でVecとインデックスを持ってればいいかなあ
    // Scannerではpreviousを使わないのでmultipeekにしてもいいけど
}

#[allow(dead_code)]
pub fn run() {
    let mut a = Scanner::new("var a = 10;");

    println!("{:?}", a.is_at_end());
    println!("{:?}", a.advance());
    println!("{:?}", a.advance());
    println!("{:?}", a.lexeme());
    println!("{:?}", a.peek());
}
