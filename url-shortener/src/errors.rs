pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidUrl { url: String },
    CodeNotFound { code: String },
    DbError,
}
