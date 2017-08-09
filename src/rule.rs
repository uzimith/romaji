pub fn is_kana(kana: &str) -> bool {
    is_hiragana(kana) || is_katakana(kana)
}

pub fn is_katakana(kana: &str) -> bool {
    kana == "ア" ||
        kana == "イ" ||
        kana == "ウ" ||
        kana == "エ" ||
        kana == "ォ" ||
        kana == "オ" ||
        kana == "カ" ||
        kana == "ガ" ||
        kana == "キ" ||
        kana == "ギ" ||
        kana == "ク" ||
        kana == "グ" ||
        kana == "ケ" ||
        kana == "ゲ" ||
        kana == "コ" ||
        kana == "ゴ" ||
        kana == "サ" ||
        kana == "ザ" ||
        kana == "シ" ||
        kana == "ジ" ||
        kana == "ス" ||
        kana == "ズ" ||
        kana == "セ" ||
        kana == "ゼ" ||
        kana == "ソ" ||
        kana == "ゾ" ||
        kana == "タ" ||
        kana == "ダ" ||
        kana == "チ" ||
        kana == "ヂ" ||
        kana == "ツ" ||
        kana == "ヅ" ||
        kana == "テ" ||
        kana == "デ" ||
        kana == "ト" ||
        kana == "ド" ||
        kana == "ナ" ||
        kana == "ニ" ||
        kana == "ヌ" ||
        kana == "ネ" ||
        kana == "ノ" ||
        kana == "ハ" ||
        kana == "バ" ||
        kana == "パ" ||
        kana == "ヒ" ||
        kana == "ビ" ||
        kana == "ピ" ||
        kana == "フ" ||
        kana == "ブ" ||
        kana == "プ" ||
        kana == "ヘ" ||
        kana == "ベ" ||
        kana == "ペ" ||
        kana == "ホ" ||
        kana == "ボ" ||
        kana == "ポ" ||
        kana == "マ" ||
        kana == "ミ" ||
        kana == "ム" ||
        kana == "メ" ||
        kana == "モ" ||
        kana == "ヤ" ||
        kana == "ユ" ||
        kana == "ヨ" ||
        kana == "ラ" ||
        kana == "リ" ||
        kana == "ル" ||
        kana == "レ" ||
        kana == "ロ" ||
        kana == "ワ" ||
        kana == "ヰ" ||
        kana == "ヱ" ||
        kana == "ヲ" ||
        kana == "ン" ||
        kana == "ー"
}

pub fn is_hiragana(kana: &str) -> bool {
    kana == "あ" ||
        kana == "い" ||
        kana == "ぅ" ||
        kana == "う" ||
        kana == "え" ||
        kana == "お" ||
        kana == "か" ||
        kana == "が" ||
        kana == "き" ||
        kana == "ぎ" ||
        kana == "く" ||
        kana == "ぐ" ||
        kana == "け" ||
        kana == "げ" ||
        kana == "こ" ||
        kana == "ご" ||
        kana == "さ" ||
        kana == "ざ" ||
        kana == "し" ||
        kana == "じ" ||
        kana == "す" ||
        kana == "ず" ||
        kana == "せ" ||
        kana == "ぜ" ||
        kana == "そ" ||
        kana == "ぞ" ||
        kana == "た" ||
        kana == "だ" ||
        kana == "ち" ||
        kana == "ぢ" ||
        kana == "っ" ||
        kana == "つ" ||
        kana == "づ" ||
        kana == "て" ||
        kana == "で" ||
        kana == "と" ||
        kana == "ど" ||
        kana == "な" ||
        kana == "に" ||
        kana == "ぬ" ||
        kana == "ね" ||
        kana == "の" ||
        kana == "は" ||
        kana == "ば" ||
        kana == "ぱ" ||
        kana == "ひ" ||
        kana == "び" ||
        kana == "ぴ" ||
        kana == "ふ" ||
        kana == "ぶ" ||
        kana == "ぷ" ||
        kana == "へ" ||
        kana == "べ" ||
        kana == "ぺ" ||
        kana == "ほ" ||
        kana == "ぼ" ||
        kana == "ぽ" ||
        kana == "ま" ||
        kana == "み" ||
        kana == "む" ||
        kana == "め" ||
        kana == "も" ||
        kana == "や" ||
        kana == "ゆ" ||
        kana == "よ" ||
        kana == "ら" ||
        kana == "り" ||
        kana == "る" ||
        kana == "れ" ||
        kana == "ろ" ||
        kana == "わ" ||
        kana == "ゐ" ||
        kana == "ゑ" ||
        kana == "を" ||
        kana == "ん" ||
        kana == "ー"
}

pub fn is_sutegana(kana: &str) -> bool {
    is_youon(kana) || is_hatsuon(kana)
}

pub fn is_youon(kana: &str) -> bool {
    kana == "ァ" ||
        kana == "ィ" ||
        kana == "ゥ" ||
        kana == "ェ" ||
        kana == "ォ" ||
        kana == "ャ" ||
        kana == "ュ" ||
        kana == "ョ" ||
        kana == "ぁ" ||
        kana == "ぃ" ||
        kana == "ぅ" ||
        kana == "ぇ" ||
        kana == "ぉ" ||
        kana == "ゃ" ||
        kana == "ゅ" ||
        kana == "ょ"
}

pub fn is_hatsuon(kana: &str) -> bool {
    kana == "っ" ||
        kana == "ッ"
}
