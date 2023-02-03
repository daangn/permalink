use lazy_static::lazy_static;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

pub(crate) fn slugify(str: &str) -> String {
    lazy_static! {
        static ref RE1: Regex = Regex::new(
            // strip half-width quotation marks
            // - 0022: quotation mark (")
            // - 0027: apostrophe (')
            // - 02bc: modifier letter apostrophe
            // - 0060: grave accent
            // - 00b4: acute accent
            // - 2018: left single quotation mark
            // - 2019: right single quotation mark
            // - 201c: left double quotation mark
            // - 201d: right double quotation mark
            r"[\u0022\u0027\u02bc\u0060\u00b4\u2018\u2019\u201c\u201d]",
        ).unwrap();

        static ref RE2: Regex = Regex::new(
            // replace all whitespaces with a dash
            r"[\s_]+",
        ).unwrap();

        static ref RE3: Regex = Regex::new(
            // replace non-allowed sequences with a dash
            //
            // allowed sequences:
            // - Alphanumeric
            // - Dash
            // - SEO-friendly CJK character sequences
            //   3040 - 309f: Hiragana
            //   30a0 - 30ff: Katakana
            //   3400 - 4dbf: CJK unified ideographs Extension A - Rare Kanji
            //   4e00 - 9faf: CJK unified ideographs - Common and uncommon Kanji
            //   ac00 - d7a3: Korean completed words (가-힣)
            //   ff00 - ff9f: Full-width Roman characters and half-width Katakana
            r"[^a-zA-Z\d\-\u3040-\u309f\u30a0-\u30ff\u3400-\u4dbf\u4e00-\u9faf\uac00-\ud7a3\uff00-\uff9f]",
        ).unwrap();

        static ref RE4: Regex = Regex::new(
            // replace multiple dashes with a single dash
            r"-+"
        ).unwrap();

        static ref RE5: Regex = Regex::new(
            // remove leading / trailing dashes
            r"^-|-$"
        ).unwrap();
    }

    let slug = normalize(str);
    let slug = RE1.replace_all(slug.as_str(), "").to_string();
    let slug = RE2.replace_all(slug.as_str(), "-").to_string();
    let slug = RE3.replace_all(slug.as_str(), "-").to_string();
    let slug = RE4.replace_all(slug.as_str(), "-").to_string();
    let slug = RE5.replace_all(slug.as_str(), "").to_string();

    slug.to_lowercase()
}

#[test]
fn test_slugify_1() {
    assert_eq!(
        slugify("당근마켓 - 대한민국 1등 동네 커뮤니티"),
        "당근마켓-대한민국-1등-동네-커뮤니티".to_string(),
    );
}

#[test]
fn test_slugify_2() {
    assert_eq!(
        slugify("PM/기획"),
        "pm-기획".to_string(),
    );
}

#[test]
fn test_slugify_3() {
    assert_eq!(
        slugify("개발 (iOS / 안드로이드)"),
        "개발-ios-안드로이드".to_string(),
    );
}

#[test]
fn test_slugify_4() {
    assert_eq!(
        slugify("자바스크립트도 \"당근\"이세요?"),
        "자바스크립트도-당근이세요".to_string(),
    );
}

#[test]
fn test_slugify_5() {
    assert_eq!(
        slugify("ホロライブプロダクション Hororaibu Purodakushon"),
        "ホロライブプロダクション-hororaibu-purodakushon".to_string(),
    );
}

#[test]
fn test_slugify_6() {
    assert_eq!(
        slugify("------TITLE------"),
        "title".to_string(),
    );
}

#[test]
fn test_slugify_7() {
    assert_eq!(
        slugify("Parse, don’t validate"),
        "parse-dont-validate".to_string(),
    );
}

#[test]
fn test_slugify_8() {
    assert_eq!(
        slugify("Alexis King said \"Parse, don\'t validate\""),
        "alexis-king-said-parse-dont-validate".to_string(),
    );
}

#[test]
fn test_slugify_9() {
    assert_eq!(
        slugify("Someone shout \"help, help!\", but no one care"),
        "someone-shout-help-help-but-no-one-care".to_string(),
    );
}

fn normalize(str: &str) -> String {
    normalize_hangul(normalize_katakana(str).as_str())
}

fn normalize_hangul(str: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"([\u1100-\u11ff\u3130-\u318f\u3200-\u321e\u3260-\u327f\uffa0-\uffdc\uffe6]+)",
        ).unwrap();
    }

    RE.replace_all(str, |caps: &regex::Captures<'_>| {
        let substr = caps.get(1).unwrap().as_str();
        substr.nfkc().collect::<String>()
    }).to_string()
}

#[test]
fn test_normalize_hungul() {
    assert_eq!(
        normalize_hangul("㈜당근마켓"),
        "(주)당근마켓",
    );
}

fn normalize_katakana(str: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"([\uff60-\uff9f]+)",
        ).unwrap();
    }

    RE.replace_all(str, |caps: &regex::Captures<'_>| {
        let substr = caps.get(1).unwrap().as_str();
        substr.nfkc().collect::<String>()
    }).to_string()
}

#[test]
fn test_normalize_katakana() {
    assert_eq!(
        normalize_katakana("ﾆｯﾎﾟﾝ"),
        "ニッポン",
    );
}
