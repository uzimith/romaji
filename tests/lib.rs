extern crate romaji;
extern crate env_logger;
extern crate log;

use romaji::romaji::RomajiExt;

#[test]
fn test_to_katakana() {
    assert_eq!(
        "キョウモシナイトネ",
        "kyoumoshinaitone".to_katakana()
    );
    assert_eq!(
        "今日モシナイトネ",
        "今日もshinaitone".to_katakana()
    );
    assert_eq!("イッカクジュウ", "ikkakuzyuu".to_katakana());
    assert_eq!("スシノタベタs", "SushiNoTabetas".to_katakana());
    assert_eq!("シンバシ", "shimbashi".to_katakana());
    assert_eq!("キンカクジ", "kinkakuji".to_katakana());
    assert_eq!("トットリ", "tottori".to_katakana());
    assert_eq!(
        "イイハナシダナー",
        "イイハナシダナー".to_katakana()
    );
    assert_eq!("ハッチョウ", "hacchou".to_katakana());
    assert_eq!(
        "イイハナシダナー",
        "いいはなしだなー".to_katakana()
    );
}

#[test]
fn test_to_hiragana() {
    assert_eq!(
        "きょうもしないとね",
        "kyoumoshinaitone".to_hiragana()
    );
    assert_eq!(
        "今日もしないとね",
        "今日もshinaitone".to_hiragana()
    );
    assert_eq!("いっかくじゅう", "ikkakuzyuu".to_hiragana());
    assert_eq!("すしのたべたs", "SushiNoTabetas".to_hiragana());
    assert_eq!("しんばし", "shimbashi".to_hiragana());
    assert_eq!("きんかくじ", "kinkakuji".to_hiragana());
    assert_eq!(
        "いいはなしだなー",
        "いいはなしだなー".to_hiragana()
    );
    assert_eq!(
        "いいはなしだなー",
        "イイハナシダナー".to_hiragana()
    );
    assert_eq!("はっちょう", "hacchou".to_hiragana());
    assert_eq!("とっとりっ", "tottorixtsu".to_hiragana());
    assert_eq!("ぁっょっっっっ", "xaxtsuxyoxxxxtu".to_hiragana());
}

#[test]
fn test_to_romaji() {
    let _ = env_logger::init();
    assert_eq!(
        "kyoumoshinaitone",
        "キョウモシナイトネ".to_romaji()
    );
    assert_eq!("sushinotabetasa", "すしのたべたさ".to_romaji());
    assert_eq!("shinbashi", "シンバシ".to_romaji());
    assert_eq!("kinkakuji", "キンカクジ".to_romaji());
    assert_eq!("菜xtsu葉", "菜っ葉".to_romaji());
    assert_eq!("vu", "ゔ".to_romaji());
    assert_eq!("wiriamu", "ウィリアム".to_romaji());
    assert_eq!("dhu-ku", "デューク".to_romaji());
    assert_eq!("hacchou", "ハッチョウ".to_romaji());
    assert_eq!("atsuryokunabe", "アツリョクナベ".to_romaji());
    assert_eq!("tottori", "トットリ".to_romaji());

    assert_eq!(
        "吾輩ha猫dearu。名前hamada無i。\ndokode生retakatonto見当gatsukanu。",
        "吾輩は猫である。名前はまだ無い。\nどこで生れたかとんと見当がつかぬ。".to_romaji()
    );
}
