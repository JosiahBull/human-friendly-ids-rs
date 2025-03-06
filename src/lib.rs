#![doc = include_str!("../README.md")]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::uninlined_format_args)]

pub mod alphabet;
pub mod distribution;
pub mod error;
pub mod id;

// Re-export main types for convenience
pub use distribution::UploadIdDist;

pub use crate::id::UploadId;

#[allow(
    clippy::all,
    clippy::pedantic,
    unused_must_use,
    reason = "It's a test, bro."
)]
#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use rand::{Rng, distr::Distribution};

    use super::*;
    use crate::alphabet::GEN_ALPHABET;

    #[test]
    fn assert_largest_id_is_fixed() {
        let largest = UploadId::max_length();
        assert_eq!(largest, 838_488_366_986_797_801); // Absurdly large number, but it's fixed.

        // Try and generate an id with a very large length, notably this will allocate a string
        // of this size.
        const TEST_SIZE: usize = 1024 * 1024; // 1mb

        let mut rng = rand::rng();
        let id = UploadIdDist::<TEST_SIZE>.sample(&mut rng);
        assert_eq!(id.as_str().len(), TEST_SIZE);

        // Decode and re-encode the id.
        let id_str = id.to_string();
        let id_decoded: UploadId = id_str.parse().expect("Failed to decode UploadId");

        assert_eq!(id_decoded.to_string(), id_str);
    }

    #[test]
    fn test_decode() {
        let test_string = String::from("wcfytxww4opin4jmjjes4ccfd");
        let decoded = UploadId::try_from(test_string).expect("Failed to decode UploadId");
        assert_eq!(
            decoded.as_str(),
            "wcfytxww4opin4jmjjes4ccfd",
            "decoded value should be equal to input string"
        );
    }

    #[test]
    fn fuzz_generated_ids() {
        for _ in 0_u64..10_000_u64 {
            let mut rng = rand::rng();
            let id = UploadIdDist::<25>.sample(&mut rng);
            println!("{}", id);
            assert_eq!(id.as_str().len(), 25);

            // Assert that serializing and deserializing the id doesn't change it.
            let id_str = id.to_string();
            let id = UploadId::try_from(id_str.clone()).expect("Failed to decode UploadId");
            assert_eq!(id.to_string(), id_str);
        }
    }

    #[test]
    fn fuzz_gen_alphabet_strings() {
        let mut rng = rand::rng();
        for _ in 0..100_000_u64 {
            // Generate a random string of characters from 2 to 25 characters long.
            let string = (0..rng.random_range(2..25))
                .map(|_| GEN_ALPHABET[rng.random_range(0..GEN_ALPHABET.len())])
                .collect::<String>();

            // Try and decode it - should not panic.
            UploadId::try_from(string.clone());
        }
    }

    #[test]
    fn fuzz_random_strings() {
        let mut rng = rand::rng();
        for _ in 0..100_000_u64 {
            // Generate a random string of characters from 2 to 25 characters long.
            let string = (0..rng.random_range(2..25))
                .map(|_| rng.random_range(0..=255) as u8 as char)
                .collect::<String>();

            // Try and decode it - should not panic.
            UploadId::try_from(string.clone());
        }
    }

    #[test]
    fn test_invalid_chars_error() {
        let id = "abc123".to_string();
        let result = UploadId::try_from(id);
        assert!(result.is_err());
        let err = result.expect_err("Should fail due to invalid characters");
        assert_eq!(err.to_string(), "Invalid check bit");
    }

    #[test]
    fn test_invalid_check_bit_error() {
        let invalid_id = String::from("abbsyhbbb4tyxnnmrtjx4crom");
        let result = UploadId::try_from(invalid_id);
        assert!(result.is_err());
        let err = result.expect_err("Should fail due to invalid check-bit");
        assert_eq!(err.to_string(), "Invalid check bit");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_roundtrip() {
        let id = UploadId::try_from("wcfytxww4opin4jmjjes4ccfd".to_string())
            .expect("Failed to decode UploadId");
        let serialized = serde_json::to_string(&id).expect("Failed to serialize UploadId");
        let deserialized: UploadId =
            serde_json::from_str(&serialized).expect("Failed to deserialize UploadId");
        assert_eq!(id, deserialized);
    }

    #[test]
    fn test_too_short_error() {
        let invalid_id = String::from("aa");
        let result = UploadId::try_from(invalid_id);
        assert!(result.is_err());
        let err = result.expect_err("Should fail due to invalid check-bit");
        assert_eq!(err.to_string(), "ID length too short, minimum 3 characters");
    }

    #[test]
    fn test_weird_unicode() {
        let invalid_id = String::from("🦀🦀🦀");
        let result = UploadId::try_from(invalid_id);
        assert!(result.is_err());
        let err = result.expect_err("Should fail due to invalid characters");
        assert_eq!(err.to_string(), "Invalid character in ID");
    }

    #[test]
    fn test_invalid_chars() {
        let invalid_id = String::from("¡¢£¤¥¦§¨©ª«¬®¯°±²³´µ¶·¸¹º»¼½¾¿gg");
        let result = UploadId::try_from(invalid_id);
        assert!(result.is_err());
        let err = result.expect_err("Should fail due to invalid characters");
        assert_eq!(err.to_string(), "Invalid character in ID");
    }
}
