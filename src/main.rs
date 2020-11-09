mod ziparchive;
mod huffman;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use crate::huffman::HuffmanNode;

// Zip compression_method flags: https://users.cs.jmu.edu/buchhofp/forensics/formats/pkzip.html
// RFC for DEFLATE https://tools.ietf.org/html/rfc1951
// https://www2.cs.duke.edu/csed/poop/huff/info/

fn main() {
    let _ = ziparchive::ZipArchive::new("./resources/testarchive.zip");
    //y.print_all_data();
    test_huffman("red.txt");
}


fn test_huffman(resource_file: &str){
    let path_string = format!("./resources/{}", resource_file).clone();
    let path = Path::new(&path_string);

    let mut file = File::open(path).expect("Failed to open file");

    let mut file_data = String::new();
    file.read_to_string(&mut file_data).expect("Couldn't read file");
    //file_data = "fuck this shit m8.".to_string();
    let huffman_tree_root = huffman::HuffmanNode::new(&file_data);
    let code_map = huffman::gen_code_map(&huffman_tree_root);
    let mut codes = huffman::gen_codes(&huffman_tree_root);

    let mut codes_to_write:Vec<huffman::HuffCode> = Vec::new();

    for code_key in file_data.chars() {
        codes_to_write.push(code_map[&code_key].clone())
    }

    // for code in &codes_to_write {
    //     new_size_bits += code.bitlength as u32;
    //     println!("{}", code);
    // }



    let compressed = huffman::codes_to_bin(&mut codes_to_write);

    println!("Compressed! Orig: {}bytes Compressed: {}bytes", file_data.len(), compressed.len())

}