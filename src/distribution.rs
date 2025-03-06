// src/distribution.rs
//! Random generation of user-friendly IDs

use rand::{Rng, distr::Distribution};

use crate::{UploadId, alphabet};

/// Distribution for generating IDs of specific length
///
/// # Example
/// ```
/// use human_friendly_ids::UploadIdDist;
/// use rand::{Rng, distr::Distribution};
///
/// let dist = UploadIdDist::<8>;
/// let id = dist.sample(&mut rand::thread_rng());
/// ```
#[derive(Debug, Clone)]
pub struct UploadIdDist<const N: usize>;

impl<const N: usize> UploadIdDist<N> {
    /// Create new distribution with compile-time length check
    #[must_use]
    pub const fn new() -> Self {
        assert!(N >= 3, "ID length must be at least 3 characters");
        Self
    }
}

impl<const N: usize> Default for UploadIdDist<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Distribution<UploadId> for UploadIdDist<N> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UploadId {
        debug_assert!(N >= 3, "ID length must be at least 3 characters");

        let mut body = String::with_capacity(N.saturating_sub(1));
        let mut last_char = None;

        while body.len() < N.saturating_sub(1) {
            let idx = rng.random_range(0..alphabet::GEN_ALPHABET.len());
            #[allow(clippy::indexing_slicing, reason = "index is generated within bounds")]
            let c = alphabet::GEN_ALPHABET[idx];
            // Avoid ambiguous sequences
            match (last_char, c) {
                (Some('r'), 'n') | (Some('v'), 'v') => {}
                // Don't end with 'r' or 'v', because the check-bit could create an ambiguous sequence
                (_, 'r' | 'v') if body.len() == N.saturating_sub(2) => {}
                _ => {
                    body.push(c);
                    last_char = Some(c);
                }
            }
        }

        let check_char = alphabet::calculate_check_char(&body)
            .expect("Generated body should be valid for check calculation");

        UploadId(format!("{}{}", body, check_char))
    }
}
