#![allow(dead_code)]
// https://www.techiedelight.com/huffman-coding/
use std::cmp::Ordering;

#[derive(Debug, Eq, Clone)]
pub struct HuffmanNode {
    pub freq_value: i32,
    pub left: Option<Box<HuffmanNode>>,
    pub right: Option<Box<HuffmanNode>>,
    pub value: Option<char> // only populated if it is a leaf
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.freq_value.cmp(&other.freq_value);
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for HuffmanNode {
    fn eq (&self, other: &Self) -> bool {
        return self.freq_value == other.freq_value;
    }
}