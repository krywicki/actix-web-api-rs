use mongodb::bson::oid::ObjectId;
use validator;
use validator::ValidationError;

pub fn validate_email_or_object_id(value: &String) -> Result<(), ValidationError> {
    if validator::validate_email(value) {
        Ok(())
    } else if ObjectId::parse_str(value).is_ok() {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid email or ObjectId"))
    }
}
