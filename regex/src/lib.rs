#![feature(test)]
extern crate test;
extern crate regex;

use regex::Regex;

pub fn number_literal() {
    let text = r#"print("hoge")\n#      this is comment\na = 1\na += 2"#;
    let re = Regex::new(r"(0x[0-9a-fA-F]([0-9a-fA-F]|\.)*|\d(\d|\.)*)([uU][lL]{0,2}|([eE][-+]\d*)?[fFlL]*)")
            .unwrap();
    re.find(text);
}

pub fn number_literal_match() {
    let text = r#"print("hoge")\n#      this is comment\na = 1\na += 2"#;
    let re = Regex::new(r"^(0x[0-9a-fA-F]([0-9a-fA-F]|\.)*|\d(\d|\.)*)([uU][lL]{0,2}|([eE][-+]\d*)?[fFlL]*)")
            .unwrap();
    re.find(text);
}

pub fn number_literal_match2() {
    let text = r#"print("hoge")\n#      this is comment\na = 1\na += 2"#;
    let re = Regex::new(r"^((0x[0-9a-fA-F]([0-9a-fA-F]|\.)*|\d(\d|\.)*)([uU][lL]{0,2}|([eE][-+]\d*)?[fFlL]*))")
            .unwrap();
    re.find(text);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_num_search(b: &mut Bencher) {
        b.iter(|| number_literal());
    }

    #[bench]
    fn bench_num_match(b: &mut Bencher) {
        b.iter(|| number_literal_match());
    }

    #[bench]
    fn bench_num_match2(b: &mut Bencher) {
        b.iter(|| number_literal_match2());
    }
}
