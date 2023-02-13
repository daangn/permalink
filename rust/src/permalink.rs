use std::{str::FromStr, fmt::Display};

use lazy_static::lazy_static;
use percent_encoding::{percent_decode_str, utf8_percent_encode};
use regex::Regex;
use url::{Url, Origin};

use crate::cjk_slug;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum PermalinkError {
    #[error("invalid url")]
    InvalidUrl(Box<url::ParseError>),
    #[error("invalid permalink")]
    InvalidPermalink,
    #[error("unknown country code `{0}`")]
    UnknownCountry(String),
}

impl From<url::ParseError> for PermalinkError {
    fn from(err: url::ParseError) -> Self {
        Self::InvalidUrl(Box::new(err))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permalink {
    pub country: WellKnownCountry,
    pub default_language: String,
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
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"/(?P<country>[a-zA-Z]{2})/(?P<service_type>[a-z\-]{3,})/(?P<slug>((?P<title>((([a-z0-9]|%[0-9A-F]{2})+)\-?)+?)\-)?(?P<id>[a-zA-Z0-9]{8,}))(/(?P<data>[a-zA-Z0-9\-_]+))?/?",
            ).unwrap();
        }

        let pathname = url.path();

        if let Some(caps) = RE.captures(pathname) {
            let country = match url.origin().to_well_known_country() {
                Some(country) => country,
                None => {
                    let value = caps.name("country").unwrap().as_str();
                    value.to_well_known_country()
                        .ok_or_else(|| PermalinkError::UnknownCountry(value.to_string()))?
                },
            };

            let default_language = country.default_language();

            let service_type = caps.name("service_type").unwrap().as_str().to_string();

            let title = caps.name("title")
                .map(|m| percent_decode_str(m.as_str()).decode_utf8().unwrap().to_string());

            let id = caps.name("id").unwrap().as_str().to_string();

            let data = caps.name("data")
                .map(|m| m.as_str().to_string());

            Ok(Permalink {
                country,
                default_language,
                service_type,
                title,
                id,
                data,
            })
        } else {
            Err(PermalinkError::InvalidPermalink)
        }
    }

    pub fn normalize(&self) -> String {
        format!(
            "{}/{}/{}/{}/",
            "https://www.karrotmarket.com",
            self.country,
            self.service_type,
            self.id,
        )
    }

    pub fn canonicalize(&self, title: &str) -> String {
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

        let origin = match self.country {
            WellKnownCountry::CA => "https://ca.karrotmarket.com".to_string(),
            WellKnownCountry::JP => "https://jp.karrotmarket.com".to_string(),
            WellKnownCountry::KR => "https://www.daangn.com".to_string(),
            WellKnownCountry::UK => "https://uk.karrotmarket.com".to_string(),
            WellKnownCountry::US => "https://us.karrotmarket.com".to_string(),
        };
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

impl Display for Permalink {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(fmt, "permalink")?;
        writeln!(fmt, "\tcountry: {}", self.country)?;
        writeln!(fmt, "\tdefault_language: {}", self.default_language)?;
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
            "ca" | "CA" | "cA" | "Ca" => Ok(Self::CA),
            "jp" | "JP" | "jP" | "Jp" => Ok(Self::JP),
            "kr" | "KR" | "kR" | "Kr"  => Ok(Self::KR),
            "uk" | "UK" | "uK" | "Uk" => Ok(Self::UK),
            "us" | "US" | "uS" | "Us" => Ok(Self::US),
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

trait ToWellKnownCountry {
    fn to_well_known_country(&self) -> Option<WellKnownCountry>;
}

impl ToWellKnownCountry for str {
    fn to_well_known_country(&self) -> Option<WellKnownCountry> {
        WellKnownCountry::from_str(self).ok()
    }
}

impl ToWellKnownCountry for String {
    fn to_well_known_country(&self) -> Option<WellKnownCountry> {
        WellKnownCountry::from_str(self.as_str()).ok()
    }
}

impl ToWellKnownCountry for Origin {
    fn to_well_known_country(&self) -> Option<WellKnownCountry> {
        let origin = self.ascii_serialization();
        let origin = match origin.as_str() {
            "https://daangn.com" => "https://www.daangn.com",
            "https://karrotmarket.com" => "https://www.karrotmarket.com",
            _ => origin.as_str(),
        };
        match origin {
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
}

impl WellKnownCountry {
    fn default_language(&self) -> String {
        match self {
            Self::CA => "en".to_string(),
            Self::JP => "ja".to_string(),
            Self::KR => "ko".to_string(),
            Self::UK => "en".to_string(),
            Self::US => "en".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_permalink() {
        let permalink = Permalink::parse_str("https://www.daangn.com/kr/app/당근마켓-대한민국-1등-동네-앱-id1018769995/").unwrap();
        assert_eq!(permalink.country, WellKnownCountry::KR);
        assert_eq!(permalink.default_language, "ko".to_string());
        assert_eq!(permalink.service_type, "app".to_string());
        assert_eq!(permalink.title, Some("당근마켓-대한민국-1등-동네-앱".to_string()));
        assert_eq!(permalink.id, "id1018769995".to_string());
    }

    #[test]
    fn test_parse_valid_permalink_without_trailing_slash() {
        let permalink = Permalink::parse_str("https://www.daangn.com/kr/app/당근마켓-대한민국-1등-동네-앱-id1018769995").unwrap();
        assert_eq!(permalink.country, WellKnownCountry::KR);
        assert_eq!(permalink.default_language, "ko".to_string());
        assert_eq!(permalink.service_type, "app".to_string());
        assert_eq!(permalink.title, Some("당근마켓-대한민국-1등-동네-앱".to_string()));
        assert_eq!(permalink.id, "id1018769995".to_string());
    }

    #[test]
    fn test_parse_valid_permalink_without_title() {
        let permalink = Permalink::parse_str("https://www.daangn.com/kr/app/id1018769995/").unwrap();
        assert_eq!(permalink.country, WellKnownCountry::KR);
        assert_eq!(permalink.default_language, "ko".to_string());
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
        assert!(matches!(result, Err(PermalinkError::InvalidPermalink)));
    }

    #[test]
    fn test_parse_well_known_host() {
        let permalink = Permalink::parse_str("https://www.daangn.com/ca/app/id1018769995/").unwrap();
        assert_eq!(permalink.country, WellKnownCountry::KR);
        assert_eq!(permalink.default_language, "ko".to_string());
        assert_eq!(permalink.service_type, "app".to_string());
        assert_eq!(permalink.title, None);
        assert_eq!(permalink.id, "id1018769995".to_string());
    }

    #[test]
    fn test_parse_country_case_insensitive() {
        let permalink = Permalink::parse_str("https://www.daangn.com/KR/app/id1018769995/").unwrap();
        assert_eq!(permalink.country, WellKnownCountry::KR);
        assert_eq!(permalink.default_language, "ko".to_string());
        assert_eq!(permalink.service_type, "app".to_string());
        assert_eq!(permalink.title, None);
        assert_eq!(permalink.id, "id1018769995".to_string());
    }

    #[test]
    fn test_parse_unknown_country() {
        let result = Permalink::parse_str("http://localhost/xx/app/id1018769995/");
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
