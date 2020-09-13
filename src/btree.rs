use super::row::Row;
use std::convert::TryInto;
use std::mem::size_of;
use std::ops::Range;

// CONSTANTS
const PAGE_SIZE: usize = 4096;
const ROW_SIZE: usize = 291;

// NODE HEADER CONSTANTS
const HEADER_NODE_TYPE_SIZE: usize = size_of::<u8>();
const HEADER_NODE_TYPE_OFFSET: usize = 0;

const HEADER_IS_ROOT_SIZE: usize = size_of::<u8>();
const HEADER_IS_ROOT_OFFSET: usize = HEADER_NODE_TYPE_OFFSET + HEADER_NODE_TYPE_SIZE;

const HEADER_PARENT_POINTER_SIZE: usize = size_of::<u32>();
const HEADER_PARENT_POINTER_OFFSET: usize = HEADER_IS_ROOT_OFFSET + HEADER_IS_ROOT_SIZE;

const HEADER_NUM_CELLS_SIZE: usize = size_of::<u32>();
const HEADER_NUM_CELLS_OFFSET: usize = HEADER_PARENT_POINTER_OFFSET + HEADER_PARENT_POINTER_SIZE;

const HEADER_SIZE: usize = HEADER_NODE_TYPE_SIZE
    + HEADER_IS_ROOT_SIZE
    + HEADER_PARENT_POINTER_SIZE
    + HEADER_NUM_CELLS_SIZE;

// NODE BODY CONSTANTS
const LEAF_NODE_BODY_OFFSET: usize = HEADER_SIZE;
const LEAF_NODE_KEY_SIZE: usize = size_of::<u32>();
const LEAF_NODE_VALUE_SIZE: usize = ROW_SIZE;
const CELL_SIZE: usize = LEAF_NODE_KEY_SIZE + LEAF_NODE_VALUE_SIZE;
const CELLS_SPACE: usize = PAGE_SIZE - HEADER_SIZE;
const MAX_NUM_CELLS: usize = CELLS_SPACE / CELL_SIZE;

type Key = u32;
type Value = Row;

#[derive(Eq, PartialEq, Debug)]
pub struct LeafNodeHeader {
    is_root: bool,
    parent: u32,
    num_cells: u32,
}

#[derive(Eq, PartialEq, Debug)]
pub struct LeafNode {
    header: LeafNodeHeader,
    body: Vec<(Key, Value)>,
}

fn bool_to_bytes(b: bool) -> Vec<u8> {
    vec![b as u8]
}

fn bytes_to_bool(v: &[u8]) -> bool {
    if v.len() != 1 {
        panic!("input length must be 1");
    }

    v[0] != 0
}

fn u32_to_bytes(num: u32) -> Vec<u8> {
    let mut v = Vec::<u8>::with_capacity(4);
    for p in num.to_le_bytes().iter() {
        v.push(*p);
    }
    v
}

fn bytes_to_u32(input: &[u8]) -> u32 {
    let raw: [u8; 4] = input.try_into().expect("slice with incorrect length");
    u32::from_le_bytes(raw)
}

impl LeafNode {
    const NODE_TYPE_RANGE: Range<usize> = HEADER_NODE_TYPE_OFFSET..HEADER_IS_ROOT_OFFSET;
    const IS_ROOT_RANGE: Range<usize> = HEADER_IS_ROOT_OFFSET..HEADER_PARENT_POINTER_OFFSET;
    const PARENT_POINTER_RANGE: Range<usize> =
        HEADER_PARENT_POINTER_OFFSET..HEADER_NUM_CELLS_OFFSET;
    const NUM_CELLS_RANGE: Range<usize> = HEADER_NUM_CELLS_OFFSET..HEADER_SIZE;

    fn node_type_bytes() -> Vec<u8> {
        vec![1]
    }

    pub fn new(is_root: bool, parent: u32, num_cells: u32, body: Vec<(Key, Value)>) -> Self {
        Self {
            header: LeafNodeHeader {
                is_root,
                parent,
                num_cells,
            },
            body,
        }
    }

