use std::str;
use std::fs::File;
use crate::error::{self, Result};

pub fn detect(val: i32) -> Result<i32> {
    if val == 0 {
        File::open("I_believe_this_does_not_exist.")
            .and_then(|_| Ok(0))
            .map_err(error::Error::from)
    } else if val == 1 {
        Err(error::new_value_error("keyword"))
    } else if val == 2 {
        Err(error::new_format_error())
    } else if val == 3 {
        let sparkle_heart = vec![0, 159, 146, 150];
        let text = str::from_utf8(&sparkle_heart)?;
        Ok(text.len() as i32)
    } else {
        Err(error::new_other_error("something else"))
    }
}