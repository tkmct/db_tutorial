use super::{
    page::*,
    pager::Pager,
    row::{Row, ROW_SIZE},
};

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

    /// returns slice where to read/write in memory for a particular row
    pub fn row_slots(&mut self, row_num: usize) -> (&Page, usize) {
        let page_num = row_num / ROWS_PER_PAGE;
        self.pager.prepare_page(page_num);

        let page = self.pager.pages[page_num].as_ref().unwrap();
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        (page, byte_offset)
    }

    pub fn insert_row(&mut self, row: &Row) -> Result<(), Box<dyn Error>> {
        let row_num = self.num_rows;
        let page_num = row_num / ROWS_PER_PAGE;
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        self.pager.insert_at(row, page_num, byte_offset)
    }
}
