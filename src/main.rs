mod ziparchive;
mod huffman;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
// Zip compression_method flags: https://users.cs.jmu.edu/buchhofp/forensics/formats/pkzip.html
// RFC for DEFLATE https://tools.ietf.org/html/rfc1951

fn main() {
    let y = ziparchive::ZipArchive::new("./resources/testarchive.zip");
    //y.print_all_data();
    readFile();

}


fn readFile(){
    let mut heap = BinaryHeap::new();

    let path = Path::new("./resources/red.txt");

    let mut file = File::open(path).expect("Failed to open file");

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Couldn't read file");

    let mut occuranses = HashMap::new();


    for c in s.chars(){
        if !occuranses.contains_key(&c){
            occuranses.insert(c, huffman::HuffmanNode {freq_value: 1, value: Some(c), left: None, right: None});
        }
        else{
            let mut item = occuranses.get_mut(&c).unwrap();

            item.freq_value = item.freq_value + 1;
        }
    }

    for (_, v) in occuranses.into_iter() {
        heap.push(v);
    }

    while let Some(node) = heap.pop() {
        let node2 = heap.pop();
        
        if !&node2.is_some(){
            println!("Last node!");
            println!("{:#?}", node);
            continue;
        }

        let node2_unwrapped = node2.unwrap();

        let mut newNode = huffman::HuffmanNode {
            freq_value: node.freq_value +&node2_unwrapped.freq_value,
            value: None,
            left: Some(Box::new(node)),
            right: Some(Box::new(node2_unwrapped))
        };

        heap.push(newNode);



        //println!("{:#?} {:#?}", node.freq_value, node.value.unwrap())
    }
}