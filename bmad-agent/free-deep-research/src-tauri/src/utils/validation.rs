use crate::error::{AppError, AppResult};
use crate::models::{ServiceProvider, ApiKeyStatus, ResearchMethodology, OutputFormat};
use url::Url;
use uuid::Uuid;

/// Validate an API key string
pub fn validate_api_key(api_key: &str) -> AppResult<()> {
    if api_key.is_empty() {
        return Err(AppError::validation("api_key", "API key cannot be empty"));
    }
    
    if api_key.len() < 10 {
        return Err(AppError::validation("api_key", "API key is too short"));
    }
    
    if api_key.len() > 500 {
        return Err(AppError::validation("api_key", "API key is too long"));
    }
    
    // Check for common invalid characters
    if api_key.contains('\n') || api_key.contains('\r') || api_key.contains('\t') {
        return Err(AppError::validation("api_key", "API key contains invalid characters"));
    }
    
    Ok(())
}

/// Validate a service provider
pub fn validate_service_provider(service: &ServiceProvider) -> AppResult<()> {
    // All enum variants are valid
    Ok(())
}

/// Validate an API key name
pub fn validate_api_key_name(name: &str) -> AppResult<()> {
    if name.is_empty() {
        return Err(AppError::validation("name", "Name cannot be empty"));
    }
    
    if name.len() > 100 {
        return Err(AppError::validation("name", "Name is too long (max 100 characters)"));
    }
    
    // Check for invalid characters
    if name.contains('\n') || name.contains('\r') || name.contains('\t') {
        return Err(AppError::validation("name", "Name contains invalid characters"));
    }
    
    Ok(())
}

/// Validate a rate limit value
pub fn validate_rate_limit(rate_limit: u32) -> AppResult<()> {
    if rate_limit == 0 {
        return Err(AppError::validation("rate_limit", "Rate limit must be greater than 0"));
    }
    
    if rate_limit > 1_000_000 {
        return Err(AppError::validation("rate_limit", "Rate limit is too high"));
    }
    
    Ok(())
}

/// Validate a UUID string
pub fn validate_uuid(uuid_str: &str) -> AppResult<Uuid> {
    Uuid::parse_str(uuid_str)
        .map_err(|_| AppError::validation("uuid", "Invalid UUID format"))
}

/// Validate a research query
pub fn validate_research_query(query: &str) -> AppResult<()> {
    if query.is_empty() {
        return Err(AppError::validation("query", "Research query cannot be empty"));
    }
    
    if query.len() < 3 {
        return Err(AppError::validation("query", "Research query is too short (minimum 3 characters)"));
    }
    
    if query.len() > 10_000 {
        return Err(AppError::validation("query", "Research query is too long (maximum 10,000 characters)"));
    }
    
    Ok(())
}

/// Validate a workflow name
pub fn validate_workflow_name(name: &str) -> AppResult<()> {
    if name.is_empty() {
        return Err(AppError::validation("name", "Workflow name cannot be empty"));
    }
    
    if name.len() > 200 {
        return Err(AppError::validation("name", "Workflow name is too long (max 200 characters)"));
    }
    
    Ok(())
}

/// Validate max iterations for research
pub fn validate_max_iterations(max_iterations: u32) -> AppResult<()> {
    if max_iterations == 0 {
        return Err(AppError::validation("max_iterations", "Max iterations must be greater than 0"));
    }
    
    if max_iterations > 100 {
        return Err(AppError::validation("max_iterations", "Max iterations is too high (maximum 100)"));
    }
    
    Ok(())
}

/// Validate research methodology
pub fn validate_research_methodology(methodology: &ResearchMethodology) -> AppResult<()> {
    // All enum variants are valid
    Ok(())
}

/// Validate output format
pub fn validate_output_format(format: &OutputFormat) -> AppResult<()> {
    // All enum variants are valid
    Ok(())
}

/// Validate a URL string
pub fn validate_url(url_str: &str) -> AppResult<Url> {
    Url::parse(url_str)
        .map_err(|_| AppError::validation("url", "Invalid URL format"))
}

