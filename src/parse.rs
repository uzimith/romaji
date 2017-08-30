use std::str;
use std::string::String;
use romaji::Japanase;
use error::{Result, Error};
use combine::char::{string};
use combine::{Parser, Stream, many, many1, token, one_of, choice, optional, try, satisfy};

parser!{
    fn japanase[I]()(I) -> Vec<Japanase>
    where
        [I: Stream<Item = char>,]
    {
        many(
            choice!(
                try(romaji()),
                try(hatsuon_romaji()),
                try(hiragana()),
                try(hatsuon_hiragana()),
                try(katakana()),
                try(hatsuon_katakana()),
                try(other())
            )
        )
    }
}

parser!{
    fn romaji[I]()(I) -> Japanase
    where
        [I: Stream<Item = char>,]
    {
        choice!(
            try((optional(romaji_consonant()), romaji_vowel()))
            .map(|(a, b): (Option<String>, char)|
                vec!(a, Some(b.to_string())).into_iter().flat_map(|c| c).collect::<String>()
            ),
            try(romaji_bion()).map(|c: char| c.to_string())
        ).map(|c: String| Japanase::romaji(c))
    }
}

parser!{
    fn romaji_consonant[I]()(I) -> String
    where
        [I: Stream<Item = char>,]
    {
        choice([
            try(string("ts")),
            try(string("xw")),
            try(string("xts")),
            try(string("xy")),
            try(string("xk")),
            try(string("cy")),
            try(string("ch")),
            try(string("ky")),
            try(string("gy")),
            try(string("zy")),
            try(string("jy")),
            try(string("sy")),
            try(string("sh")),
            try(string("ty")),
            try(string("tw")),
            try(string("dy")),
            try(string("dw")),
            try(string("dh")),
            try(string("th")),
            try(string("ty")),
            try(string("ny")),
            try(string("hy")),
            try(string("by")),
            try(string("py")),
            try(string("my")),
            try(string("ry")),
            try(string("ly")),
        ]).map(|s: &str| s.to_string() )
        .or(one_of("xckgzjstdtnhbpmrl".chars()).map(|s: char| s.to_string()))
    }
}

parser!{
    fn romaji_vowel[I]()(I) -> char
    where
        [I: Stream<Item = char>,]
    {
        choice([token('a'), token('i'), token('u'), token('e'), token('o')])
    }
}

parser!{
    fn romaji_bion[I]()(I) -> char
    where
        [I: Stream<Item = char>,]
    {
        choice([token('n'), token('m')])
    }
}

parser!{
    fn hatsuon_romaji[I]()(I) -> Japanase
    where
        [I: Stream<Item = char>,]
    {
        choice!(
            try((hatsuon_romaji_consonant(), romaji_vowel()))
            .map(|((a, b), c): ((String, String), char)|
                Japanase::hatsuon_romaji_some(a, b + &c.to_string())
            ),
            (many1(token('t')), string("ch"), romaji_vowel())
            .map(|(a, b, c): (Vec<char>, &str, char)|
                Japanase::hatsuon_romaji_some(a.into_iter().collect::<String>(),
                    b.to_string() + &c.to_string())
            )
        )
    }
}

parser!{
    fn hatsuon_romaji_consonant[I]()(I) -> (String, String)
    where
        [I: Stream<Item = char>,]
    {
        choice!(
            try((token('x'), many1(token('x')), string("ts")))
            .map(|(a, b, c): (char, Vec<char>, &str)| {
                (a.to_string().repeat(b.len()), a.to_string() + c)
            }),
            choice([
                try((token('t'), many1(token('t')), token('s'))),
                try((token('x'), many1(token('x')), token('w'))),
                try((token('x'), many1(token('x')), token('y'))),
                try((token('x'), many1(token('x')), token('k'))),
                try((token('x'), many1(token('x')), token('t'))),
                try((token('c'), many1(token('c')), token('y'))),
                try((token('c'), many1(token('c')), token('h'))),
                try((token('k'), many1(token('k')), token('y'))),
                try((token('g'), many1(token('g')), token('y'))),
                try((token('z'), many1(token('z')), token('y'))),
                try((token('j'), many1(token('j')), token('y'))),
                try((token('s'), many1(token('s')), token('y'))),
                try((token('s'), many1(token('s')), token('h'))),
                try((token('t'), many1(token('t')), token('y'))),
                try((token('t'), many1(token('t')), token('w'))),
                try((token('d'), many1(token('d')), token('y'))),
                try((token('d'), many1(token('d')), token('w'))),
                try((token('d'), many1(token('d')), token('h'))),
                try((token('t'), many1(token('t')), token('h'))),
                try((token('t'), many1(token('t')), token('y'))),
                try((token('n'), many1(token('n')), token('y'))),
                try((token('h'), many1(token('h')), token('y'))),
                try((token('b'), many1(token('b')), token('y'))),
                try((token('p'), many1(token('p')), token('y'))),
                try((token('m'), many1(token('m')), token('y'))),
                try((token('r'), many1(token('r')), token('y'))),
                try((token('l'), many1(token('l')), token('y')))
            ])
            .map(|(a, b, c): (char, Vec<char>, char)| {
                (a.to_string().repeat(b.len()), a.to_string() + &c.to_string())
            }),
            try(one_of("xckgzjstdtnhbpmrl".chars()).then(|c: char| many1(token(c))))
            .map(|c: Vec<char>| (c[0].to_string().repeat(c.len()), c[0].to_string()))
        )
    }
}

