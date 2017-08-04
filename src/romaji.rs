//! Romaji <-> Hira/Kana transliterator

#[derive(Debug)]
pub struct Romaji {
    inner: String
}

impl Romaji {
    pub fn new(s: &str) -> Romaji {
        Romaji {
            inner: s.to_string()
        }
    }

    pub fn hiragana(&self) -> &Romaji {
        self
    }

    pub fn katakana(&self) -> &Romaji {
        self
    }

    pub fn romaji(&self) -> &Romaji {
        self
    }

    pub fn normalize(&self) -> &Romaji {
        self
    }

    pub fn to_str(&self) -> &str {
        ""
    }

}
