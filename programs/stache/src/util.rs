
use crate::error::StacheError;
use anchor_lang::prelude::*;

// checks that a given string contains only lowercase letters and numbers
pub fn is_valid_name(s: &str, require_lower: bool) -> bool {
    let len = s.len();
    if len > 32 || len < 2 {
        return false;
    }
    s.chars().all(|c| !c.is_whitespace()  && (c.is_ascii_lowercase() || c.is_ascii_digit() || (!require_lower && c.is_ascii_uppercase())))
}

// adds an index to a list, returning the index that got added
pub fn add_index(list: &mut Vec<u8>, max: usize, next_index: &mut u8) -> Result<u8> {
    // check that we've got room
    require!(usize::from(list.len()) < max, StacheError::HitLimit);

    // todo: handle wrapping properly
    let mut index: u8 = *next_index;
    if index + 1 == u8::MAX {
        *next_index = 2;
        index = 1;
    } else {
        *next_index += 1;
    }

    list.push(index);
    return Ok(index);
}