parser!{
    fn hiragana[I]()(I) -> Japanase
    where
        [I: Stream<Item = char>,]
    {
        (
            optional(many1(token('っ'))),
            one_of("あぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢつづてでとどなにぬねの\
                    はばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろゎわゐゑをんゔゕゖ".chars()),
            optional(one_of("ぁぃぅぇぉゃゅょ".chars())),
        )
        .map(|(a, b, c): (Option<Vec<char>>, char, Option<char>)|
            match a {
                Some(a) => Japanase::hatsuon_hiragana_some(a.into_iter().collect::<String>(),
                    vec!(Some(b), c).into_iter().flat_map(|s| s).collect::<String>()),
                None => Japanase::hiragana(vec!(Some(b), c).into_iter()
                    .flat_map(|s| s).collect::<String>()),
            }
        )
    }
}

parser!{
    fn hatsuon_hiragana[I]()(I) -> Japanase
    where
        [I: Stream<Item = char>,]
    {
        token('っ').map(|c: char| Japanase::hatsuon_hiragana(c.to_string()))
    }
}

parser!{
    fn katakana[I]()(I) -> Japanase
    where
        [I: Stream<Item = char>,]
    {
        (
            optional(many1(token('ッ'))),
            one_of("アィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂツヅテデトドナニヌネノ\
                    ハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリルレロヮワヰヱヲンヴヵヶー".chars()),
            optional(one_of("ァィゥェォャュョ".chars())),
        )
        .map(|(a, b, c): (Option<Vec<char>>, char, Option<char>)|
            match a {
                Some(a) => Japanase::hatsuon_katakana_some(a.into_iter().collect::<String>(),
                    vec!(Some(b), c).into_iter().flat_map(|s| s).collect::<String>()),
                None => Japanase::katakana(vec!(Some(b), c).into_iter()
                    .flat_map(|s| s).collect::<String>()),
            }
        )
    }
}

parser!{
    fn hatsuon_katakana[I]()(I) -> Japanase
    where
        [I: Stream<Item = char>,]
    {
        token('ッ').map(|c: char| Japanase::hatsuon_katakana(c.to_string()))
    }
}

parser!{
    fn other[I]()(I) -> Japanase
    where
        [I: Stream<Item = char>,]
    {
        satisfy(|_| true).expected("other").map(|c: char| Japanase::other(c.to_string()))
    }
}


pub fn parse<T: AsRef<str>>(input: T) -> Result<Vec<Japanase>> {
    let input: String = input.as_ref().to_lowercase();
    let input: &str = input.as_ref();
    match japanase().parse(input) {
        Ok((output, _)) => Ok(output),
        e => {
            debug!("{:?}", e);
            Err(Error::RomajiParse)
        }
    }
}

#[cfg(test)]
mod tests {
    use env_logger;
    use super::*;
    use romaji::Japanase;

    fn r(s: &str) -> Japanase {
        Japanase::romaji(s)
    }

    fn hrs(s1: &str, s2: &str) -> Japanase {
        Japanase::hatsuon_romaji_some(s1, s2)
    }

    fn h(s: &str) -> Japanase {
        Japanase::hiragana(s)
    }

    fn hh(s: &str) -> Japanase {
        Japanase::hatsuon_hiragana(s)
    }

    fn hhs(s1: &str, s2: &str) -> Japanase {
        Japanase::hatsuon_hiragana_some(s1, s2)
    }

    fn k(s: &str) -> Japanase {
        Japanase::katakana(s)
    }

    fn hks(s1: &str, s2: &str) -> Japanase {
        Japanase::hatsuon_katakana_some(s1, s2)
    }

    fn o(s: &str) -> Japanase {
        Japanase::other(s)
    }

    #[test]
    fn test_romaji_parse1() {
        let _ = env_logger::init();
        assert_eq!(
            parse("kyoumoshinaitone"),
            Ok(vec![
                r("kyo"),
                r("u"),
                r("mo"),
                r("shi"),
                r("na"),
                r("i"),
                r("to"),
                r("ne"),
            ])
        );
    }

