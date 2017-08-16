//! Romaji <-> Hira/Kana transliterator

use convert;
use std::ascii::AsciiExt;

pub trait RomajiExt {

    fn to_romaji(&self) -> String;

    fn to_hiragana(&self) -> String;

    fn to_katakana(&self) -> String;

    fn is_kana(&self) -> bool;

    fn is_katakana(&self) -> bool;

    fn is_hiragana(&self) -> bool;

    fn is_sutegana(&self) -> bool;

    fn is_youon(&self) -> bool;

    fn is_hatsuon(&self) -> bool;

    fn romaji_split(&self) -> Vec<String>;

    fn kana_split(&self) -> Vec<String>;

}

impl RomajiExt for str {

    fn to_hiragana(&self) -> String {
        self.to_katakana()
            .romaji_split()
            .iter()
            .cloned()
            .map(|x| convert::romaji_to_katakana(x))
            .map(|x| convert::katakana_to_hiragana(x))
            .collect::<Vec<String>>()
            .join("")
    }

    fn to_katakana(&self) -> String {
        self
            .romaji_split()
            .iter()
            .cloned()
            .map(|x| convert::romaji_to_katakana(x))
            .map(|x| convert::hiragana_to_katakana(x))
            .collect::<Vec<String>>()
            .join("")
    }

    fn to_romaji(&self) -> String {
        self
            .kana_split()
            .iter()
            .cloned()
            .map(|x| convert::katakana_to_romaji(x))
            .collect::<Vec<String>>()
            .join("")
    }

    fn kana_split(&self) -> Vec<String> {
        let mut chars = self
            .chars()
            .map(|x| x.to_string())
            .map(|x| convert::hiragana_to_katakana(x))
            .rev()
            .collect::<Vec<String>>()
        ;

        let mut res = vec!();
        let mut buffer = "".to_string();
        let mut hatsuon = false;
        while let Some(char) = chars.pop() {
            match char.as_ref() {
                x if x.is_youon() && buffer.is_katakana() => {
                    if hatsuon {
                        res.push("ッ".to_string() + &buffer + &char);
                        hatsuon = false;
                    } else {
                        res.push(buffer + &char);
                    }
                    buffer = "".to_string()
                },
                x if x.is_katakana() => {
                    if buffer != "" {
                        if hatsuon {
                            res.push("ッ".to_string() + &buffer);
                            hatsuon = false;
                        } else {
                            res.push(buffer);
                        }
                    }
                    buffer = char.to_string()
                },
                x if x.is_hatsuon() => {
                    if buffer != "" {
                        if hatsuon {
                            res.push("ッ".to_string() + &buffer);
                        } else {
                            res.push(buffer);
                        }
                        buffer = "".to_string();
                    }
                    hatsuon = true
                },
                "" => {}
                _ => {
                    if hatsuon {
                        res.push("ッ".to_string());
                        hatsuon = false;
                    }
                    if buffer != "" {
                        res.push(buffer);
                        buffer = "".to_string()
                    }
                    res.push(char.clone());
                }
            }
        }
        if buffer != "" {
            if hatsuon {
                res.push("ッ".to_string() + &buffer);
            } else {
                res.push(buffer);
            }
        } else if hatsuon {
            res.push("ッ".to_string());
        }
        res
    }

