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

// STUB!! Does not work yet
pub fn codes_to_bin(codes: &mut Vec<HuffCode>) -> Vec<u8> {

    let mut output:Vec<u8> = Vec::new();
    let mut sum_bits: u32 = 0;
    let mut num_bytes_needed:u32 = 0;

    for code in codes.clone() {
        sum_bits += code.bitlength as u32;
    }
    num_bytes_needed = (sum_bits / 8) + (if (sum_bits % 8) > 1 {1} else {0});

    let mut leftover = 0;
    let mut code_index: usize = 0;
    while output.len() < num_bytes_needed as usize {
        let mut cur_byte:u8 = 0;

        if leftover > 0 {  // handle overflow from prev iteration.
            println!("Left: {:02}", leftover);
            let code_that_went_over = &codes[code_index];
            cur_byte = 0 | (code_that_went_over.code as u8) << (8 - leftover);
            code_index += 1;
        }

        let code = &codes[code_index];




        println!("vec_size: {:02} leftover:{:02} codelen: {:02} sum: {:02}", output.len(), leftover, code.bitlength, leftover + code.bitlength);
        if code.bitlength + leftover > 8 { // if we cannot fit a whole code
            cur_byte = cur_byte | ((code.code as u8) >> 8 - leftover);
            leftover = code.bitlength + leftover - 8;
            output.push(cur_byte);
        }
        else if code.bitlength + leftover == 8 {   // if we can fit exactly one code
            cur_byte = 0 | code.code as u8;
            output.push(cur_byte);
            leftover = 0;
            code_index += 1;
        }
        else {  // if we can fit more than just the current code
            println!("3rd branch");
            cur_byte = cur_byte | ((code.code as u8) >> leftover);
            code_index += 1;
            let mut nxtcode = &codes[code_index];
            let space_left: i8 = 8 - code.bitlength as i8;
            if space_left <= nxtcode.bitlength as i8{   // if we can fit exactly one more code, or not a full extra code
                println!("3rd sub 1 - nxtcode_length: {}", nxtcode.bitlength);
                leftover = nxtcode.bitlength - space_left as u8;
                cur_byte = cur_byte << 8 - space_left;
                cur_byte = cur_byte | (nxtcode.code as u8 >> leftover);
                if leftover == 0 { // only if the space left was = nextcode.bitlength
                    code_index += 1;
                }
                output.push(cur_byte);
            }
            else{   // if we can fit a whole code and have space left for a whole extra code, and more
                cur_byte = 0 | code.code as u8;
                code_index += 1;
                let mut total_length = code.bitlength; // set the amount of space we have used
                let mut space_left: i8 = 8 - code.bitlength as i8;  // set the amount of space we have left
                while space_left >= 0 { // while we have space left
                    nxtcode = &codes[code_index];   // grab the current next code
                    total_length += nxtcode.bitlength;  // add the next code to the total length
                    space_left = 8 - total_length as i8;    // re-calculate our space left

                    if space_left >= 0 {    // if we have space left *still*
                        let shifted_code = nxtcode.code >> (total_length - nxtcode.bitlength);
                        cur_byte = cur_byte | shifted_code as u8;
                    }
                    else { // if space left is negative, and we will have leftover
                        let shifted_code = nxtcode.code >> (total_length - nxtcode.bitlength);
                        cur_byte = cur_byte | shifted_code as u8;
                        leftover = space_left.abs() as u8;
                        output.push(cur_byte);
                        // should break, but wont need too. if it gets down here, itll be end of loop
                    }
                }
            }
        }

    }

    return output;
}