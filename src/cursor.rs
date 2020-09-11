use super::{page::*, row::*, table::*};

pub struct Cursor<'a> {
    table: &'a mut Table,
    row_num: usize,
    end_of_table: bool,
}

impl<'a> Cursor<'a> {
    pub fn new(table: &'a mut Table, row_num: usize, end_of_table: bool) -> Self {
        Self {
            table,
            row_num,
            end_of_table,
        }
    }

    /// returns Vec<u8> of row the cursor is pointing at
    pub fn get_value(&mut self) -> Option<Vec<u8>> {
        let row_num = self.row_num;
        let page_num = row_num / ROWS_PER_PAGE;
        self.table.pager.prepare_page(page_num);

        let page = self.table.pager.pages[page_num].as_ref().unwrap();
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        page.get_row(byte_offset)
    }

    /// insert given row into the position where the cursor is pointing at
    pub fn insert_value(&mut self, row: &Row) {
        let row_num = self.table.num_rows;
        let page_num = row_num / ROWS_PER_PAGE;
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        let _ = self.table.pager.insert_at(row, page_num, byte_offset);
    }

    /// advance cursor pointer by one
    pub fn advance_cursor(&mut self) {
        self.row_num += 1;
        if self.row_num >= self.table.num_rows {
            self.end_of_table = true;
        }
    }

    /// returns if the cursor is pointing at the end of the table
    pub fn is_end(&self) -> bool {
        self.end_of_table
    }
}
