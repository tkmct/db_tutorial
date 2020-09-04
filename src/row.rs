use std::iter;

pub const ROW_LENGTH: usize = 291;

pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;

#[derive(Debug, PartialEq, Eq)]
pub struct Row<'a> {
    id: u32,
    username: &'a str,
    email: &'a str,
}

impl<'a> Row<'a> {
    pub fn new(id: u32, username: &'a str, email: &'a str) -> Self {
        Row {
            id,
            username,
            email,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::with_capacity(ROW_LENGTH);

        // id_bytes length is 4
        let id_bytes = self.id.to_le_bytes();
        for b in id_bytes.iter() {
            result.push(*b);
        }

        for b in self
            .username
            .chars()
            .chain(iter::repeat('\0'))
            .take(COLUMN_USERNAME_SIZE)
        {
            result.push(b as u8)
        }

        for b in self
            .email
            .chars()
            .chain(iter::repeat('\0'))
            .take(COLUMN_EMAIL_SIZE)
        {
            result.push(b as u8)
        }

        result
    }

    pub fn deserialize(input: Vec<u8>) -> Option<Self> {
        Some(Row {
            id: 0,
            username: "",
            email: "",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_length() {
        let row = Row {
            id: 12,
            username: "John Doe",
            email: "john@example.com",
        };

        let serialized = row.serialize();
        assert_eq!(serialized.len(), 291);

        let row = Row {
            id: 112,
            username: "Takamichi Tsutsumi",
            email: "tkmct@gmail.com",
        };

        let serialized = row.serialize();
        assert_eq!(serialized.len(), 291);
    }

    #[test]
    fn test_serialize_row() {
        let row = Row {
            id: 12,
            username: "John Doe",
            email: "john@example.com",
        };
        let serialized = row.serialize();
        let deserialized = Row::deserialize(serialized);

        assert_eq!(Some(row), deserialized);
    }
}
