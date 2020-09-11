use super::{cursor::Cursor, page::*, pager::Pager, row::ROW_SIZE};
use std::error::Error;

pub const TABLE_MAX_PAGES: usize = 100;
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Table {
    pub num_rows: usize,
    pub pager: Pager,
}

impl Table {
    pub fn open(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut pager = Pager::open(filename)?;
        let file_length = pager.get_file_length();
        let num_rows: usize = file_length as usize / ROW_SIZE;

        Ok(Table { num_rows, pager })
    }

    pub fn close(&mut self) {
        let pager = &mut self.pager;
        let num_full_pages = self.num_rows / ROWS_PER_PAGE;

        for i in 0..num_full_pages {
            if pager.pages[i].is_none() {
                continue;
            }
            pager.flush(i, PAGE_SIZE);
        }

        // There might be a partial page to write at the end of the file
        // remove this when we implement B-tree
        let num_additional_rows = self.num_rows % ROWS_PER_PAGE;
        if num_additional_rows > 0 {
            let page_num = num_full_pages;
            if !pager.pages[page_num].is_none() {
                pager.flush(page_num, num_additional_rows * ROW_SIZE);
            }
        }
    }

    pub fn table_start(&mut self) -> Cursor {
        Cursor::new(self, 0, self.num_rows == 0)
    }

    pub fn table_end(&mut self) -> Cursor {
        Cursor::new(self, self.num_rows, true)
    }
}
