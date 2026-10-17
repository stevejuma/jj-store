use super::*;
pub(super) fn review_error(error: ReviewError) -> Error {
    match error {
        ReviewError::Limit => Error::Limit("review model"),
        ReviewError::Invalid(_) | ReviewError::Encoding(_) => Error::Corrupt("review model"),
    }
}
