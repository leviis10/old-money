use validator::ValidationError;

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new(
            "Password must be at least 8 characters long",
        ));
    }

    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;
    let mut has_special = false;

    for ch in password.chars() {
        if ch.is_ascii_uppercase() {
            has_upper = true;
        } else if ch.is_ascii_lowercase() {
            has_lower = true;
        } else if ch.is_ascii_digit() {
            has_digit = true;
        } else if ch.is_ascii_punctuation() || ch.is_ascii_graphic() {
            has_special = true;
        }
    }

    if !has_upper {
        return Err(ValidationError::new(
            "Password must be at least have a uppercase character",
        ));
    }
    if !has_lower {
        return Err(ValidationError::new(
            "Password must be at least have a lowercase character",
        ));
    }
    if !has_digit {
        return Err(ValidationError::new(
            "Password must be at least have a digit",
        ));
    }
    if !has_special {
        return Err(ValidationError::new(
            "Password must be at least have a special character",
        ));
    }

    Ok(())
}
