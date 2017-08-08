//! Romaji <-> Hira/Kana transliterator

use std::ascii::AsciiExt;

use convert::romaji_to_kana;
use convert::kana_to_romaji;

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
        self
    }

    pub fn katakana(&mut self) -> &mut Romaji {
        self.inner = self.inner
            .clone()
            .into_iter()
            .map(|x| romaji_to_kana(&x).to_string())
            .collect::<Vec<String>>();
        self
    }

    pub fn romaji(&mut self) -> &mut Romaji {
        self.inner = self.inner
            .clone()
            .into_iter()
            .map(|x| kana_to_romaji(&x).to_string())
            .collect::<Vec<String>>();
        self
    }

    pub fn to_string(&self) -> String {
        self.inner.join("")
    }

    fn split(input: &str) -> Vec<String> {
        let mut chars = input
            .to_ascii_lowercase()
            .chars()
            .into_iter()
            .map(|x| x.to_string())
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
                x if x.is_ascii() => {
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
        vec!["今", "日", "も", "shi", "na", "i", "to", "ne"],
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
}
