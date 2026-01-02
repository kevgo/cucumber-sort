mod finding;
mod result;
mod user_error;

pub use finding::{Finding, Issue, deduplicate_unsorted_lines};
pub use result::AppResult;
pub use user_error::UserError;
