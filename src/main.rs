mod ziparchive;
mod huffman;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
// Zip compression_method flags: https://users.cs.jmu.edu/buchhofp/forensics/formats/pkzip.html
// RFC for DEFLATE https://tools.ietf.org/html/rfc1951
// https://www2.cs.duke.edu/csed/poop/huff/info/

fn main() {
    let _ = ziparchive::ZipArchive::new("./resources/testarchive.zip");
    //y.print_all_data();
    test_huffman("english3.txt");

}


fn test_huffman(resource_file: &str){

    let path_string = format!("./resources/{}", resource_file).clone();
    let path = Path::new(&path_string);

    let mut file = File::open(path).expect("Failed to open file");

    let mut file_data = String::new();
    file.read_to_string(&mut file_data).expect("Couldn't read file");

    let n = huffman::HuffmanNode::new(&file_data);
    let codes = huffman::gen_codes(&n);

    for code in codes {
        println!("\"{}\": {:3}\t{}:{}", code.val, code.code, code.code_str, code.bitlength);
    }
}