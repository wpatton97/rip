#![allow(dead_code)]
// https://www.techiedelight.com/huffman-coding/
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse; // Used for min heap


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
        let mut min_heap:BinaryHeap<Reverse<HuffmanNode>> = BinaryHeap::new();

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
            min_heap.push(Reverse(v));
        }

        while let Some(node1) = min_heap.pop() {
            let tmp_node2 = min_heap.pop();

            if !tmp_node2.is_some(){
                return node1.0;
            }

            let node2 = tmp_node2.unwrap();

            let merged_node = HuffmanNode {
                freq_value: node1.0.freq_value + node2.0.freq_value,
                value: None,
                left: Some(Box::new(node1.0)),
                right: Some(Box::new(node2.0))
            };

            min_heap.push(Reverse(merged_node));
        }

        // should never get down here.
        return HuffmanNode {freq_value: 1, value: Some('d'), left: None, right: None}
    }
}

pub fn gen_codes(node: &HuffmanNode) -> HashMap<char, String> {
    let mut outMap: HashMap<char, String> = HashMap::new();

    recurse_codes(node, &mut outMap, "".to_string());

    return outMap;
}

fn recurse_codes(node: &HuffmanNode, map: &mut HashMap<char, String>, location: String){

    let loc_clone = location.to_owned();
    if node.value.is_some() {
        let char_val = node.value.unwrap();
        map.insert(char_val, loc_clone);
    }

    let left_code = format!("{}0", location).to_owned();
    let right_code = format!("{}1", location).to_owned();


    println!("curLocation: {}\t{:#?}", location, node.value);
    println!("left: {:?}", left_code);
    println!("right: {:?}", right_code);

    if node.left.is_some() {
        recurse_codes(&node.left.as_ref().unwrap(), map, left_code)
    }

    if node.right.is_some() {
        recurse_codes(&node.right.as_ref().unwrap(), map, right_code);
    }

}