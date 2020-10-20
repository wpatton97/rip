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

    let path = Path::new("./resources/red.txt");

    let mut file = File::open(path).expect("Failed to open file");

    let mut red = String::new();
    file.read_to_string(&mut red).expect("Couldn't read file");

    let n = huffman::HuffmanNode::new("this is a huffman test");
    let codes = huffman::gen_codes(&n);



    println!("{:#?}", codes)
}