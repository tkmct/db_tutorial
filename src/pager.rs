use super::btree::{LeafNode, PAGE_SIZE};
use super::row::Row;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

pub struct Pager {
    pub file: std::fs::File,
    pub pages: Vec<LeafNode>,
}

impl Pager {
    pub fn open(filename: &str) -> Result<Self, Box<dyn Error>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)?;

        let pages = Vec::new();
        let mut pager = Pager { file, pages };

        let file_length = pager.get_file_length();
        let num_pages = file_length as usize / PAGE_SIZE;

        if file_length == 0 {
            return Ok(pager);
        }

        for i in 0..num_pages {
            pager.prepare_page(i);
        }

        Ok(pager)
    }

    /// flush data in memory to disk
    pub fn flush(&mut self, page_num: usize) {
        if self.pages.get(page_num).is_none() {
            panic!("Tried to flush null page");
        }

        self.file
            .seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64))
            .unwrap();
        let content = &self.pages[page_num].serialize();
        let _ = self.file.write_all(content);
    }

    pub fn get_file_length(&mut self) -> u64 {
        let file_length = self.file.seek(SeekFrom::End(0)).unwrap();
        file_length
    }

    pub fn prepare_page(&mut self, page_num: usize) {
        if self.pages.get(page_num).is_some() {
            return ();
        }

        let file_length = self.get_file_length() as usize;
        let num_pages_on_file = file_length / PAGE_SIZE;
        if file_length % PAGE_SIZE != 0 {
            // this should not happen
            panic!("broken file");
        }

        if num_pages_on_file <= page_num {
            panic!("page_num out of index");
        }

        let file_offset = (page_num * PAGE_SIZE) as u64;
        let mut buff = vec![0; PAGE_SIZE];

        let _ = self.file.seek(SeekFrom::Start(file_offset));
        let _ = self.file.read(&mut buff);

        if let Some(node) = LeafNode::deserialize(buff) {
            self.pages.push(node);
        } else {
            panic!("broken file")
        }
    }

    pub fn insert_at(
        &mut self,
        row: &Row,
        page_num: usize,
        pos: usize,
    ) -> Result<(), Box<dyn Error>> {
        if self.pages.len() <= page_num {
            panic!("index out of bounds");
        }
        let node = &mut self.pages[page_num];
        let _ = node.insert_at(pos, row.id, row.clone());
        Ok(())
    }
}
