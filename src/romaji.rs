use map;
use parse::parse;

#[derive(Debug, PartialEq)]
pub enum Japanase {
    /// romaji is a consonant(k, s, sh, etc.) and a vowel(a, i, u, e, o)
    ///
    /// include `-`
    Romaji(String),
    /// Hiragana
    Hiragana(String),
    /// Katakana
    ///
    /// include `ー`
    Katakana(String),
    HatsuonRomaji(String, Option<String>),
    HatsuonHiragana(String, Option<String>),
    HatsuonKatakana(String, Option<String>),
    Other(String),
}

impl Japanase {
    pub fn romaji<T: Into<String>>(s: T) -> Self {
        Japanase::Romaji(s.into())
    }

    pub fn hiragana<T: Into<String>>(s: T) -> Self {
        Japanase::Hiragana(s.into())
    }

    pub fn katakana<T: Into<String>>(s: T) -> Self {
        Japanase::Katakana(s.into())
    }

    pub fn hatsuon_romaji<T: Into<String>>(s: T) -> Self {
        Japanase::HatsuonRomaji(s.into(), None)
    }

    pub fn hatsuon_romaji_some<T: Into<String>, U: Into<String>>(s1: T, s2: U) -> Self {
        Japanase::HatsuonRomaji(s1.into(), Some(s2.into()))
    }

    pub fn hatsuon_hiragana<T: Into<String>>(s: T) -> Self {
        Japanase::HatsuonHiragana(s.into(), None)
    }

    pub fn hatsuon_hiragana_some<T: Into<String>, U: Into<String>>(s1: T, s2: U) -> Self {
        Japanase::HatsuonHiragana(s1.into(), Some(s2.into()))
    }

    pub fn hatsuon_katakana<T: Into<String>>(s: T) -> Self {
        Japanase::HatsuonKatakana(s.into(), None)
    }

    pub fn hatsuon_katakana_some<T: Into<String>, U: Into<String>>(s1: T, s2: U) -> Self {
        Japanase::HatsuonKatakana(s1.into(), Some(s2.into()))
    }

    pub fn other<T: Into<String>>(s: T) -> Self {
        Japanase::Other(s.into())
    }

    pub fn to_romaji(&self) -> Option<Self> {
        debug!("to_romaji: {:?}", self);
        match self {
            &Japanase::Hiragana(ref s) => {
                map::HIRAGANA_ROMAJI
                    .get(s.as_ref() as &str)
                    .map(|&s: &&str| s.to_string())
                    .map(Japanase::Romaji)
            }
            &Japanase::Katakana(ref s) => {
                map::KATAKANA_ROMAJI
                    .get(s.as_ref() as &str)
                    .map(|s: &&str| s.to_string())
                    .map(Japanase::Romaji)
            }
            &Japanase::HatsuonHiragana(ref s1, None) => {
                Some(Japanase::HatsuonRomaji(
                    "xtsu".repeat(s1.chars().count()),
                    None,
                ))
            }
            &Japanase::HatsuonHiragana(ref s1, Some(ref s2)) => {
                map::HIRAGANA_ROMAJI
                    .get(s2.as_ref() as &str)
                    .map(|&s: &&str| s.to_string())
                    .and_then(|str: String| {
                        str.chars().nth(0).map(|c: char| {
                            Japanase::HatsuonRomaji(
                                c.to_string().repeat(s1.chars().count()),
                                Some(str),
                            )
                        })
                    })
            }
            &Japanase::HatsuonKatakana(ref s1, None) => {
                Some(Japanase::HatsuonRomaji(
                    "xtsu".repeat(s1.chars().count()),
                    None,
                ))
            }
            &Japanase::HatsuonKatakana(ref s1, Some(ref s2)) => {
                map::KATAKANA_ROMAJI
                    .get(s2.as_ref() as &str)
                    .map(|&s: &&str| s.to_string())
                    .and_then(|str: String| {
                        str.chars().nth(0).map(|c: char| {
                            Japanase::HatsuonRomaji(
                                c.to_string().repeat(s1.chars().count()),
                                Some(str),
                            )
                        })
                    })
            }
            _ => None,
        }
    }

