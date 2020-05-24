use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Cursor;
use std::io::Seek;
use std::slice;
use std::mem;
use std::io::SeekFrom;

/// Marks the start of a file, and provides the uncompressed data
struct LocalFileHeader {            
    
                                    // OFFSETS:
    magic_number: u32,              // 0            0x04034b50 (read as a little-endian number)
    version_needed: u16,            // 4
    spacer_unused: u16,             // 6
    compression_method: u16,        // 8
    last_modify_time: u16,          // 10
    last_modify_date: u16,          // 12
    crc32_uncompressed: u32,        // 14
    compressed_size: u32,           // 18
    uncompressed_size: u32,         // 22
    file_name_length: u16,          // 26 (n)
    extra_field_length: u16,        // 28 (m)
    file_name: Vec<u8>,             // 30
    extra_field: Vec<u8>,           // 30 + n
    compressed_data: Vec<u8>
    // https://en.wikipedia.org/wiki/Zip_(file_format)
}

/// The central directory record (CDR) is an expanded form of the local header
struct CentralDirectoryFileHeader {
    /// The Central Directory Contains multiple CDRs     
                                        // OFFSETS
    magic_number: u32,                  // 0        0x02014b50 (Central directory file header signature)
    version_made_by: u16,               // 4
    version_needed: u16,                // 6
    spacer_unused: u16,                 // 8
    compression_method: u16,            // 10
    last_modify_time: u16,              // 12
    last_modify_date: u16,              // 14
    crc32_uncompressed: u32,            // 16
    compressed_size: u32,               // 20
    uncompressed_size: u32,             // 24
    file_name_length: u16,              // 28       (n)
    extra_field_length: u16,            // 30       (m)
    file_comment_length: u16,           // 32       (k)
    disk_number_source: u16,            // 34
    internal_file_attributes: u16,      // 36
    external_file_attributes: u32,      // 38
    relative_offset_localheader: u32,   // 42       Relative offset of local file header. This is the number of bytes between the start of the first disk on which the file occurs, and the start of the local file header.
    filename: Vec<u8>,                  // 46
    extra_field: Vec<u8>,               // 46 + n
    file_comment: Vec<u8>               // 46 + n + m
}

/// After all the central directory entries comes the end of central directory (EOCD) record, which marks the end of the ZIP file
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct EndOfCentralDirectoryRecord {

                                        // OFFSETS
    magic_number: u32,                  // 0        0x06054b50
    number_of_current_disk: u16,        // 4
    disk_where_cdr_starts: u16,         // 6
    num_cdr_on_disk: u16,               // 8
    total_cdr: u16,                     // 10
    size_of_cdr: u32,                   // 12       Size of the Central Directory in Bytes
    offset_cdr_start: u32,              // 16       Offset from the start of the archive where the CentralDirectory starts (in bytes, obvi)
    comment_length: u16,                 // 20       (n)
    // comment: Vec<u8>
}

impl EndOfCentralDirectoryRecord {
    /// Reads a binary array into a struct, using the C representaion
    /// https://stackoverflow.com/questions/25410028/how-to-read-a-struct-from-a-file-in-rust
    pub fn load_data(&mut self, mut file: std::fs::File, offset_starting: u64, size: u64){
        println!("Loading from offset: {}", offset_starting);
        let mut struct_data = vec![0u8; size as usize];

        file.seek(SeekFrom::Start(offset_starting)).unwrap();
        file.read(&mut struct_data).unwrap();

        let mut data: EndOfCentralDirectoryRecord = unsafe {mem::zeroed()};
        let data_size = mem::size_of::<EndOfCentralDirectoryRecord>();

        let mut c = Cursor::new(struct_data);

        unsafe {
            let data_slice = slice::from_raw_parts_mut(&mut data as *mut _ as *mut u8, data_size);
            c.read_exact(data_slice).unwrap();
        }

        println!("Struct: {:#?}", data);


        // let mut magic_buf: [u8; 4] = [0x0; 4];
        // file.seek(SeekFrom::Start(offset_starting)).unwrap();
        // file.read(&mut magic_buf).unwrap();
        // println!("{:?}", magic_buf);
        // unsafe {
        //     println!("{}", mem::transmute::<[u8; 4], u32>(magic_buf));
        // }
        
        // println!("Got magic number: {}", self.magic_number);

    }

    pub fn new() -> EndOfCentralDirectoryRecord{
        EndOfCentralDirectoryRecord{
            magic_number: 0x06054b50,
            number_of_current_disk: 0,
            disk_where_cdr_starts: 0,
            num_cdr_on_disk: 0,
            total_cdr: 0,
            size_of_cdr: 0,
            offset_cdr_start: 0,
            comment_length: 0
        }
    }
}

pub struct ZipArchive {
    local_file_data: Vec<LocalFileHeader>,
    central_records: Vec<CentralDirectoryFileHeader>,
    eof_record: EndOfCentralDirectoryRecord
}


impl ZipArchive {
    pub fn new(filename: &str) -> ZipArchive{
        println!("New ZipArchive! {}", filename);
        let path = Path::new(filename);
        let mut file = match File::open(path) {
            Err(why) => panic!("Couldn't open {}: {}", path.display(), why.description()),
            Ok(file) => file
        };

        let last_pos = match file.seek(SeekFrom::End(0)) {
            Err(why) => panic!("Couldn't seek! {}", why.description()),
            Ok(pos) => pos
        };

        let eof_record_num:[u8; 4] = [0x50, 0x4b, 0x05, 0x06]; // 0x06054b50 Reversed for lil-endian

        println!("Seek'd to the last position which is: {}", last_pos);

        let mut current_index: i64 = 1;
        let mut eofdirectory_offset: u64 = 0;
        while current_index < last_pos as i64 {
            let mut buffer: [u8; 4] = [0x0; 4];
            file.seek(SeekFrom::End(-current_index)).unwrap();
            file.read(&mut buffer[..]).unwrap();
            if &eof_record_num[..] == &buffer[..] {
                println!("WE FOUND IT! {:?}", buffer);
                break;
            }
            current_index = current_index + 1;
        }

        eofdirectory_offset = last_pos - current_index as u64;
        let mut eof_data = EndOfCentralDirectoryRecord::new();
        let size = last_pos - eofdirectory_offset;
        eof_data.load_data(file, eofdirectory_offset, size);


        return ZipArchive{
            local_file_data: Vec::new(),
            central_records: Vec::new(),
            eof_record: EndOfCentralDirectoryRecord{
                magic_number: 0x06054b50,
                number_of_current_disk: 0,
                disk_where_cdr_starts: 0,
                num_cdr_on_disk: 0,
                total_cdr: 0,
                size_of_cdr: 0,
                offset_cdr_start: 0,
                comment_length: 0
            }
        };
    }
}