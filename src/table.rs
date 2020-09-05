use super::row::ROW_SIZE;

pub const PAGE_SIZE: usize = 4096;
pub const TABLE_MAX_PAGES: usize = 100;
pub const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Table {
    pub num_rows: usize,
    pub pages: Vec<Page>,
}

impl Table {
    pub fn new() -> Self {
        let pages = Vec::with_capacity(TABLE_MAX_PAGES);

        Table { num_rows: 0, pages }
    }

    /// returns slice where to read/write in memory for a particular row
    pub fn row_slots(&mut self, row_num: usize) -> (&mut Page, usize) {
        let page_num = row_num / ROWS_PER_PAGE;

        if self.pages.len() <= page_num {
            self.allocate_page(page_num);
        }
        let page = &mut self.pages[page_num];
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        (page, byte_offset)
    }

    /// allocate page up to the given page_num.
    fn allocate_page(&mut self, page_num: usize) {
        let mut i = self.pages.len();
        while i <= page_num {
            self.pages.push(Page::new());
            i += 1;
        }
    }
}

pub struct Page {
    buffer: Vec<u8>,
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
}
