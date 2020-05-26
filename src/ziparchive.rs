#![allow(dead_code)]
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Cursor;
use std::io::Seek;
use std::slice;
use std::mem;
use std::io::SeekFrom;

/// Marks the start of a file, and provides the uncompressed data
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
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
    // file_name: Vec<u8>,             // 30
    // extra_field: Vec<u8>,           // 30 + n
    // compressed_data: Vec<u8>
    // https://en.wikipedia.org/wiki/Zip_(file_format)
}


impl LocalFileHeader{
    pub fn new() -> LocalFileHeader {
        LocalFileHeader {
            magic_number: 0,
            version_needed: 0,
            spacer_unused: 0,
            compression_method: 0,
            last_modify_time: 0,
            last_modify_date: 0,
            crc32_uncompressed: 0,
            compressed_size: 0,
            uncompressed_size: 0,
            file_name_length: 0,
            extra_field_length: 0
        }
    }

    pub fn load_data(&mut self, mut file: &std::fs::File, start_offset: u64) -> u64 {
        println!("Loading LocalFileHeader from offset: {:#X}", start_offset);
        let data_size = mem::size_of::<LocalFileHeader>();
        let mut struct_data = vec![0u8; data_size];

        file.seek(SeekFrom::Start(start_offset)).expect("Could not seek to location.");
        file.read(&mut struct_data).expect("Couldn't read.");

        let mut data: LocalFileHeader = LocalFileHeader::new();
        let mut c = Cursor::new(struct_data);

        unsafe {
            let data_slice = slice::from_raw_parts_mut(&mut data as *mut _ as *mut u8, data_size);
            c.read_exact(data_slice).expect("Couldn't read from struct data");
        }

        // TODO: Add check for correct magic num/sig here

        self.magic_number = data.magic_number;
        self.version_needed = data.version_needed;
        self.spacer_unused = data.spacer_unused;
        self.compression_method = data.compression_method;
        self.last_modify_time = data.last_modify_time;
        self.last_modify_date = data.last_modify_date;
        self.crc32_uncompressed = data.crc32_uncompressed;
        self.compressed_size = data.compressed_size;
        self.uncompressed_size = data.uncompressed_size;
        self.file_name_length = data.file_name_length;
        self.extra_field_length = data.extra_field_length;


        return start_offset + data_size as u64;
    }
}


struct LocalFile {
    static_data: LocalFileHeader,
    data_start_offset: u64,
    file_name_data: Vec<u8>,
    extra_field: Vec<u8>,
    compressed_data: Vec<u8>
}

impl LocalFile {
    pub fn new() -> LocalFile {
        LocalFile {
            static_data: LocalFileHeader::new(),
            data_start_offset: 0,
            file_name_data: Vec::new(),
            extra_field: Vec::new(),
            compressed_data: Vec::new()
        }
    }

    /// Load metadata
    /// Returns the offset of the end (start_offset + static_data size + compressed_data_size)
    pub fn load_metadata(&mut self, mut file: &std::fs::File, start_offset: u64) -> u64 {
        let mut static_data = LocalFileHeader::new();
        let end_o_static_data = static_data.load_data(&mut file, start_offset);

        let mut file_name = vec![0; static_data.file_name_length as usize];
        file.seek(SeekFrom::Start(end_o_static_data)).expect("Couldn't seek!");
        file.read(&mut file_name).expect("Couldn't read");

        let mut extra_field = vec![0; static_data.extra_field_length as usize];
        file.read(&mut extra_field).expect("Couldn't read");

        self.static_data = static_data;
        self.data_start_offset = static_data.file_name_length as u64 + static_data.extra_field_length as u64 + end_o_static_data as u64;
        self.file_name_data = file_name;
        self.extra_field = extra_field;

        return self.data_start_offset + self.static_data.compressed_size as u64;
    }

    /// Loads the compressed data for the current LocalFileHeader into memory
    pub fn load_compressed_data(&mut self, mut file: &std::fs::File){
        file.seek(SeekFrom::Start(self.data_start_offset)).expect("Couldn't seek");
        let mut data = vec![0; self.static_data.compressed_size as usize];
        file.read(&mut data).expect("Couldn't read");
        self.compressed_data = data;

    }
}

/// The central directory record (CDR) is an expanded form of the local header
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
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
    // filename: Vec<u8>,                  // 46
    // extra_field: Vec<u8>,               // 46 + n
    // file_comment: Vec<u8>               // 46 + n + m
}

impl CentralDirectoryFileHeader{
    pub fn new() -> CentralDirectoryFileHeader {
        CentralDirectoryFileHeader {
            magic_number: 0,
            version_made_by: 0,
            version_needed: 0,
            spacer_unused: 0,
            compression_method: 0,
            last_modify_time: 0,
            last_modify_date: 0,
            crc32_uncompressed: 0,
            compressed_size: 0,
            uncompressed_size: 0,
            file_name_length: 0,
            extra_field_length: 0,
            file_comment_length: 0,
            disk_number_source: 0,
            internal_file_attributes: 0,
            external_file_attributes: 0,
            relative_offset_localheader: 0
        }
    }

