mod ziparchive;

fn main() {
    ziparchive::ZipArchive::new("./resources/testarchive.zip");
    println!("Hello, world!");
}
