//! Romaji <-> Hira/Kana transliterator
use std::ascii::AsciiExt;
use convert;
use rule::is_katakana;
use rule::is_youon;
use rule::is_hatsuon;

#[derive(Debug)]
pub struct Romaji {
    inner: Vec<String>
}

impl Romaji {
    pub fn new(s: &str) -> Romaji {
        Romaji {
            inner: Romaji::split(s)
        }
    }

    pub fn hiragana(&mut self) -> &mut Romaji {
        self.inner = self.inner
            .iter()
            .cloned()
            .map(|x| convert::romaji_to_katakana(x))
            .map(|x| convert::katakana_to_hiragana(x))
            .collect::<Vec<String>>();
        self
    }

    pub fn katakana(&mut self) -> &mut Romaji {
        self.inner = self.inner
            .iter()
            .cloned()
            .map(|x| convert::romaji_to_katakana(x))
            .collect::<Vec<String>>();
        self
    }

    pub fn romaji(&mut self) -> &mut Romaji {
        self.inner = self.inner
            .iter()
            .cloned()
            .map(|x| convert::hiragana_to_katakana(x))
            .map(|x| convert::katakana_to_romaji(x))
            .collect::<Vec<String>>();
        self
    }

    pub fn to_string(&self) -> String {
        self.inner.join("_")
    }

    fn split(input: &str) -> Vec<String> {
        let mut chars = input
            .to_ascii_lowercase()
            .chars()
            .map(|x| x.to_string())
            .map(|x| convert::hiragana_to_katakana(x))
            .rev()
            .collect::<Vec<String>>();

        let mut res = vec!();
        let mut buffer = "".to_string();
        while let Some(char) = chars.pop() {
            match char.as_ref() {
                "a" | "i" | "u" | "e" | "o" => {
                    res.push(buffer + &char);
                    buffer = "".to_string()
                }
                _ if buffer == "n" || buffer == "m" => {
                    // if a letter after n, m is not a kind of vowel,
                    res.push(buffer);
                    buffer = char.clone()
                },
                x if x.is_ascii() && buffer.is_ascii() => {
                    buffer += &char
                },
                x if x.is_ascii() && is_katakana(&buffer) => {
                    res.push(buffer);
                    buffer = char.to_string()
                },
                x if is_katakana(x) && is_katakana(&buffer) => {
                    res.push(buffer);
                    buffer = char.to_string()
                },
                x if is_hatsuon(x) => {
                    res.push(buffer);
                    buffer = char.to_string()
                },
                x if is_youon(x) && is_katakana(&buffer) => {
                    res.push(buffer + &char);
                    buffer = "".to_string()
                },
                x if is_katakana(x) && is_hatsuon(&buffer) => {
                    res.push(buffer + &char);
                    buffer = "".to_string()
                },
                x if is_katakana(x) => {
                    buffer += &char
                },
                _ => {
                    res.push(buffer + &char);
                    buffer = "".to_string()
                }
            }
        }
        if buffer != "" {
            res.push(buffer);
        }
        res
    }
}

#[test]
fn test_split() {
    assert_eq!(
        vec!["kyo", "u", "mo", "shi", "na", "i", "to", "ne"],
        Romaji::split("kyoumoshinaitone")
    );
    assert_eq!(
        vec!["今", "日", "モ", "shi", "na", "i", "to", "ne"],
        Romaji::split("今日もshinaitone")
    );
    assert_eq!(
        vec!["su", "shi", "no", "ta", "be", "ta", "s"],
        Romaji::split("SushiNoTabetas")
    );
    assert_eq!(
        vec!["shi", "m", "ba", "shi"],
        Romaji::split("shimbashi")
    );
    assert_eq!(
        vec!["ki", "n", "ka", "ku", "ji"],
        Romaji::split("kinkakuji")
    );
    assert_eq!(
        vec!["to", "tto", "ri"],
        Romaji::split("tottori")
    );
    assert_eq!(
        vec!["イ", "イ", "ハ", "ナ", "シ", "ダ", "ナ", "ー"],
        Romaji::split("イイハナシダナー")
    );
    assert_eq!(
        vec!["キョ", "ウ", "モ", "シ", "ナ", "イ", "ト", "ネ"],
        Romaji::split("キョウモシナイトネ")
    );
    assert_eq!(
        vec!["イ", "ッカ", "ク", "ジュ", "ウ"],
        Romaji::split("イッカクジュウ")
    );
}
