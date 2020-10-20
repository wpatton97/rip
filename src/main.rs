mod ziparchive;
// Zip compression_method flags: https://users.cs.jmu.edu/buchhofp/forensics/formats/pkzip.html

fn main() {
    let y = ziparchive::ZipArchive::new("./resources/testarchive.zip");
    y.print_all_data();
}
