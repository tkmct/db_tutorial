use super::row::ROW_SIZE;

pub const PAGE_SIZE: usize = 4096;
pub const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;

pub struct Page {
    pub buffer: Vec<u8>,
}

impl Page {
    pub fn new() -> Self {
        let buffer = vec![0; PAGE_SIZE];

        Page { buffer }
    }

    pub fn insert_row(&mut self, offset: usize, row: Vec<u8>) -> Result<(), String> {
        if offset + ROW_SIZE >= PAGE_SIZE {
            return Err(String::from("row_num exceed capacity"));
        }

        self.buffer.splice(offset..offset + ROW_SIZE, row);
        Ok(())
    }

    pub fn get_row(&self, offset: usize) -> Option<Vec<u8>> {
        if offset + ROW_SIZE >= PAGE_SIZE {
            return None;
        }

        Some(Vec::from(&self.buffer[offset..offset + ROW_SIZE]))
    }

    pub fn load_content(&mut self, content: Vec<u8>) {
        self.buffer = content;
    }
}