    /// Loads data into a CentralDirecotyFileHeader
    /// Returns where reading stopped. (offset + size of struct)
    pub fn load_data(&mut self, mut file: &std::fs::File, start_offset: u64) -> u64{
        println!("Loading CDFR from offset: {:#X}", start_offset);
        let data_size = mem::size_of::<CentralDirectoryFileHeader>();
        let mut struct_data = vec![0u8; data_size];

        file.seek(SeekFrom::Start(start_offset)).expect("Couldn't seek to start of CDFR");
        file.read(&mut struct_data).expect("Couldn't read from file.");

        let mut data: CentralDirectoryFileHeader = unsafe { mem::zeroed() };

        let mut c = Cursor::new(struct_data);

        unsafe {
            let data_slice = slice::from_raw_parts_mut(&mut data as *mut _ as *mut u8, data_size);
            c.read_exact(data_slice).expect("Couldn't read slice data into struct.");
        }

        if data.magic_number != 33639248{
            println!("\tError!! Magic number incorrect!");
        }

        // println!("Got magic number: {:#X}", data.magic_number);

        self.magic_number = data.magic_number;
        self.version_made_by = data.version_made_by;
        self.version_needed = data.version_needed;
        self.spacer_unused = data.spacer_unused;
        self.compression_method = data.compression_method;
        self.last_modify_time = data.last_modify_time;
        self.last_modify_date = data.last_modify_date;
        self.crc32_uncompressed = data.crc32_uncompressed;
        self.compressed_size = data.compressed_size;
        self.uncompressed_size = data.uncompressed_size;
        self.file_name_length = data.file_name_length;
        self.extra_field_length = data.extra_field_length;
        self.file_comment_length = data.file_comment_length;
        self.disk_number_source = data.disk_number_source;
        self.internal_file_attributes = data.internal_file_attributes;
        self.external_file_attributes = data.external_file_attributes;
        self.relative_offset_localheader = data.relative_offset_localheader;

        return start_offset + data_size as u64;
    }
}

/// A wrapper around CentralDirectoryFileHeader so that we can pac the static stuff, and then manually fill the rest.
/// Central Directory File Header Record (CDFHR)
#[derive(Debug, Clone)]
struct CDFHR {
    static_data: CentralDirectoryFileHeader,
    start_offset: u64,
    end_offset: u64,
    file_name_data: Vec<u8>,
    extra_field_data: Vec<u8>,
    file_comment_data: Vec<u8>
}

impl CDFHR {
    pub fn new() -> CDFHR {
        CDFHR {
            static_data: CentralDirectoryFileHeader::new(),
            start_offset: 0,
            end_offset: 0,
            file_name_data: Vec::new(),
            extra_field_data: Vec::new(),
            file_comment_data: Vec::new()
        }
    }

    /// Loads the object calling it.
    /// Returns a u64 containg the end position after reading.
    pub fn load_data(&mut self, mut file: &std::fs::File, start_offset: u64) -> u64{
        let mut static_data = CentralDirectoryFileHeader::new();
        let end_static_offset = static_data.load_data(&mut file, start_offset);

        

        let mut file_name_buf = vec![0; static_data.file_name_length as usize];
        let mut extra_field_buf = vec![0; static_data.extra_field_length as usize];
        let mut file_comment_buf = vec![0; static_data.file_comment_length as usize];

        file.seek(SeekFrom::Start(end_static_offset)).expect("Couldn't seek to end of static offset");

        file.read(&mut file_name_buf).expect("Couldn't read filename");
        file.read(&mut extra_field_buf).expect("Couldn't read extra field");
        file.read(&mut file_comment_buf).expect("Couldn't read file comment");

        self.static_data = static_data;
        self.start_offset = start_offset;
        self.end_offset = end_static_offset + static_data.file_name_length as u64 + static_data.file_comment_length as u64 + static_data.extra_field_length as u64;
        self.file_name_data = file_name_buf;
        self.extra_field_data = extra_field_buf;
        self.file_comment_data = file_comment_buf;

        return self.end_offset;
    }
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
    comment_length: u16,                // 20       (n)
    // comment: Vec<u8>                 Moved to wrapper EofRecord
}

#[derive(Debug, Clone)]
/// Wrapper around EndOfCentralDirectoryRecord that allows us to manually fill the variably sized data
struct EofRecord {
    static_data: EndOfCentralDirectoryRecord,
    start_offset: u64,
    end_offset: u64,
    comment: Vec<u8>,
}

