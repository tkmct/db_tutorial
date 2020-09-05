use std::convert::TryInto;
use std::fmt;
use std::iter::{self, FromIterator};

pub const ROW_SIZE: usize = 291;

pub const COLUMN_ID_SIZE: usize = 4;
pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;
const NULL_CHAR: char = '\0';

#[derive(Debug, PartialEq, Eq)]
pub struct Row {
    id: u32,
    username: String,
    email: String,
}

impl Row {
    pub fn new(id: u32, username: String, email: String) -> Self {
        Row {
            id,
            username,
            email,
        }
    }

    /// serialize row for id, username, email
    /// each field has length of 4, 32, 255 bytes.
    /// length of serialized vector will be 291
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::with_capacity(ROW_SIZE);

        // id_bytes length is 4
        let id_bytes = self.id.to_le_bytes();
        for b in id_bytes.iter() {
            result.push(*b);
        }

        for b in self
            .username
            .chars()
            .chain(iter::repeat(NULL_CHAR))
            .take(COLUMN_USERNAME_SIZE)
        {
            result.push(b as u8)
        }

        for b in self
            .email
            .chars()
            .chain(iter::repeat(NULL_CHAR))
            .take(COLUMN_EMAIL_SIZE)
        {
            result.push(b as u8)
        }

        result
    }

    /// deserialize fixed length vector of u8
    /// input vector must be length of `ROW_SIZE`
    pub fn deserialize(input: Vec<u8>) -> Option<Self> {
        if input.len() != ROW_SIZE {
            return None;
        }

        let raw_id: &[u8; 4] = &input[..COLUMN_ID_SIZE]
            .try_into()
            .expect("slice with incorrect length");
        let id = u32::from_le_bytes(*raw_id);

        let raw_username = &input[COLUMN_ID_SIZE..COLUMN_USERNAME_SIZE];
        let username: String = String::from_iter(raw_username.iter().map(|c| *c as char))
            .trim_end_matches(NULL_CHAR)
            .into();

        let raw_email = &input[COLUMN_ID_SIZE + COLUMN_USERNAME_SIZE..];
        let email: String = String::from_iter(raw_email.iter().map(|c| *c as char))
            .trim_end_matches(NULL_CHAR)
            .into();

        Some(Row {
            id,
            username,
            email,
        })
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.id,
            self.username.trim_end_matches(NULL_CHAR),
            self.email.trim_end_matches(NULL_CHAR)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_length() {
        let row = Row {
            id: 12,
            username: String::from("John Doe"),
            email: String::from("john@example.com"),
        };

        let serialized = row.serialize();
        assert_eq!(serialized.len(), 291);

        let row = Row {
            id: 112,
            username: String::from("Takamichi Tsutsumi"),
            email: String::from("tkmct@gmail.com"),
        };

        let serialized = row.serialize();
        assert_eq!(serialized.len(), 291);
    }

    #[test]
    fn test_serialize_row() {
        let row = Row {
            id: 12,
            username: String::from("John Doe"),
            email: String::from("john@example.com"),
        };
        let serialized = row.serialize();
        let deserialized = Row::deserialize(serialized);

        assert_eq!(Some(row), deserialized);
    }
}