/// Validate an email address (basic validation)
pub fn validate_email(email: &str) -> AppResult<()> {
    if email.is_empty() {
        return Err(AppError::validation("email", "Email cannot be empty"));
    }
    
    if !email.contains('@') {
        return Err(AppError::validation("email", "Invalid email format"));
    }
    
    if email.len() > 254 {
        return Err(AppError::validation("email", "Email is too long"));
    }
    
    Ok(())
}

/// Validate a password strength
pub fn validate_password_strength(password: &str) -> AppResult<()> {
    if password.len() < 8 {
        return Err(AppError::validation("password", "Password must be at least 8 characters long"));
    }
    
    if password.len() > 128 {
        return Err(AppError::validation("password", "Password is too long (max 128 characters)"));
    }
    
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
    
    if !has_lowercase {
        return Err(AppError::validation("password", "Password must contain at least one lowercase letter"));
    }
    
    if !has_uppercase {
        return Err(AppError::validation("password", "Password must contain at least one uppercase letter"));
    }
    
    if !has_digit {
        return Err(AppError::validation("password", "Password must contain at least one digit"));
    }
    
    if !has_special {
        return Err(AppError::validation("password", "Password must contain at least one special character"));
    }
    
    Ok(())
}

/// Validate a file path
pub fn validate_file_path(path: &str) -> AppResult<()> {
    if path.is_empty() {
        return Err(AppError::validation("path", "File path cannot be empty"));
    }
    
    if path.len() > 4096 {
        return Err(AppError::validation("path", "File path is too long"));
    }
    
    // Check for invalid characters (basic check)
    if path.contains('\0') {
        return Err(AppError::validation("path", "File path contains null character"));
    }
    
    Ok(())
}

/// Validate a percentage value (0.0 - 100.0)
pub fn validate_percentage(value: f64) -> AppResult<()> {
    if value < 0.0 || value > 100.0 {
        return Err(AppError::validation("percentage", "Percentage must be between 0.0 and 100.0"));
    }
    
    Ok(())
}

/// Validate a positive integer
pub fn validate_positive_integer(value: u32, field_name: &str) -> AppResult<()> {
    if value == 0 {
        return Err(AppError::validation(field_name, "Value must be greater than 0"));
    }
    
    Ok(())
}

/// Validate a non-negative integer
pub fn validate_non_negative_integer(value: u32, field_name: &str) -> AppResult<()> {
    // u32 is always non-negative, so this always passes
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_api_key() {
        assert!(validate_api_key("").is_err());
        assert!(validate_api_key("short").is_err());
        assert!(validate_api_key("valid_api_key_123").is_ok());
        assert!(validate_api_key("key_with\nnewline").is_err());
    }

    #[test]
    fn test_validate_api_key_name() {
        assert!(validate_api_key_name("").is_err());
        assert!(validate_api_key_name("Valid Name").is_ok());
        assert!(validate_api_key_name("name_with\ttab").is_err());
    }

    #[test]
    fn test_validate_rate_limit() {
        assert!(validate_rate_limit(0).is_err());
        assert!(validate_rate_limit(100).is_ok());
        assert!(validate_rate_limit(2_000_000).is_err());
    }

    #[test]
    fn test_validate_research_query() {
        assert!(validate_research_query("").is_err());
        assert!(validate_research_query("AI").is_err());
        assert!(validate_research_query("What is artificial intelligence?").is_ok());
    }

    #[test]
    fn test_validate_password_strength() {
        assert!(validate_password_strength("weak").is_err());
        assert!(validate_password_strength("StrongPass123!").is_ok());
        assert!(validate_password_strength("NoSpecial123").is_err());
        assert!(validate_password_strength("nouppercas123!").is_err());
    }

    #[test]
    fn test_validate_percentage() {
        assert!(validate_percentage(-1.0).is_err());
        assert!(validate_percentage(50.0).is_ok());
        assert!(validate_percentage(101.0).is_err());
    }
}
