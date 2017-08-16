pub fn katakana_to_romaji(katakana: String) -> String {
    let mut chars = katakana.chars();
    if chars.next() == Some('ッ') && chars.as_str() != "" {
        let res = romaji_map(chars.as_str());
        return res.chars().next().unwrap().to_string() + &res
    }
    romaji_map(katakana.as_ref())
}

pub fn romaji_map(katakana: &str) -> String {
    match katakana {
        "ア" => "a",
        "イ" => "i",
        "ウ" => "u",
        "エ" => "e",
        "オ" => "o",
        "カ" => "ka",
        "キ" => "ki",
        "ク" => "ku",
        "ケ" => "ke",
        "コ" => "ko",
        "サ" => "sa",
        "シ" => "shi",
        "ス" => "su",
        "セ" => "se",
        "ソ" => "so",
        "タ" => "ta",
        "チ" => "chi",
        "ツ" => "tsu",
        "テ" => "te",
        "ト" => "to",
        "ナ" => "na",
        "ニ" => "ni",
        "ヌ" => "nu",
        "ネ" => "ne",
        "ノ" => "no",
        "ハ" => "ha",
        "ヒ" => "hi",
        "フ" => "fu",
        "ヘ" => "he",
        "ホ" => "ho",
        "マ" => "ma",
        "ミ" => "mi",
        "ム" => "mu",
        "メ" => "me",
        "モ" => "mo",
        "ヤ" => "ya",
        "ユ" => "yu",
        "ヨ" => "yo",
        "ラ" => "ra",
        "リ" => "ri",
        "ル" => "ru",
        "レ" => "re",
        "ロ" => "ro",
        "ワ" => "wa",
        "ウィ" => "wi",
        "ウェ" => "we",
        "ヲ" => "wo",
        "ー" => "-",
        "ガ" => "ga",
        "ギ" => "gi",
        "グ" => "gu",
        "ゲ" => "ge",
        "ゴ" => "go",
        "ザ" => "za",
        "ジ" => "ji",
        "ズ" => "zu",
        "ゼ" => "ze",
        "ゾ" => "zo",
        "ダ" => "da",
        "ヂ" => "di",
        "ヅ" => "du",
        "デ" => "de",
        "ド" => "do",
        "バ" => "ba",
        "ビ" => "bi",
        "ブ" => "bu",
        "ベ" => "be",
        "ボ" => "bo",
        "パ" => "pa",
        "ピ" => "pi",
        "プ" => "pu",
        "ペ" => "pe",
        "ポ" => "po",
        "キャ" => "kya",
        "キュ" => "kyu",
        "キェ" => "kye",
        "キョ" => "kyo",
        "ギャ" => "gya",
        "ギュ" => "gyu",
        "ギェ" => "gye",
        "ギョ" => "gyo",
        "シャ" => "sha",
        "シュ" => "shu",
        "シェ" => "she",
        "ショ" => "sho",
        "ジャ" => "ja",
        "ジュ" => "ju",
        "ジェ" => "je",
        "ジョ" => "jo",
        "チャ" => "cha",
        "チュ" => "chu",
        "チェ" => "che",
        "チョ" => "cho",
        "ヂャ" => "dya",
        "ヂュ" => "dyu",
        "ヂェ" => "dye",
        "ヂョ" => "dyo",
        "テャ" => "tha",
        "ティ" => "thi",
        "テュ" => "thu",
        "テョ" => "tho",
        "トァ" => "twa",
        "トィ" => "twi",
        "トゥ" => "twu",
        "トェ" => "twe",
        "トォ" => "two",
        "ドァ" => "dwa",
        "ドィ" => "dwi",
        "ドゥ" => "dwu",
        "ドェ" => "dwe",
        "ドォ" => "dwo",
        "ニャ" => "nya",
        "ニュ" => "nyu",
        "ニェ" => "nye",
        "ニョ" => "nyo",
        "ヒャ" => "hya",
        "ヒュ" => "hyu",
        "ヒェ" => "hye",
        "ヒョ" => "hyo",
        "ビャ" => "bya",
        "ビュ" => "byu",
        "ビェ" => "bye",
        "ビョ" => "byo",
        "ピャ" => "pya",
        "ピュ" => "pyu",
        "ピェ" => "pye",
        "ピョ" => "pyo",
        "ファ" => "fa",
        "フィ" => "fi",
        "フェ" => "fe",
        "フォ" => "fo",
        "ミャ" => "mya",
        "ミュ" => "myu",
        "ミェ" => "mye",
        "ミョ" => "myo",
        "リャ" => "rya",
        "リュ" => "ryu",
        "リェ" => "rye",
        "リョ" => "ryo",
        "ヴァ" => "va",
        "ヴィ" => "vi",
        "ヴ" => "vu",
        "ヴェ" => "ve",
        "ヴォ" => "vo",
        "デャ" => "dha",
        "ディ" => "dhi",
        "デュ" => "dhu",
        "デェ" => "dhe",
        "デョ" => "dho",
        "ン" => "n",
        "ッ" => "xtsu",
        kana => kana
    }.to_string()
}

#[test]
fn test_katakana_to_romaji() {
    assert_eq!("e", katakana_to_romaji("エ".to_string()));
    assert_eq!("va", katakana_to_romaji("ヴァ".to_string()));
    assert_eq!("tto", katakana_to_romaji("ット".to_string()));
}
