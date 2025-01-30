use crate::{cornucopia::queries::url_queries::{UrlBorrowed, UrlWithStatsBorrowed}, UrlMapping, UrlMappingWithStats};

impl From<UrlBorrowed<'_>> for UrlMapping {
    fn from(value: UrlBorrowed) -> Self {
        UrlMapping {
            id: value.id,
            short_code: value.short_code.into(),
            url: value.url.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

// impl Into<UrlMapping<'_>> for UrlBorrowed {
// }

impl From<UrlWithStatsBorrowed<'_>> for UrlMappingWithStats {
    fn from(value: UrlWithStatsBorrowed) -> Self {
        UrlMappingWithStats {
            id: value.id,
            short_code: value.short_code.into(),
            url: value.url.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            access_count: value.access_count as u64,
        }
    }
}
