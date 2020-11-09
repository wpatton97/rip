#![allow(dead_code)]
// https://www.techiedelight.com/huffman-coding/
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt;
use std::cmp::Reverse; // Used for min heap, this fixed all my problems with all nodes on the left lol

#[derive(Debug, Clone)]
pub struct HuffCode {
    pub val: char,
    pub bitlength: u8, // number of bits used
    pub code: u64,
    pub code_str: String
}

impl fmt::Display for HuffCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = self.val.to_string(); 
        let o = if self.val == '\n' {"\\n"} else {&d[..]};
        write!(f, "{:5} {:0width$b}:{}", o, self.code, self.bitlength, width=self.bitlength as usize)
    }
}

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

pub fn gen_codes(root_node: &HuffmanNode) -> Vec<HuffCode>{
    let mut out_codes: Vec<HuffCode> = Vec::new();

    recurse_codes(root_node, &mut out_codes, "".to_string(), 0, 0);
    return out_codes;
}

pub fn gen_code_map(root_node: &HuffmanNode) -> HashMap<char, HuffCode> {
    let codes = gen_codes(root_node);
    let mut out_map = HashMap::new();

    for code in codes {
        out_map.insert(code.val, code);
    }

    return out_map;

}

fn recurse_codes(node: &HuffmanNode, codes: &mut Vec<HuffCode>, location_str: String, location: u64, depth: u8){

    let loc_clone = location_str.to_owned();
    if node.value.is_some() {
        let char_val = node.value.unwrap();
        codes.push(HuffCode {val: char_val, bitlength: depth, code: location, code_str: loc_clone.clone()})
    }

    let left_code_str = format!("{}0", location_str).to_owned();
    let right_code_str = format!("{}1", location_str).to_owned();

    let left_code = location << 1;
    let right_code = (location << 1) | 1;

    if node.left.is_some() {
        recurse_codes(&node.left.as_ref().unwrap(), codes, left_code_str, left_code, depth + 1)
    }

    if node.right.is_some() {
        recurse_codes(&node.right.as_ref().unwrap(), codes, right_code_str, right_code, depth + 1);
    }

}

// We have a collection of HuffCodes.
// Each HuffCode has a u64 code (which stores the actual binary data)
// and u8 bitlength, which determines the length of the u64 code we are taking.
// We are trying to concatenate all of these into a single vector of u8s.
pub fn codes_to_bin(codes: &mut Vec<HuffCode>) -> Vec<u8> {
    let mut output:Vec<u8> = Vec::new();
    let most_significant = 0x8000000000000000 as u64;

    for huffCode in codes.into_iter() {
        let mut code = huffCode.code;
        let mut index = 0;

        code = code << (64 - huffCode.bitlength);

        while index < huffCode.bitlength {
            if code & most_significant == most_significant {
                output.push(1);
            }
            else {
                output.push(0);
            }
            index += 1;
            code = code << 1;
        }
    }

    println!("{:?}", output);

    return output;
}