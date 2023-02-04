use std::{str::FromStr, fmt::Display};

use percent_encoding::{percent_decode_str, utf8_percent_encode};
use pest::Parser;
use pest_derive::Parser;
use url::Url;

use crate::cjk_slug;

#[derive(Debug, Parser)]
#[grammar = "permalink.pest"]
struct PathnameParser;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum PermalinkError {
    #[error("invalid url")]
    InvalidUrl(url::ParseError),
    #[error("invalid permalink")]
    InvalidPermalink(pest::error::Error<Rule>),
    #[error("unknown country code `{0}`")]
    UnknownCountry(String),
}

impl From<url::ParseError> for PermalinkError {
    fn from(err: url::ParseError) -> Self {
        Self::InvalidUrl(err)
    }
}

impl From<pest::error::Error<Rule>> for PermalinkError {
    fn from(err: pest::error::Error<Rule>) -> Self {
        Self::InvalidPermalink(err)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permalink {
    pub country: WellKnownCountry,
    pub language: WellKnownLanguage,
    pub service_type: String,
    pub title: Option<String>,
    pub id: String,
    pub data: Option<String>,
}

impl Permalink {
    pub fn parse_str(url_like: &str) -> Result<Self, PermalinkError> {
        let url = Url::parse(url_like)?;
        Self::parse_url(url)
    }

    pub fn parse_url(url: Url) -> Result<Self, PermalinkError> {
        let pathname = PathnameParser::parse(Rule::pathname, url.path())?.next().unwrap();

        let mut permalink = Permalink::default();

        let mut pathname_rules = pathname.into_inner();

        let country = pathname_rules
            .next()
            .unwrap()
            .as_str();
        permalink.country = WellKnownCountry::from_str(country)?;

        let service_type = pathname_rules
            .next()
            .unwrap()
            .as_str();
        permalink.service_type = service_type.to_string();

        let slug_rules = pathname_rules
            .next()
            .unwrap()
            .into_inner();
        for rule in slug_rules {
            match rule.as_rule() {
                Rule::title => {
                    let mut chars = rule.as_str().chars();
                    chars.next_back();
                    permalink.title = Some(
                        percent_decode_str(chars.as_str())
                            .decode_utf8()
                            .unwrap()
                            .to_string()
                    );
                },
                Rule::id => {
                    permalink.id = rule.as_str().to_string();
                },
                _ => {},
            }
        }


        println!("test {}", permalink);

        permalink.data = pathname_rules.next()
            .map(|rule| rule.as_str().to_string());

        Ok(permalink)
    }

    pub fn normalize(self: &Self) -> String {
        format!(
            "{}/{}/{}/{}/",
            "https://www.karrotmarket.com",
            self.country.to_string(),
            self.service_type,
            self.id,
        )
    }

    pub fn canonicalize(self: &Self, title: &str) -> String {
        const NON_URL_SAFE: &percent_encoding::AsciiSet = &percent_encoding::CONTROLS
            .add(b' ')
            .add(b'!')
            .add(b'"')
            .add(b'#')
            .add(b'$')
            .add(b'%')
            .add(b'&')
            .add(b'\'')
            .add(b'(')
            .add(b')')
            .add(b'*')
            .add(b'+')
            .add(b',')
            .add(b'.')
            .add(b'/')
            .add(b':')
            .add(b';')
            .add(b'<')
            .add(b'=')
            .add(b'>')
            .add(b'?')
            .add(b'@')
            .add(b'[')
            .add(b'\\')
            .add(b']')
            .add(b'^')
            .add(b'`')
            .add(b'{')
            .add(b'|')
            .add(b'}')
            .add(b'~');

        let origin = well_known_origin_from_country(self.country);
        format!(
            "{}/{}/{}/{}/",
            origin,
            self.country,
            self.service_type,
            utf8_percent_encode(
                cjk_slug::slugify(format!("{}-{}", title, self.id).as_str()).as_str(),
                NON_URL_SAFE,
            ),
        )
    }
}

impl Default for Permalink {
    fn default() -> Self {
        Self {
            country: WellKnownCountry::KR,
            language: WellKnownLanguage::KO,
            service_type: "about".to_string(),
            title: None,
            id: "blank".to_string(),
            data: None,
        }
    }
}

impl Display for Permalink {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(fmt, "permalink")?;
        writeln!(fmt, "\tcountry: {}", self.country)?;
        writeln!(fmt, "\tlanguage: {}", self.language)?;
        writeln!(fmt, "\tservice_type: {}", self.service_type)?;
        writeln!(fmt, "\ttitle: {:?}", self.title)?;
        writeln!(fmt, "\tid: {}", self.id)?;
        writeln!(fmt, "\tdata: {:?}", self.data)
    }
}

impl FromStr for Permalink {
    type Err = PermalinkError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Permalink::parse_str(value)
    }
}

impl TryFrom<String> for Permalink {
    type Error = PermalinkError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Permalink::parse_str(value.as_str())
    }
}

impl TryFrom<Url> for Permalink {
    type Error = PermalinkError;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        Self::parse_url(value)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WellKnownCountry {
    CA,
    JP,
    KR,
    UK,
    US,
}

impl FromStr for WellKnownCountry {
    type Err = PermalinkError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "ca" => Ok(Self::CA),
            "jp" => Ok(Self::JP),
            "kr" => Ok(Self::KR),
            "uk" => Ok(Self::UK),
            "us" => Ok(Self::US),
            _ => Err(Self::Err::UnknownCountry(value.to_string())),
        }
    }
}

