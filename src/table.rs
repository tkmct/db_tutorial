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

    /// returns cursor pointing to the given key
    pub fn table_find(&mut self, key: u32) -> Cursor {
        let root_page_num = self.root_page_num;
        if let Some(root_node) = self.get_node(root_page_num) {
            if root_node.is_root() {
                return self.leaf_node_find(root_page_num, key);
            }
        }

        panic!("root node does not exist")
    }

    pub fn leaf_node_find(&mut self, page_num: usize, key: u32) -> Cursor {
        let root_node = self.get_node(page_num).unwrap();
        let num_cells = root_node.num_cells();

        // binary search
        let mut min_index = 0;
        let mut one_past_max_index = num_cells;
        while one_past_max_index != min_index {
            let index = (min_index + one_past_max_index) / 2;
            let key_at_index = root_node.get_key(index).unwrap();

            if key == key_at_index {
                return Cursor::new(self, page_num, index, false);
            }

            if key < key_at_index {
                one_past_max_index = index;
            } else {
                min_index = index + 1;
            }
        }

        return Cursor::new(self, page_num, min_index, false);
    }

    pub fn get_node(&self, page_num: usize) -> Option<&LeafNode> {
        if page_num < self.pager.pages.len() {
            return Some(&self.pager.pages[page_num]);
        }

        None
    }
}
