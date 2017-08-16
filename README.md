# romaji

inspired by [romaji](https://github.com/makimoto/romaji) in Ruby

my romaji follows Kunrei-shiki(訓令式) that we can type by keyboard.

## Usage

```toml
[dependencies]
romaji = "0.1"
```

```rust
extern crate romaji;

use romaji::RomajiExt;

fn main() {
    "kyoumoshinaitone".to_katakana(); // => "キョウモシナイトネ".to_string()
    "kyoumoshinaitone".to_hiragana(); // => "きょうもしないとね".to_string()
    "キョウモシナイトネ".to_romaji();    // => "kyoumoshinaitone".to_string()
}
```