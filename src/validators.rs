use lazy_static::lazy_static;
use regex::{self, Regex};
use validator::ValidationError;

pub fn validate_iso_8601(value: &str) -> Result<(), ValidationError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([\+-]?\d{4}(?!\d{2}\b))((-?)((0[1-9]|1[0-2])(\3([12]\d|0[1-9]|3[01]))?|W([0-4]\d|5[0-2])(-?[1-7])?|(00[1-9]|0[1-9]\d|[12]\d{2}|3([0-5]\d|6[1-6])))([T\s]((([01]\d|2[0-3])((:?)[0-5]\d)?|24\:?00)([\.,]\d+(?!:))?)?(\17[0-5]\d([\.,]\d+)?)?([zZ]|([\+-])([01]\d|2[0-3]):?([0-5]\d)?)?)?)?$").unwrap();
    }

    if RE.is_match(value) {
        Ok(())
    } else {
        Err(ValidationError::new("INVALID_ISO_8601"))
    }
}

pub fn validate_alpha_numeric(value: &str) -> Result<(), ValidationError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[A-Za-z0-9]+$").unwrap();
    }

    if RE.is_match(value) {
        Ok(())
    } else {
        Err(ValidationError::new("INVALID_ALPHA_NUMERIC"))
    }
}
