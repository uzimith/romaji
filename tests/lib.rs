extern crate romaji;

use romaji::Romaji;

#[test]
fn test_katakana() {
    assert_eq!("キョウモシナイトネ", Romaji::new("kyoumoshinaitone").hiragana().to_str());
    assert_eq!("今日モシナイトネ", Romaji::new("今日もshinaitone").hiragana().to_str());
    assert_eq!("スシノタベタs", Romaji::new("SushiNoTabetas").hiragana().to_str());
    assert_eq!("シンバシ", Romaji::new("shimbashi").hiragana().to_str());
    assert_eq!("キンカクジ", Romaji::new("kinkakuji").hiragana().to_str());
    assert_eq!("トットリ", Romaji::new("tottori").hiragana().to_str());
    assert_eq!("イイハナシダナー", Romaji::new("イイハナシダナー").hiragana().to_str());
}

#[test]
fn test_hiragana() {
    assert_eq!("きょうもしないとね", Romaji::new("kyoumoshinaitone").hiragana().to_str());
}

#[test]
fn test_romaji() {
    assert_eq!("kyoumoshinaitone", Romaji::new("キョウモシナイトネ").romaji().to_str());
    assert_eq!("sushinotabetasa", Romaji::new("すしのたべたさ").romaji().to_str());
    assert_eq!("shimbashi", Romaji::new("シンバシ").romaji().to_str());
    assert_eq!("kinkakuji", Romaji::new("キンカクジ").romaji().to_str());
    assert_eq!("tottori", Romaji::new("トットリ").romaji().to_str());
    assert_eq!("菜xtsu葉", Romaji::new("菜っ葉").romaji().to_str());
    assert_eq!("vu", Romaji::new("ゔ").romaji().to_str());
    assert_eq!("wiriamu", Romaji::new("ウィリアム").romaji().to_str());
    assert_eq!("dhu-ku", Romaji::new("デューク").romaji().to_str());
    assert_eq!("atsuryokunabe", Romaji::new("アツリョクナベ").romaji().to_str());
}

#[test]
fn test_normalize() {
    assert_eq!("スシ", Romaji::new("ｽｼ").normalize().to_str());
    assert_eq!("パグ", Romaji::new("ﾊﾟｸﾞ").normalize().to_str());
}
