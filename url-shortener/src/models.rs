use crate::{utils::now, Error, Result};
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use url::Url as ExtUrl;

use rand::{distributions::Alphanumeric, prelude::*};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct ShortCode(String);

const CODE_LEN: u32 = 6;

impl ShortCode {
    pub fn generate() -> Self {
        // Im aware I could use uuid, but wanted to use my own impl here
        let mut rng = rand::thread_rng();
        let code: String = (0..CODE_LEN)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect();
        let code = code.to_lowercase();
        Self(code)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ShortCode {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<ShortCode> for String {
    fn from(value: ShortCode) -> Self {
        value.0
    }
}

fn total_codes(code_len: u32) -> u32 {
    u32::pow(36, code_len)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Url(String);

impl Url {
    pub fn new(url: String) -> Result<Self> {
        if Url::valid_url(&url) == false {
            return Err(Error::InvalidUrl { url });
        }
        Ok(Self(url))
    }

    // somewhat simple url validation, but good enough for this specific project
    fn valid_url(url: &String) -> bool {
        match ExtUrl::parse(&url) {
            Ok(ext_url) => {
                let scheme = ext_url.scheme();
                scheme == "http" || scheme == "https"
            }
            Err(_) => false,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_string(self) -> String {
        self.0
    }
}

impl From<&str> for Url {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<Url> for String {
    fn from(value: Url) -> Self {
        value.0
    }
}

#[derive(Debug, Serialize)]
pub struct UrlMapping {
    pub id: Uuid,
    pub short_code: ShortCode,
    pub url: Url,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct UrlMappingWithStats {
    pub id: Uuid,
    pub short_code: ShortCode,
    pub url: Url,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub access_count: u64,
}

impl UrlMapping {
    pub fn new(id: Uuid, url: Url, short_code: ShortCode) -> Self {
        Self {
            id,
            url,
            short_code,
            created_at: now(),
            updated_at: now(),
        }
    }
}

pub struct UrlAccessCount {
    id: Uuid,
    access_count: u64,
}

impl UrlAccessCount {
    pub fn increment_count(&mut self) {
        self.access_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_url() {
        let result = Url::new("https://www.crates.io".to_string());
        assert!(result.is_ok());

        let bad_url = String::from("malito:cta@acme.com");
        let result = Url::new(bad_url);
        assert!(matches!(result, Err(Error::InvalidUrl { url: _ })));

        let bad_url = String::from("http://no.sp aces");
        let result = Url::new(bad_url);
        assert!(matches!(result, Err(Error::InvalidUrl { url: _ })));

        let bad_url = String::from("http://no.bad.ch<rs");
        let result = Url::new(bad_url);
        assert!(matches!(result, Err(Error::InvalidUrl { url: _ })));

        let bad_url = String::from("crates.io");
        let result = Url::new(bad_url);
        assert!(matches!(result, Err(Error::InvalidUrl { url: _ })));
    }

    #[test]
    fn generate_random_short_codes() {
        println!("Total codes len {}: {}", CODE_LEN, total_codes(CODE_LEN));
        let mut short_codes: Vec<ShortCode> =
            (0..).take(100).map(|_| ShortCode::generate()).collect();
        short_codes.sort();
        let mut repeated = false;
        let mut last_code: Option<ShortCode> = None;
        for code in short_codes.iter() {
            if let Some(lc) = last_code {
                if lc == *code {
                    repeated = true;
                    println!("Faulty at: {} = {}", lc.0, code.0);
                    break;
                }
            }
            last_code = Some(code.clone());
        }
        assert!(!repeated, "Error: generate creates repeated code");
    }
}
