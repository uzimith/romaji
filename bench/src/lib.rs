#![feature(test)]
extern crate test;
extern crate romaji;

#[cfg(test)]
mod tests {
    use std::fs;
    use test::Bencher;
    use romaji::RomajiExt;
    use std::io::prelude::*;

    #[bench]
    fn bench_neko_to_romaji(b: &mut Bencher) {
        let mut file = fs::File::open("./text/neko.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        b.iter(|| contents.to_romaji());
    }

    #[bench]
    fn bench_ningen_to_romaji(b: &mut Bencher) {
        let mut file = fs::File::open("./text/ningen.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        b.iter(|| contents.to_romaji());
    }

    #[bench]
    fn bench_to_hiragana(b: &mut Bencher) {
        let mut file = fs::File::open("./text/romaji.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        b.iter(|| contents.to_hiragana());
    }

    #[bench]
    fn bench_to_katakana(b: &mut Bencher) {
        let mut file = fs::File::open("./text/romaji.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        b.iter(|| contents.to_katakana());
    }
}