impl EofRecord {
    pub fn new(mut file: &std::fs::File, offset_starting: u64) -> EofRecord {
        let mut static_data = EndOfCentralDirectoryRecord::new();
        let end_offset = static_data.load_data(&mut file, offset_starting);
        let mut comment_buf = vec![0; static_data.comment_length as usize];
        file.seek(SeekFrom::Start(end_offset)).expect("Couldn't seek to EOF comment");
        file.read(&mut comment_buf).expect("Error reading EOF comment");

        return EofRecord{
            static_data: static_data,
            start_offset: offset_starting,
            end_offset: end_offset,
            comment: comment_buf
        }
        
    }
}

impl EndOfCentralDirectoryRecord {
    /// Reads a binary array into a struct, using the C representaion
    /// Returns a offset of where the reading ended
    /// https://stackoverflow.com/questions/25410028/how-to-read-a-struct-from-a-file-in-rust
    pub fn load_data(&mut self, mut file: &std::fs::File, offset_starting: u64) -> u64{
        println!("Loading EOF Record from offset: {:#X}", offset_starting);
        let data_size = mem::size_of::<EndOfCentralDirectoryRecord>();
        let mut struct_data = vec![0u8; data_size];

        file.seek(SeekFrom::Start(offset_starting)).expect("Couldn't seek to start of EOF Record");
        file.read(&mut struct_data).expect("Couldn't read from file.");

        let mut data: EndOfCentralDirectoryRecord = unsafe {mem::zeroed()};
        

        let mut c = Cursor::new(struct_data);

        unsafe {
            let data_slice = slice::from_raw_parts_mut(&mut data as *mut _ as *mut u8, data_size);
            c.read_exact(data_slice).expect("Couldn't read data into struct.");
        }

        self.magic_number = data.magic_number;
        self.number_of_current_disk = data.number_of_current_disk;
        self.disk_where_cdr_starts = data.disk_where_cdr_starts;
        self.num_cdr_on_disk = data.num_cdr_on_disk;
        self.total_cdr = data.total_cdr;
        self.size_of_cdr = data.size_of_cdr;
        self.offset_cdr_start = data.offset_cdr_start;
        self.comment_length = data.comment_length;

        return offset_starting + data_size as u64;
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

pub struct ZipArchive<'a> {
    filename: &'a str,
    local_file_data: Vec<LocalFileHeader>,
    central_records: Vec<CentralDirectoryFileHeader>,
    eof_record: EofRecord
}


impl ZipArchive<'_> {
    pub fn new(filename: &str) -> ZipArchive{
        println!("New ZipArchive! {}", filename);
        let path = Path::new(filename);
        let mut file = match File::open(path) {
            Err(why) => panic!("Couldn't open {}: {}", path.display(), why.to_string()),
            Ok(file) => file
        };

        let last_pos = match file.seek(SeekFrom::End(0)) {
            Err(why) => panic!("Couldn't seek! {}", why.to_string()),
            Ok(pos) => pos
        };

        let eof_record_num:[u8; 4] = [0x50, 0x4b, 0x05, 0x06]; // 0x06054b50 Reversed for lil-endian

        let mut current_index: i64 = 1;
        while current_index < last_pos as i64 { // basically, this loop moves the read position back 1 byte at a time from the end, until our
            // four-byte buffer looks like the eof_record_num, which means we have found the start of the EOF record.
            let mut buffer: [u8; 4] = [0x0; 4];
            file.seek(SeekFrom::End(-current_index)).unwrap();
            file.read(&mut buffer[..]).unwrap();
            if &eof_record_num[..] == &buffer[..] {
                println!("Found magic number for EOF structure at offset {:#X}", last_pos-current_index as u64);
                break;
            }
            current_index = current_index + 1;
        }

        let eofdirectory_offset: u64 = last_pos - current_index as u64;

        return ZipArchive{
            filename: filename,
            local_file_data: Vec::new(),
            central_records: Vec::new(),
            eof_record: EofRecord::new(&mut file, eofdirectory_offset)
        };
    }

    pub fn print_eof(self){
        println!("EofRecord: {:#?}", self.eof_record);
    }

    pub fn test_cdr_read(self){
        let start_offset = self.eof_record.static_data.offset_cdr_start;

        let path = Path::new(self.filename);
        let mut file = match File::open(path) {
            Err(why) => panic!("Couldn't open {}: {}", path.display(), why.to_string()),
            Ok(file) => file
        };

        let mut x = CDFHR::new();
        let mut y = CDFHR::new();
        let _done = x.load_data(&mut file, start_offset as u64);
        let _done2 = y.load_data(&mut file, _done as u64);
        println!("Data1: {:#?}", x);
        let filename1 = std::str::from_utf8(&x.file_name_data).expect("Couldn't convert bytes to utf8");
        println!("Data1 file: {}", filename1);
        println!("Data2: {:#?}", y);
        let filename2 = std::str::from_utf8(&y.file_name_data).expect("Couldn't convert bytes to utf8");
        println!("Data2 file: {}", filename2);
    }
}