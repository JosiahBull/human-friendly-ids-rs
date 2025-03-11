// src/alphabet.rs
//! Character handling and validation for user-friendly IDs

use crate::error::IdError;

/// Primary generation alphabet (23 characters)
pub const GEN_ALPHABET: [char; 23] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'r', 's', 't', 'w', 'x',
    'y', '3', '4', 'v',
];

/// Check bit alphabet (23 characters)
pub const CHECK_ALPHABET: [char; 23] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'r', 's', 't', 'w', 'x',
    'y', '3', '4', 'v',
];

/// LUT for check alphabet character lookup
#[allow(
    clippy::indexing_slicing,
    clippy::cast_possible_truncation,
    reason = "const fn will fail early"
)]
const CHECK_LOOKUP: [u8; 256] = {
    let mut lookup = [0; 256];
    let mut i = 0;
    while i < CHECK_ALPHABET.len() {
        if i >= u8::MAX as usize {
            panic!("Check alphabet is too large for lookup table");
        } else {
            lookup[CHECK_ALPHABET[i] as usize] = i as u8;
            i += 1;
        }
    }
    lookup
};

/// Normalize potentially ambiguous characters
#[must_use]
pub const fn normalize_char(c: char) -> char {
    match c {
        '0' => 'o',
        '1' | 'l' | '7' => 'i',
        'z' | '5' | '2' => 's',
        'u' => 'v',
        '6' | '8' | '9' | 'g' | 'q' => 'b',
        c => c,
    }
}

/// Normalize and replace ambiguous sequences in a string
pub fn normalize_string(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(normalize_char)
        .collect::<String>()
        .replace("rn", "m")
        .replace("vv", "w")
}

/// Validate a character against the check alphabet
///
/// ## Errors
///
/// - [`IdError::InvalidCharacter`] if the character is not in the check alphabet
pub fn validate_char(c: char) -> Result<(), IdError> {
    if CHECK_ALPHABET.contains(&c) {
        Ok(())
    } else {
        Err(IdError::InvalidCharacter)
    }
}

/// Calculate expected check character for a string
///
/// ## Errors
///
/// - [`IdError::InvalidCharacter`] if a character is not in the check alphabet
/// - [`IdError::InvalidCheckBit`] if the check bit calculation fails
pub fn calculate_check_char(s: &str) -> Result<char, IdError> {
    const _: () = assert!(
        std::mem::size_of::<usize>() == 8,
        "This function is only safe on 64-bit platforms"
    );

    let sum: u64 = s
        .chars()
        .map(|c| {
            CHECK_LOOKUP
                .get(c as usize)
                .copied()
                .ok_or(IdError::InvalidCharacter)
                .map(u64::from)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();

    #[allow(
        clippy::cast_possible_truncation,
        reason = "u64 -> usize is safe, and we check that this is only used on 64-bit platforms."
    )]
    let index = (sum
        .checked_rem(CHECK_ALPHABET.len() as u64)
        .ok_or(IdError::InvalidCheckBit)?) as usize;
    CHECK_ALPHABET
        .get(index)
        .copied()
        .ok_or(IdError::InvalidCheckBit)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde_json::json;

    use crate::{Id, alphabet::normalize_string};

    #[test]
    fn snapshot_lut() {
        // A silly test to satisfy cargo mutants.
        insta::assert_debug_snapshot!(crate::alphabet::CHECK_LOOKUP);
    }

    #[test]
    fn edge_case_1() {
        let id = String::from("9qg6G8B2Z5SIl170O");
        let check = crate::alphabet::calculate_check_char(&normalize_string(&id));
        let formatted_id = Id::from_str(&format!("{}{}", id, check.clone().unwrap())).unwrap();
        insta::assert_json_snapshot!(json!({
            "id": id,
            "check": check.unwrap(),
            "formatted_id": formatted_id
        }));
    }

    #[test]
    fn edge_case_2() {
        let id = String::from("Il717il");
        let check = crate::alphabet::calculate_check_char(&normalize_string(&id));
        let formatted_id = Id::from_str(&format!("{}{}", id, check.clone().unwrap())).unwrap();
        insta::assert_json_snapshot!(json!({
            "id": id,
            "check": check.unwrap(),
            "formatted_id": formatted_id
        }));
    }

    #[test]
    fn edge_case_3() {
        let id = String::from("5s25zs5");
        let check = crate::alphabet::calculate_check_char(&normalize_string(&id));
        let formatted_id = Id::from_str(&format!("{}{}", id, check.clone().unwrap())).unwrap();
        insta::assert_json_snapshot!(json!({
            "id": id,
            "check": check.unwrap(),
            "formatted_id": formatted_id
        }));
    }

    #[test]
    fn edge_case_4() {
        let id = String::from("6G6GGG6");
        let check = crate::alphabet::calculate_check_char(&normalize_string(&id));
        let formatted_id = Id::from_str(&format!("{}{}", id, check.clone().unwrap())).unwrap();
        insta::assert_json_snapshot!(json!({
            "id": id,
            "check": check.unwrap(),
            "formatted_id": formatted_id
        }));
    }
    #[test]
    fn edge_case_5() {
        let id = String::from("0oO0OooO");
        let check = crate::alphabet::calculate_check_char(&normalize_string(&id));
        let formatted_id = Id::from_str(&format!("{}{}", id, check.clone().unwrap())).unwrap();
        insta::assert_json_snapshot!(json!({
            "id": id,
            "check": check.unwrap(),
            "formatted_id": formatted_id
        }));
    }
    #[test]
    fn edge_case_6() {
        let id = String::from("rnmrnmrn");
        let check = crate::alphabet::calculate_check_char(&normalize_string(&id));
        let formatted_id = Id::from_str(&format!("{}{}", id, check.clone().unwrap())).unwrap();
        insta::assert_json_snapshot!(json!({
            "id": id,
            "check": check.unwrap(),
            "formatted_id": formatted_id
        }));
    }
    #[test]
    fn edge_case_7() {
        let id = String::from("vuuvvnwvvwv");
        let check = crate::alphabet::calculate_check_char(&normalize_string(&id));
        let formatted_id = Id::from_str(&format!("{}{}", id, check.clone().unwrap())).unwrap();
        insta::assert_json_snapshot!(json!({
            "id": id,
            "check": check.unwrap(),
            "formatted_id": formatted_id
        }));
    }
    #[test]
    fn edge_case_8() {
        // audibly ambiguous id.
        let id = String::from("bbbpbpb");
        let check = crate::alphabet::calculate_check_char(&normalize_string(&id));
        let formatted_id = Id::from_str(&format!("{}{}", id, check.clone().unwrap())).unwrap();
        insta::assert_json_snapshot!(json!({
            "id": id,
            "check": check.unwrap(),
            "formatted_id": formatted_id
        }));
    }
}
