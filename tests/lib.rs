extern crate romaji;

use romaji::Romaji;

#[test]
fn test_to_katakana() {
    assert_eq!("キョウモシナイトネ", Romaji::new("kyoumoshinaitone").to_hiragana());
    assert_eq!("今日モシナイトネ", Romaji::new("今日もshinaitone").to_hiragana());
    assert_eq!("スシノタベタs", Romaji::new("SushiNoTabetas").to_hiragana());
    assert_eq!("シンバシ", Romaji::new("shimbashi").to_hiragana());
    assert_eq!("キンカクジ", Romaji::new("kinkakuji").to_hiragana());
    assert_eq!("トットリ", Romaji::new("tottori").to_hiragana());
    assert_eq!("イイハナシダナー", Romaji::new("イイハナシダナー").to_hiragana());
}

#[test]
fn test_to_hiragana() {
    assert_eq!("きょうもしないとね", Romaji::new("kyoumoshinaitone").to_hiragana());
}

#[test]
fn test_to_romaji() {
    assert_eq!("kyoumoshinaitone", Romaji::new("キョウモシナイトネ").to_romaji());
    assert_eq!("sushinotabetasa", Romaji::new("すしのたべたさ").to_romaji());
    assert_eq!("shimbashi", Romaji::new("シンバシ").to_romaji());
    assert_eq!("kinkakuji", Romaji::new("キンカクジ").to_romaji());
    assert_eq!("tottori", Romaji::new("トットリ").to_romaji());
    assert_eq!("菜xtsu葉", Romaji::new("菜っ葉").to_romaji());
    assert_eq!("vu", Romaji::new("ゔ").to_romaji());
    assert_eq!("wiriamu", Romaji::new("ウィリアム").to_romaji());
    assert_eq!("dhu-ku", Romaji::new("デューク").to_romaji());
    assert_eq!("atsuryokunabe", Romaji::new("アツリョクナベ").to_romaji());
}

#[test]
fn test_normalize() {
    assert_eq!("スシ", Romaji::new("ｽｼ").normalize());
    assert_eq!("パグ", Romaji::new("ﾊﾟｸﾞ").normalize());
}
