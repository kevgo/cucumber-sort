use crate::errors::UserError;

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call site
pub type Result<T> = core::result::Result<T, UserError>;
