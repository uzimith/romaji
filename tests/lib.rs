extern crate romaji;

use romaji::Romaji;

#[test]
fn test_katakana() {
    assert_eq!("キョウモシナイトネ", Romaji::new("kyoumoshinaitone").katakana().to_string());
    assert_eq!("今日モシナイトネ", Romaji::new("今日もshinaitone").katakana().to_string());
    assert_eq!("イッカクジュウ", Romaji::new("ikkakuzyuu").katakana().to_string());
    assert_eq!("スシノタベタs", Romaji::new("SushiNoTabetas").katakana().to_string());
    assert_eq!("シンバシ", Romaji::new("shimbashi").katakana().to_string());
    assert_eq!("キンカクジ", Romaji::new("kinkakuji").katakana().to_string());
    assert_eq!("トットリ", Romaji::new("tottori").katakana().to_string());
    assert_eq!("イイハナシダナー", Romaji::new("イイハナシダナー").katakana().to_string());
}

#[test]
fn test_hiragana() {
    assert_eq!("きょうもしないとね", Romaji::new("kyoumoshinaitone").hiragana().to_string());
}

#[test]
fn test_romaji() {
    assert_eq!("kyoumoshinaitone", Romaji::new("キョウモシナイトネ").romaji().to_string());
    assert_eq!("sushinotabetasa", Romaji::new("すしのたべたさ").romaji().to_string());
    assert_eq!("shimbashi", Romaji::new("シンバシ").romaji().to_string());
    assert_eq!("kinkakuji", Romaji::new("キンカクジ").romaji().to_string());
    assert_eq!("tottori", Romaji::new("トットリ").romaji().to_string());
    assert_eq!("菜xtsu葉", Romaji::new("菜っ葉").romaji().to_string());
    assert_eq!("vu", Romaji::new("ゔ").romaji().to_string());
    assert_eq!("wiriamu", Romaji::new("ウィリアム").romaji().to_string());
    assert_eq!("dhu-ku", Romaji::new("デューク").romaji().to_string());
    assert_eq!("atsuryokunabe", Romaji::new("アツリョクナベ").romaji().to_string());
}
