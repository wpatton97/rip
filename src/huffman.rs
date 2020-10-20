#![allow(dead_code)]
// https://www.techiedelight.com/huffman-coding/
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

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

impl HuffmanNode {
    pub fn new(data: &str) -> HuffmanNode {

        let mut freq_map = HashMap::new();
        let mut min_heap:BinaryHeap<HuffmanNode> = BinaryHeap::new();

        // create a frequency map, and build each huffman node
        for c in data.chars() {
            if !freq_map.contains_key(&c){
                freq_map.insert(c, HuffmanNode {freq_value: 1, value: Some(c), left: None, right: None});
            }
            else{
                let mut item = freq_map.get_mut(&c).unwrap();

                item.freq_value = item.freq_value + 1;
            }
        }

        for (_, v) in freq_map.into_iter(){
            min_heap.push(v);
        }

        while let Some(node1) = min_heap.pop() {
            let tmp_node2 = min_heap.pop();

            if !tmp_node2.is_some(){
                return node1;
            }

            let node2 = tmp_node2.unwrap();

            let merged_node = HuffmanNode {
                freq_value: node1.freq_value + node2.freq_value,
                value: None,
                left: Some(Box::new(node1)),
                right: Some(Box::new(node2))
            };

            min_heap.push(merged_node);
        }

        // should never get down here.
        return HuffmanNode {freq_value: 1, value: Some('d'), left: None, right: None}
    }
}