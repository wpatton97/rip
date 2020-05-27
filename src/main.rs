mod ziparchive;

fn main() {
    let y = ziparchive::ZipArchive::new("./resources/testarchive.zip");
    y.print_all_data();
}