impl Display for WellKnownCountry {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(match self {
            Self::CA => "ca",
            Self::JP => "jp",
            Self::KR => "kr",
            Self::UK => "uk",
            Self::US => "us",
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WellKnownLanguage {
    EN,
    JA,
    KO,
}

impl Display for WellKnownLanguage {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(match self {
            Self::EN => "en",
            Self::JA => "ja",
            Self::KO => "ko",
        })
    }
}

impl From<WellKnownCountry> for WellKnownLanguage {
    fn from(country: WellKnownCountry) -> Self {
        well_known_language_from_country(country)
    }
}

pub fn well_known_country_from_origin(origin: String) -> Option<WellKnownCountry> {
    match normalize_origin(origin).as_str() {
        "https://www.daangn.com" => Some(WellKnownCountry::KR),
        "https://www.karrotmarket.com" => None,
        "https://ca.karrotmarket.com" => Some(WellKnownCountry::CA),
        "https://jp.karrotmarket.com" => Some(WellKnownCountry::JP),
        "https://uk.karrotmarket.com" => Some(WellKnownCountry::UK),
        "https://us.karrotmarket.com" => Some(WellKnownCountry::US),
        "https://kr.karrotmarket.com" => Some(WellKnownCountry::KR),
        _ => None,
    }
}

pub fn well_known_origin_from_country(country: WellKnownCountry) -> String {
    match country {
        WellKnownCountry::CA => "https://ca.karrotmarket.com".to_string(),
        WellKnownCountry::JP => "https://jp.karrotmarket.com".to_string(),
        WellKnownCountry::KR => "https://www.daangn.com".to_string(),
        WellKnownCountry::UK => "https://uk.karrotmarket.com".to_string(),
        WellKnownCountry::US => "https://us.karrotmarket.com".to_string(),
    }
}

pub fn well_known_language_from_country(country: WellKnownCountry) -> WellKnownLanguage {
    match country {
        WellKnownCountry::CA => WellKnownLanguage::EN,
        WellKnownCountry::JP => WellKnownLanguage::JA,
        WellKnownCountry::KR => WellKnownLanguage::KO,
        WellKnownCountry::UK => WellKnownLanguage::EN,
        WellKnownCountry::US => WellKnownLanguage::EN,
    }
}

fn normalize_origin(origin: String) -> String {
    match origin.as_str() {
        "https://daangn.com" => "https://www.daangn.com".to_string(),
        "https://karrotmarket.com" => "https://www.karrotmarket.com".to_string(),
        _ => origin,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_permalink() {
        let permalink = Permalink::parse_str("https://www.daangn.com/kr/app/당근마켓-대한민국-1등-동네-앱-id1018769995/").unwrap();
        assert_eq!(permalink.country, WellKnownCountry::KR);
        assert_eq!(permalink.language, WellKnownLanguage::KO);
        assert_eq!(permalink.service_type, "app".to_string());
        assert_eq!(permalink.title, Some("당근마켓-대한민국-1등-동네-앱".to_string()));
        assert_eq!(permalink.id, "id1018769995".to_string());
    }

    #[test]
    fn test_parse_valid_permalink_without_title() {
        let permalink = Permalink::parse_str("https://www.daangn.com/kr/app/id1018769995/").unwrap();
        assert_eq!(permalink.country, WellKnownCountry::KR);
        assert_eq!(permalink.language, WellKnownLanguage::KO);
        assert_eq!(permalink.service_type, "app".to_string());
        assert_eq!(permalink.title, None);
        assert_eq!(permalink.id, "id1018769995".to_string());
    }

    #[test]
    fn test_parse_invalid_url() {
        let result = Permalink::parse_str("invalid/kr/app/id1018769995/");
        assert!(matches!(result, Err(PermalinkError::InvalidUrl(_))));
    }

    #[test]
    fn test_parse_invalid_permalink() {
        let result = Permalink::parse_str("https://apps.apple.com/kr/app/%EB%8B%B9%EA%B7%BC%EB%A7%88%EC%BC%93/id1018769995");
        assert!(matches!(result, Err(PermalinkError::InvalidPermalink(_))));
    }

    #[test]
    fn test_parse_unknown_country() {
        let result = Permalink::parse_str("https://www.daangn.com/xx/app/id1018769995/");
        assert_eq!(result, Err(PermalinkError::UnknownCountry("xx".to_string())));
    }

    #[test]
    fn test_normalize() {
        let permalink = Permalink::parse_str("https://www.daangn.com/kr/app/당근마켓-대한민국-1등-동네-앱-id1018769995/").unwrap();
        assert_eq!(
            permalink.normalize(),
            "https://www.karrotmarket.com/kr/app/id1018769995/".to_string(),
        );
    }

    #[test]
    fn test_canonicalize() {
        let permalink = Permalink::parse_str("https://www.daangn.com/kr/app/id1018769995/").unwrap();
        assert_eq!(
            permalink.canonicalize("당근마켓-대한민국-1등-동네-앱"),
            "https://www.daangn.com/kr/app/%EB%8B%B9%EA%B7%BC%EB%A7%88%EC%BC%93-%EB%8C%80%ED%95%9C%EB%AF%BC%EA%B5%AD-1%EB%93%B1-%EB%8F%99%EB%84%A4-%EC%95%B1-id1018769995/".to_string(),
        );
    }
}