    pub fn num_cells(&self) -> usize {
        self.header.num_cells as usize
    }

    pub fn get_value(&self, num: usize) -> Option<Value> {
        if num >= self.num_cells() {
            return None;
        }

        Some(self.body[num].1.clone())
    }

    pub fn get_key(&self, num: usize) -> Option<Key> {
        if num >= self.num_cells() {
            return None;
        }

        Some(self.body[num].0)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buff = vec![0; PAGE_SIZE];

        // Serialize header
        buff.splice(LeafNode::NODE_TYPE_RANGE, LeafNode::node_type_bytes());
        buff.splice(LeafNode::IS_ROOT_RANGE, bool_to_bytes(self.header.is_root));
        buff.splice(
            LeafNode::PARENT_POINTER_RANGE,
            u32_to_bytes(self.header.parent),
        );
        buff.splice(
            LeafNode::NUM_CELLS_RANGE,
            u32_to_bytes(self.header.num_cells),
        );

        // Serialize values
        for (i, (k, v)) in self.body.iter().enumerate() {
            let key_start = LEAF_NODE_BODY_OFFSET + i * CELL_SIZE;
            let key_end = key_start + LEAF_NODE_KEY_SIZE;
            let key_range = key_start..key_end;

            let value_start = key_end;
            let value_end = value_start + LEAF_NODE_VALUE_SIZE;
            let value_range = value_start..value_end;

            buff.splice(key_range, u32_to_bytes(*k));
            buff.splice(value_range, v.serialize());
        }

        buff
    }

    pub fn deserialize(raw: Vec<u8>) -> Option<Self> {
        if raw.len() != PAGE_SIZE {
            panic!(
                "input length does not match. given {} expected {}",
                raw.len(),
                PAGE_SIZE
            );
        }

        let node_type = &raw[0];
        if *node_type != 1 {
            return None;
        }

        let is_root = bytes_to_bool(&raw[LeafNode::IS_ROOT_RANGE]);
        let parent = bytes_to_u32(&raw[LeafNode::PARENT_POINTER_RANGE]);
        let num_cells = bytes_to_u32(&raw[LeafNode::NUM_CELLS_RANGE]);

        let mut body = Vec::with_capacity(MAX_NUM_CELLS);

        for i in 0..num_cells as usize {
            let key_start = LEAF_NODE_BODY_OFFSET + i * CELL_SIZE;
            let key_end = key_start + LEAF_NODE_KEY_SIZE;
            let key_range = key_start..key_end;

            let value_start = key_end;
            let value_end = value_start + LEAF_NODE_VALUE_SIZE;
            let value_range = value_start..value_end;

            let key = bytes_to_u32(&raw[key_range]);
            let value = Row::deserialize(raw[value_range].into()).unwrap();

            body.push((key, value));
        }

        Some(LeafNode::new(is_root, parent, num_cells, body))
    }
}

impl Default for LeafNode {
    fn default() -> Self {
        Self {
            header: LeafNodeHeader {
                is_root: false,
                parent: 0,
                num_cells: 0,
            },
            body: Vec::with_capacity(MAX_NUM_CELLS),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_to_page() {
        let body = vec![
            (1, Row::new(1, "user1".into(), "user1@example.com".into())),
            (2, Row::new(2, "user2".into(), "user2@example.com".into())),
            (3, Row::new(3, "user3".into(), "user3@example.com".into())),
        ];
        let node = LeafNode::new(true, 10, 3, body);
        let serialized = node.serialize();
        assert_eq!(serialized.len(), PAGE_SIZE);
    }

    #[test]
    fn test_serialize_and_deserialize() {
        let body = vec![
            (1, Row::new(1, "user1".into(), "user1@example.com".into())),
            (2, Row::new(2, "user2".into(), "user2@example.com".into())),
            (3, Row::new(3, "user3".into(), "user3@example.com".into())),
        ];
        let node = LeafNode::new(true, 10, 3, body);
        let serialized = node.serialize();
        let deserialized = LeafNode::deserialize(serialized);

        assert_eq!(Some(node), deserialized);
    }
}