    fn romaji_split(&self) -> Vec<String> {
        let mut chars = self
            .to_ascii_lowercase()
            .chars()
            .map(|x| x.to_string())
            .map(|x| convert::hiragana_to_katakana(x))
            .rev()
            .collect::<Vec<String>>()
        ;

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
                }
                _ if buffer == "" => {
                    res.push(char.clone())
                }
                _ => {
                    res.push(buffer);
                    res.push(char.clone());
                    buffer = "".to_string()
                }
            }
        }
        if buffer != "" {
            res.push(buffer);
        }
        res
    }

    fn is_kana(&self) -> bool {
        self.is_hiragana() || self.is_katakana()
    }

    fn is_katakana(&self) -> bool {
        self == "ア" ||
            self == "イ" ||
            self == "ウ" ||
            self == "エ" ||
            self == "ォ" ||
            self == "オ" ||
            self == "カ" ||
            self == "ガ" ||
            self == "キ" ||
            self == "ギ" ||
            self == "ク" ||
            self == "グ" ||
            self == "ケ" ||
            self == "ゲ" ||
            self == "コ" ||
            self == "ゴ" ||
            self == "サ" ||
            self == "ザ" ||
            self == "シ" ||
            self == "ジ" ||
            self == "ス" ||
            self == "ズ" ||
            self == "セ" ||
            self == "ゼ" ||
            self == "ソ" ||
            self == "ゾ" ||
            self == "タ" ||
            self == "ダ" ||
            self == "チ" ||
            self == "ヂ" ||
            self == "ツ" ||
            self == "ヅ" ||
            self == "テ" ||
            self == "デ" ||
            self == "ト" ||
            self == "ド" ||
            self == "ナ" ||
            self == "ニ" ||
            self == "ヌ" ||
            self == "ネ" ||
            self == "ノ" ||
            self == "ハ" ||
            self == "バ" ||
            self == "パ" ||
            self == "ヒ" ||
            self == "ビ" ||
            self == "ピ" ||
            self == "フ" ||
            self == "ブ" ||
            self == "プ" ||
            self == "ヘ" ||
            self == "ベ" ||
            self == "ペ" ||
            self == "ホ" ||
            self == "ボ" ||
            self == "ポ" ||
            self == "マ" ||
            self == "ミ" ||
            self == "ム" ||
            self == "メ" ||
            self == "モ" ||
            self == "ヤ" ||
            self == "ユ" ||
            self == "ヨ" ||
            self == "ラ" ||
            self == "リ" ||
            self == "ル" ||
            self == "レ" ||
            self == "ロ" ||
            self == "ワ" ||
            self == "ヰ" ||
            self == "ヱ" ||
            self == "ヲ" ||
            self == "ン" ||
            self == "ー"
    }

    fn is_hiragana(&self) -> bool {
        self == "あ" ||
            self == "い" ||
            self == "ぅ" ||
            self == "う" ||
            self == "え" ||
            self == "お" ||
            self == "か" ||
            self == "が" ||
            self == "き" ||
            self == "ぎ" ||
            self == "く" ||
            self == "ぐ" ||
            self == "け" ||
            self == "げ" ||
            self == "こ" ||
            self == "ご" ||
            self == "さ" ||
            self == "ざ" ||
            self == "し" ||
            self == "じ" ||
            self == "す" ||
            self == "ず" ||
            self == "せ" ||
            self == "ぜ" ||
            self == "そ" ||
            self == "ぞ" ||
            self == "た" ||
            self == "だ" ||
            self == "ち" ||
            self == "ぢ" ||
            self == "っ" ||
            self == "つ" ||
            self == "づ" ||
            self == "て" ||
            self == "で" ||
            self == "と" ||
            self == "ど" ||
            self == "な" ||
            self == "に" ||
            self == "ぬ" ||
            self == "ね" ||
            self == "の" ||
            self == "は" ||
            self == "ば" ||
            self == "ぱ" ||
            self == "ひ" ||
            self == "び" ||
            self == "ぴ" ||
            self == "ふ" ||
            self == "ぶ" ||
            self == "ぷ" ||
            self == "へ" ||
            self == "べ" ||
            self == "ぺ" ||
            self == "ほ" ||
            self == "ぼ" ||
            self == "ぽ" ||
            self == "ま" ||
            self == "み" ||
            self == "む" ||
            self == "め" ||
            self == "も" ||
            self == "や" ||
            self == "ゆ" ||
            self == "よ" ||
            self == "ら" ||
            self == "り" ||
            self == "る" ||
            self == "れ" ||
            self == "ろ" ||
            self == "わ" ||
            self == "ゐ" ||
            self == "ゑ" ||
            self == "を" ||
            self == "ん" ||
            self == "ー"
    }

    fn is_sutegana(&self) -> bool {
        self.is_youon() || self.is_hatsuon()
    }

    fn is_youon(&self) -> bool {
        self == "ァ" ||
            self == "ィ" ||
            self == "ゥ" ||
            self == "ェ" ||
            self == "ォ" ||
            self == "ャ" ||
            self == "ュ" ||
            self == "ョ" ||
            self == "ぁ" ||
            self == "ぃ" ||
            self == "ぅ" ||
            self == "ぇ" ||
            self == "ぉ" ||
            self == "ゃ" ||
            self == "ゅ" ||
            self == "ょ"
    }

    fn is_hatsuon(&self) -> bool {
        self == "っ" ||
            self == "ッ"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_method() {
        assert!("あ".is_hiragana());
        assert!("ァ".is_youon());
        assert!("あ".to_string().is_hiragana());
    }

    #[test]
    fn test_romaji_split() {
        assert_eq!(
            vec!["kyo", "u", "mo", "shi", "na", "i", "to", "ne"],
            "kyoumoshinaitone".romaji_split()
        );
        assert_eq!(
            vec!["今", "日", "モ", "shi", "na", "i", "to", "ne"],
            "今日もshinaitone".romaji_split()
        );
        assert_eq!(
            vec!["su", "shi", "no", "ta", "be", "ta", "s"],
            "SushiNoTabetas".romaji_split()
        );
        assert_eq!(
            vec!["shi", "m", "ba", "shi"],
            "shimbashi".romaji_split()
        );
        assert_eq!(
            vec!["ki", "n", "ka", "ku", "ji"],
            "kinkakuji".romaji_split()
        );
        assert_eq!(
            vec!["to", "tto", "ri"],
            "tottori".romaji_split()
        );
        assert_eq!(
            vec!["ha", "ccho", "u"],
            "hacchou".romaji_split()
        );
        assert_eq!(
            vec!["shi", "m", "ba", "shi"],
            "shimbashi".romaji_split()
        );
        assert_eq!(
            vec!["イ", "イ", "ハ", "ナ", "シ", "ダ", "ナ", "ー"],
            "イイハナシダナー".romaji_split()
        );
        assert_eq!(
            vec!["キ", "ョ", "ウ", "モ", "シ", "ナ", "イ", "ト", "ネ"],
            "キョウモシナイトネ".romaji_split()
        );
        assert_eq!(
            vec!["イ", "ッ", "カ", "ク", "ジ", "ュ", "ウ"],
            "イッカクジュウ".romaji_split()
        );
        assert_eq!(
            vec!["菜", "ッ", "葉"],
            "菜っ葉".romaji_split()
        );
    }

    #[test]
    fn test_kana_split() {
        assert_eq!(
            vec!["k", "y", "o", "u", "m", "o", "s", "h", "i", "n", "a", "i", "t", "o", "n", "e"],
            "kyoumoshinaitone".kana_split()
        );
        assert_eq!(
            vec!["今", "日", "モ", "s", "h", "i", "n", "a", "i", "t", "o", "n", "e"],
            "今日もshinaitone".kana_split()
        );
        assert_eq!(
            vec!["S", "u", "s", "h", "i", "N", "o", "T", "a", "b", "e", "t", "a", "s"],
            "SushiNoTabetas".kana_split()
        );
        assert_eq!(
            vec!["s", "h", "i", "m", "b", "a", "s", "h", "i"],
            "shimbashi".kana_split()
        );
        assert_eq!(
            vec!["k", "i", "n", "k", "a", "k", "u", "j", "i"],
            "kinkakuji".kana_split()
        );
        assert_eq!(
            vec!["t", "o", "t", "t", "o", "r", "i"],
            "tottori".kana_split()
        );
        assert_eq!(
            vec!["イ", "イ", "ハ", "ナ", "シ", "ダ", "ナ", "ー"],
            "イイハナシダナー".kana_split()
        );
        assert_eq!(
            vec!["キョ", "ウ", "モ", "シ", "ナ", "イ", "ト", "ネ"],
            "キョウモシナイトネ".kana_split()
        );
        assert_eq!(
            vec!["イ", "ッカ", "ク", "ジュ", "ウ"],
            "イッカクジュウ".kana_split()
        );
        assert_eq!(
            vec!["ハ", "ッチョ", "ウ"],
            "ハッチョウ".kana_split()
        );
        assert_eq!(
            vec!["菜", "ッ", "葉"],
            "菜っ葉".kana_split()
        );
        assert_eq!(
            vec!["菜", "ッ", "葉", "ッ"],
            "菜っ葉っ".kana_split()
        );
    }
}
