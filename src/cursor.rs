use super::{row::*, table::*};

pub struct Cursor<'a> {
    table: &'a mut Table,
    page_num: usize,
    cell_num: usize,
    end_of_table: bool,
}

impl<'a> Cursor<'a> {
    pub fn new(table: &'a mut Table, page_num: usize, cell_num: usize, end_of_table: bool) -> Self {
        Self {
            table,
            page_num,
            cell_num,
            end_of_table,
        }
    }

    /// returns Vec<u8> of row the cursor is pointing at
    pub fn get_value(&mut self) -> Option<Row> {
        let page_num = self.page_num;

        // TODO: prepare
        // self.table.pager.prepare_page(page_num);

        let node = self.table.get_node(page_num).unwrap();
        node.get_value(self.cell_num)
    }

    /// insert given row into the position where the cursor is pointing at
    pub fn insert_value(&mut self, row: &Row) {
        let _ = self
            .table
            .pager
            .insert_at(row, self.page_num, self.cell_num);
    }

    /// advance cursor pointer by one
    pub fn advance(&mut self) {
        let page_num = self.page_num;
        let node = self.table.get_node(page_num).unwrap();

        self.cell_num += 1;
        if self.cell_num >= node.num_cells() {
            self.end_of_table = true;
        }
    }

    /// returns if the cursor is pointing at the end of the table
    pub fn is_end(&self) -> bool {
        self.end_of_table
    }
}