    #[test]
    fn test_romaji_parse2() {
        let _ = env_logger::init();
        assert_eq!(
            parse("今日もshinaitone"),
            Ok(vec![
                o("今"),
                o("日"),
                h("も"),
                r("shi"),
                r("na"),
                r("i"),
                r("to"),
                r("ne"),
            ])
        );
    }

    #[test]
    fn test_romaji_parse3() {
        let _ = env_logger::init();
        assert_eq!(
            parse("SushiNoTabetas"),
            Ok(vec![
                r("su"),
                r("shi"),
                r("no"),
                r("ta"),
                r("be"),
                r("ta"),
                o("s"),
            ])
        );
    }

    #[test]
    fn test_romaji_parse4() {
        let _ = env_logger::init();
        assert_eq!(
            parse("shimbashi"),
            Ok(vec![r("shi"), r("m"), r("ba"), r("shi")])
        );
    }

    #[test]
    fn test_romaji_parse5() {
        let _ = env_logger::init();
        assert_eq!(
            parse("kinkakuji"),
            Ok(vec![r("ki"), r("n"), r("ka"), r("ku"), r("ji")])
        );
    }

    #[test]
    fn test_romaji_parse6() {
        let _ = env_logger::init();
        assert_eq!(parse("tottori"), Ok(vec![r("to"), hrs("t", "to"), r("ri")]));
    }

    #[test]
    fn test_romaji_parse7() {
        let _ = env_logger::init();
        assert_eq!(parse("hacchou"), Ok(vec![r("ha"), hrs("c", "cho"), r("u")]));
    }

    #[test]
    fn test_romaji_parse15() {
        let _ = env_logger::init();
        assert_eq!(parse("hatchou"), Ok(vec![r("ha"), hrs("t", "cho"), r("u")]));
    }

    #[test]
    fn test_romaji_parse8() {
        let _ = env_logger::init();
        assert_eq!(
            parse("イイハナシダナー"),
            Ok(vec![
                k("イ"),
                k("イ"),
                k("ハ"),
                k("ナ"),
                k("シ"),
                k("ダ"),
                k("ナ"),
                k("ー"),
            ])
        );
    }

    #[test]
    fn test_romaji_parse9() {
        let _ = env_logger::init();
        assert_eq!(
            parse("キョウモシナイトネ"),
            Ok(vec![
                k("キョ"),
                k("ウ"),
                k("モ"),
                k("シ"),
                k("ナ"),
                k("イ"),
                k("ト"),
                k("ネ"),
            ])
        );
    }

    #[test]
    fn test_romaji_parse10() {
        let _ = env_logger::init();
        assert_eq!(
            parse("イッカクジュウ"),
            Ok(vec![
                k("イ"),
                hks("ッ", "カ"),
                k("ク"),
                k("ジュ"),
                k("ウ"),
            ])
        );
    }

    #[test]
    fn test_romaji_parse11() {
        let _ = env_logger::init();
        assert_eq!(parse("菜っ葉"), Ok(vec![o("菜"), hh("っ"), o("葉")]));
    }

    #[test]
    fn test_romaji_parse12() {
        let _ = env_logger::init();
        assert_eq!(
            parse("とっっっとり"),
            Ok(vec![h("と"), hhs("っっっ", "と"), h("り")])
        );
    }

    #[test]
    fn test_romaji_parse13() {
        let _ = env_logger::init();
        assert_eq!(parse("xaxtsuxyo"), Ok(vec![r("xa"), r("xtsu"), r("xyo")]));
    }

    #[test]
    fn test_romaji_parse14() {
        let _ = env_logger::init();
        assert_eq!(
            parse("hikkoshi"),
            Ok(vec![r("hi"), hrs("k", "ko"), r("shi")])
        );
    }

    #[test]
    fn test_romaji_parse16() {
        let _ = env_logger::init();
        assert_eq!(
            parse("エヴァンゲリオン"),
            Ok(vec![
                k("エ"),
                k("ヴァ"),
                k("ン"),
                k("ゲ"),
                k("リ"),
                k("オ"),
                k("ン"),
            ])
        );
    }

    #[test]
    fn test_romaji_parse17() {
        let _ = env_logger::init();
        assert_eq!(
            parse("はっちょう"),
            Ok(vec![h("は"), hhs("っ", "ちょ"), h("う")])
        );
    }

    #[test]
    fn test_romaji_parse18() {
        let _ = env_logger::init();
        assert_eq!(
            parse("tottttori"),
            Ok(vec![r("to"), hrs("ttt", "to"), r("ri")])
        );
    }
}
