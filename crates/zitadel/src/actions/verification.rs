//! Webhook signature verification for ZITADEL Actions v3

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::actions::WebhookError;

type HmacSha256 = Hmac<Sha256>;

/// Webhook signature verifier
///
/// Verifies HMAC-SHA256 signatures for ZITADEL webhook requests
#[derive(Clone)]
pub struct WebhookVerifier {
    secret: String,
    max_timestamp_age: Option<u64>,
}

impl WebhookVerifier {
    /// Create a new webhook verifier with the given secret
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: secret.into(),
            max_timestamp_age: Some(300), // 5 minutes default
        }
    }

    /// Set the maximum age for timestamp validation (in seconds)
    ///
    /// Set to None to disable timestamp validation
    pub fn with_max_timestamp_age(mut self, seconds: Option<u64>) -> Self {
        self.max_timestamp_age = seconds;
        self
    }

    /// Verify a webhook signature
    ///
    /// # Arguments
    ///
    /// * `body` - The raw request body bytes
    /// * `signature_header` - The signature header value in format "t=timestamp,v1=signature"
    pub fn verify(
        &self,
        body: &[u8],
        signature_header: &str,
    ) -> Result<(), WebhookError> {
        // Parse the signature header
        let (timestamp, signature) = self.parse_signature_header(signature_header)?;

        // Verify timestamp
        self.verify_timestamp(&timestamp)?;

        // Create HMAC with timestamp and body
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");

        // Include timestamp in the signature as per ZITADEL's implementation
        mac.update(timestamp.as_bytes());
        mac.update(b".");
        mac.update(body);

        // Verify signature
        let expected = mac.finalize();
        let expected_bytes = expected.into_bytes();

        // Decode provided signature from hex
        let provided_bytes = hex::decode(&signature)
            .map_err(|_| WebhookError::InvalidSignature)?;

        // Constant-time comparison of raw bytes
        if !constant_time_eq_bytes(&expected_bytes, &provided_bytes) {
            return Err(WebhookError::InvalidSignature);
        }

        Ok(())
    }

    /// Parse the signature header in format "t=timestamp,v1=signature"
    fn parse_signature_header(&self, header: &str) -> Result<(String, String), WebhookError> {
        let mut timestamp = None;
        let mut signature = None;

        for part in header.split(',') {
            if let Some((key, value)) = part.split_once('=') {
                match key.trim() {
                    "t" => timestamp = Some(value.to_string()),
                    "v1" => signature = Some(value.to_string()),
                    _ => {} // Ignore unknown keys
                }
            }
        }

        match (timestamp, signature) {
            (Some(ts), Some(sig)) => Ok((ts, sig)),
            _ => Err(WebhookError::InvalidSignature),
        }
    }

    /// Verify timestamp is within acceptable range
    fn verify_timestamp(&self, timestamp: &str) -> Result<(), WebhookError> {
        let Some(max_age) = self.max_timestamp_age else {
            return Ok(());
        };

        let ts = timestamp
            .parse::<u64>()
            .map_err(|_| WebhookError::InvalidTimestamp("Invalid timestamp format".to_string()))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| WebhookError::InvalidTimestamp("System time error".to_string()))?
            .as_secs();

        let age = now.saturating_sub(ts);

        if age > max_age {
            return Err(WebhookError::InvalidTimestamp(
                format!("Timestamp too old: {} seconds", age)
            ));
        }

        // Also check for future timestamps (with 60 second tolerance)
        if ts > now + 60 {
            return Err(WebhookError::InvalidTimestamp(
                "Timestamp is in the future".to_string()
            ));
        }

        Ok(())
    }
}

/// Constant-time string comparison
fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    constant_time_eq_bytes(a_bytes, b_bytes)
}

/// Constant-time byte slice comparison
fn constant_time_eq_bytes(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for i in 0..a.len() {
        result |= a[i] ^ b[i];
    }

    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_verification() {
        // Use current timestamp to avoid timestamp validation issues
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let timestamp = now.to_string();

        let verifier = WebhookVerifier::new("test-secret");
        let body = b"test body";

        // Generate expected signature with timestamp
        let mut mac = HmacSha256::new_from_slice(b"test-secret").unwrap();
        mac.update(timestamp.as_bytes());
        mac.update(b".");
        mac.update(body);
        let expected_sig = hex::encode(mac.finalize().into_bytes());

        // Format as ZITADEL signature header
        let signature_header = format!("t={},v1={}", timestamp, expected_sig);

        // Should verify correctly
        assert!(verifier.verify(body, &signature_header).is_ok());

        // Should fail with wrong signature
        let bad_header = format!("t={},v1=wrong", timestamp);
        assert!(verifier.verify(body, &bad_header).is_err());
    }

    #[test]
    fn test_signature_parsing() {
        let verifier = WebhookVerifier::new("test-secret");

        // Test valid format from the example
        let result = verifier.parse_signature_header("t=1756575078,v1=111328aa6b217e31d7f81deca0ca21c9f4f44e279396dfbd8bc3783f5db26bc8");
        assert!(result.is_ok());
        let (timestamp, signature) = result.unwrap();
        assert_eq!(timestamp, "1756575078");
        assert_eq!(signature, "111328aa6b217e31d7f81deca0ca21c9f4f44e279396dfbd8bc3783f5db26bc8");

        // Test invalid format
        assert!(verifier.parse_signature_header("invalid").is_err());
        assert!(verifier.parse_signature_header("t=123").is_err());
        assert!(verifier.parse_signature_header("v1=abc").is_err());
    }

    #[test]
    fn test_example_signature_format() {
        // Test that we can parse the exact format from the user's example
        let verifier = WebhookVerifier::new("webhook-secret").with_max_timestamp_age(None); // Disable timestamp validation for this test

        let signature_header = "t=1756575078,v1=111328aa6b217e31d7f81deca0ca21c9f4f44e279396dfbd8bc3783f5db26bc8";
        let result = verifier.parse_signature_header(signature_header);

        assert!(result.is_ok());
        let (timestamp, signature) = result.unwrap();
        assert_eq!(timestamp, "1756575078");
        assert_eq!(signature, "111328aa6b217e31d7f81deca0ca21c9f4f44e279396dfbd8bc3783f5db26bc8");
        assert_eq!(signature.len(), 64); // Should be 64 characters (32 bytes in hex)

        // Test that hex decoding works
        assert!(hex::decode(&signature).is_ok());
    }

    #[test]
    fn test_timestamp_validation() {
        let verifier = WebhookVerifier::new("test-secret")
            .with_max_timestamp_age(Some(300)); // 5 minutes

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Current timestamp should be valid
        assert!(verifier.verify_timestamp(&now.to_string()).is_ok());

        // Old timestamp should fail
        let old = now - 400; // 400 seconds ago
        assert!(verifier.verify_timestamp(&old.to_string()).is_err());

        // Future timestamp should fail
        let future = now + 120; // 2 minutes in future
        assert!(verifier.verify_timestamp(&future.to_string()).is_err());
    }
}
