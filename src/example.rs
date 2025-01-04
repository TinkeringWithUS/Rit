use std::fs::File;

use zip::write::{ExtendedFileOptions, FileOptionExtension, FileOptions};

pub fn doit() -> zip::result::ZipResult<()>
{
    use std::io::Write;

    // For this example we write to a buffer, but normally you should use a File
    let mut buf: &mut [u8] = &mut [0u8; 65536];
    let mut w = std::io::Cursor::new(buf);

    let archive_file = File::create("archive.bzip");

    let unwrapped = archive_file.unwrap();

    let mut zip = zip::ZipWriter::new(unwrapped);

    let options: FileOptions<ExtendedFileOptions> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Bzip2);
    zip.start_file("archive.bzip", options)?;

    let text_bytes = ("Contrary to popular belief, Lorem Ipsum is not simply random text. It has roots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock, a Latin professor at Hampden-Sydney College in Virginia, looked up one of the more obscure Latin words, consectetur, from a Lorem Ipsum passage, and going through the cites of the word in classical literature, discovered the undoubtable source. Lorem Ipsum comes from sections 1.10.32 and 1.10.33 of".repeat(10)) ;

    let write_result = zip.write(text_bytes.as_bytes())?;

    println!("number of bytes written: {}", write_result);

    println!("inside doit, after writing hello world");

    // Optionally finish the zip. (this is also done on drop)
    zip.finish()?;

    Ok(())
}