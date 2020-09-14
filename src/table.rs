use super::{btree::LeafNode, cursor::Cursor, pager::Pager};
use std::error::Error;

pub struct Table {
    pub root_page_num: usize, // index of root node
    pub pager: Pager,
}

impl Table {
    pub fn open(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut pager = Pager::open(filename)?;
        if pager.pages.is_empty() {
            pager.pages.push(LeafNode::new(true, 0, 0, Vec::new()));
        }

        Ok(Table {
            root_page_num: 0,
            pager,
        })
    }

    pub fn close(&mut self) {
        let pager = &mut self.pager;
        let num_pages = pager.pages.len();

        for i in 0..num_pages {
            pager.flush(i);
        }
    }

    /// returns cursor pointing to the start of the table
    pub fn table_start(&mut self) -> Cursor {
        let num_cells = self.pager.pages[self.root_page_num].num_cells();

        Cursor::new(self, self.root_page_num, 0, num_cells == 0)
    }

    /// returns cursor pointing to the end of the table
    pub fn table_end(&mut self) -> Cursor {
        let num_cells = self.pager.pages[self.root_page_num].num_cells();

        Cursor::new(self, self.root_page_num, num_cells, true)
    }

    pub fn get_node(&self, page_num: usize) -> Option<&LeafNode> {
        if page_num < self.pager.pages.len() {
            return Some(&self.pager.pages[page_num]);
        }

        None
    }
}
