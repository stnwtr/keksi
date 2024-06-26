use axum_extra::extract::cookie::{Cookie, Expiration, SameSite};
use serde::{Deserialize, Serialize};
use time::serde::timestamp;
use time::{Duration, OffsetDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keksi {
    name: String,
    value: String,
    expires: Option<KeksiExpiration>,
    max_age: Option<i64>,
    domain: Option<String>,
    path: Option<String>,
    secure: Option<bool>,
    http_only: Option<bool>,
    same_site: Option<KeksiSameSite>,
    partitioned: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeksiExpiration {
    #[serde(rename = "date_time")]
    DateTime(#[serde(with = "timestamp")] OffsetDateTime),
    #[serde(rename = "session")]
    Session,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeksiSameSite {
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "lax")]
    Lax,
    #[serde(rename = "none")]
    None,
}

impl<'a> From<Keksi> for Cookie<'a> {
    fn from(value: Keksi) -> Self {
        let mut cookie = Cookie::new(value.name, value.value);

        if let Some(expiration) = value.expires {
            cookie.set_expires(expiration);
        }

        if let Some(max_age) = value.max_age {
            cookie.set_max_age(Duration::seconds(max_age));
        }

        if let Some(domain) = value.domain {
            cookie.set_domain(domain);
        }

        if let Some(path) = value.path {
            cookie.set_path(path);
        }

        if let Some(secure) = value.secure {
            cookie.set_secure(secure);
        }

        if let Some(http_only) = value.http_only {
            cookie.set_http_only(http_only);
        }

        if let Some(same_site) = value.same_site {
            cookie.set_same_site(Into::<SameSite>::into(same_site));
        }

        if let Some(partitioned) = value.partitioned {
            cookie.set_partitioned(partitioned);
        }

        cookie
    }
}

impl<'a> From<&Cookie<'a>> for Keksi {
    fn from(value: &Cookie) -> Self {
        Keksi {
            name: value.name().to_string(),
            value: value.value().to_string(),
            expires: value.expires().map(|expiration| expiration.into()),
            max_age: value
                .max_age()
                .map(|duration| duration.as_seconds_f32() as i64),
            domain: value.domain().map(|domain| domain.to_string()),
            path: value.path().map(|path| path.to_string()),
            secure: value.secure(),
            http_only: value.http_only(),
            same_site: value.same_site().map(|same_site| same_site.into()),
            partitioned: value.partitioned(),
        }
    }
}

impl Keksi {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl From<KeksiExpiration> for Expiration {
    fn from(value: KeksiExpiration) -> Self {
        match value {
            KeksiExpiration::DateTime(date_time) => Expiration::DateTime(date_time),
            KeksiExpiration::Session => Expiration::Session,
        }
    }
}

impl From<Expiration> for KeksiExpiration {
    fn from(value: Expiration) -> Self {
        match value {
            Expiration::DateTime(date_time) => KeksiExpiration::DateTime(date_time),
            Expiration::Session => KeksiExpiration::Session,
        }
    }
}

impl From<KeksiSameSite> for SameSite {
    fn from(value: KeksiSameSite) -> Self {
        match value {
            KeksiSameSite::Strict => SameSite::Strict,
            KeksiSameSite::Lax => SameSite::Lax,
            KeksiSameSite::None => SameSite::None,
        }
    }
}

impl From<SameSite> for KeksiSameSite {
    fn from(value: SameSite) -> Self {
        match value {
            SameSite::Strict => KeksiSameSite::Strict,
            SameSite::Lax => KeksiSameSite::Lax,
            SameSite::None => KeksiSameSite::None,
        }
    }
}
