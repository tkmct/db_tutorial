use super::page::{Page, PAGE_SIZE};
use super::row::Row;
use super::table::TABLE_MAX_PAGES;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

pub struct Pager {
    pub file: std::fs::File,
    pub pages: Vec<Option<Page>>,
}

impl Pager {
    pub fn open(filename: &str) -> Result<Self, Box<dyn Error>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)?;

        let mut pages = Vec::new();
        for _ in 0..TABLE_MAX_PAGES {
            pages.push(None);
        }

        Ok(Pager { file, pages })
    }

    /// flush data in memory to disk
    pub fn flush(&mut self, page_num: usize, buff_size: usize) {
        if self.pages[page_num].is_none() {
            panic!("Tried to flush null page");
        }

        self.file
            .seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64))
            .unwrap();
        let content = &self.pages[page_num].as_ref().unwrap().buffer[..buff_size];
        let _ = self.file.write_all(content);
    }

    pub fn get_file_length(&mut self) -> u64 {
        let file_length = self.file.seek(SeekFrom::End(0)).unwrap();
        file_length
    }

    pub fn prepare_page(&mut self, page_num: usize) {
        if self.pages[page_num].is_some() {
            return ();
        }

        let mut page = Page::new();

        let file_length = self.get_file_length() as usize;
        let mut num_pages = file_length / PAGE_SIZE;
        if file_length % PAGE_SIZE != 0 {
            num_pages += 1;
        }

        if page_num <= num_pages {
            let offset = page_num * PAGE_SIZE;

            let mut buff = vec![0; PAGE_SIZE];
            self.file.seek(SeekFrom::Start(offset as u64)).unwrap();
            let _ = self.file.read(&mut buff);
            page.load_content(buff);
        }

        self.pages[page_num] = Some(page);
    }

    pub fn insert_at(
        &mut self,
        row: &Row,
        page_num: usize,
        byte_offset: usize,
    ) -> Result<(), Box<dyn Error>> {
        // load content to page if page is none
        self.prepare_page(page_num);

        if let Some(ref mut page) = self.pages[page_num] {
            page.insert_row(byte_offset, row.serialize())?;
        }

        Ok(())
    }
}