    pub fn to_hiragana(&self) -> Option<Self> {
        debug!("to_hiragana: {:?}", self);
        match self {
            &Japanase::Romaji(ref s) => {
                map::ROMAJI_HIRAGANA
                    .get(s.as_ref() as &str)
                    .map(|s: &&str| s.to_string())
                    .map(Japanase::Hiragana)
            }
            &Japanase::Katakana(ref s) => {
                map::KATAKANA_HIRAGANA
                    .get(s.as_ref() as &str)
                    .map(|s: &&str| s.to_string())
                    .map(Japanase::Hiragana)
            }
            &Japanase::HatsuonRomaji(ref s1, None) => {
                Some(Japanase::HatsuonHiragana(
                    "っ".repeat(s1.chars().count()),
                    None,
                ))
            }
            &Japanase::HatsuonRomaji(ref s1, Some(ref s2)) => {
                map::ROMAJI_HIRAGANA
                    .get(s2.as_ref() as &str)
                    .map(|&s: &&str| s.to_string())
                    .map(|str: String| {
                        Japanase::HatsuonHiragana("っ".repeat(s1.chars().count()), Some(str))
                    })
            }
            &Japanase::HatsuonKatakana(ref s1, None) => {
                Some(Japanase::HatsuonHiragana(
                    "っ".repeat(s1.chars().count()),
                    None,
                ))
            }
            &Japanase::HatsuonKatakana(ref s1, Some(ref s2)) => {
                map::KATAKANA_HIRAGANA
                    .get(s2.as_ref() as &str)
                    .map(|&s: &&str| s.to_string())
                    .map(|str: String| {
                        Japanase::HatsuonHiragana("っ".repeat(s1.chars().count()), Some(str))
                    })
            }
            _ => None,
        }
    }

    pub fn to_katakana(&self) -> Option<Self> {
        debug!("to_katakana: {:?}", self);
        match self {
            &Japanase::Romaji(ref s) => {
                map::ROMAJI_KATAKANA
                    .get(s.as_ref() as &str)
                    .map(|s: &&str| s.to_string())
                    .map(Japanase::Katakana)
            }
            &Japanase::Hiragana(ref s) => {
                map::HIRAGANA_KATAKANA
                    .get(s.as_ref() as &str)
                    .map(|s: &&str| s.to_string())
                    .map(Japanase::Katakana)
            }
            &Japanase::HatsuonRomaji(ref s1, None) => {
                Some(Japanase::HatsuonKatakana(
                    "ッ".repeat(s1.chars().count()),
                    None,
                ))
            }
            &Japanase::HatsuonRomaji(ref s1, Some(ref s2)) => {
                map::ROMAJI_KATAKANA
                    .get(s2.as_ref() as &str)
                    .map(|&s: &&str| s.to_string())
                    .map(|str: String| {
                        Japanase::HatsuonKatakana("ッ".repeat(s1.chars().count()), Some(str))
                    })
            }
            &Japanase::HatsuonHiragana(ref s1, None) => {
                Some(Japanase::HatsuonKatakana(
                    "ッ".repeat(s1.chars().count()),
                    None,
                ))
            }
            &Japanase::HatsuonHiragana(ref s1, Some(ref s2)) => {
                map::HIRAGANA_KATAKANA
                    .get(s2.as_ref() as &str)
                    .map(|&s: &&str| s.to_string())
                    .map(|str: String| {
                        Japanase::HatsuonKatakana("ッ".repeat(s1.chars().count()), Some(str))
                    })
            }
            _ => None,
        }
    }
}

impl From<Japanase> for String {
    fn from(japanase: Japanase) -> Self {
        match japanase {
            Japanase::Romaji(s) => s,
            Japanase::HatsuonRomaji(s1, None) => s1,
            Japanase::HatsuonRomaji(s1, Some(s2)) => s1 + &s2,
            Japanase::Hiragana(s) => s,
            Japanase::HatsuonHiragana(s1, None) => s1,
            Japanase::HatsuonHiragana(s1, Some(s2)) => s1 + &s2,
            Japanase::Katakana(s) => s,
            Japanase::HatsuonKatakana(s1, None) => s1,
            Japanase::HatsuonKatakana(s1, Some(s2)) => s1 + &s2,
            Japanase::Other(s) => s,
        }
    }
}

pub trait RomajiExt {
    fn to_romaji(&self) -> String;

    fn to_hiragana(&self) -> String;

    fn to_katakana(&self) -> String;
}

impl RomajiExt for str {
    fn to_romaji(&self) -> String {
        parse(self)
            .unwrap()
            .into_iter()
            .map(|x: Japanase| x.to_romaji().unwrap_or(x))
            .map(String::from)
            .collect::<String>()
    }

    fn to_hiragana(&self) -> String {
        parse(self)
            .unwrap()
            .into_iter()
            .map(|x: Japanase| x.to_hiragana().unwrap_or(x))
            .map(String::from)
            .collect::<String>()
    }

    fn to_katakana(&self) -> String {
        parse(self)
            .unwrap()
            .into_iter()
            .map(|x: Japanase| x.to_katakana().unwrap_or(x))
            .map(String::from)
            .collect::<String>()
    }
}
