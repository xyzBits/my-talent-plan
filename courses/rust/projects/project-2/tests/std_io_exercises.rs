use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

#[test]
fn test_read_file() -> io::Result<()> {
    let mut file = File::open("1.log")?;

    let mut buffer = [0; 10];

    let len = file.read(&mut buffer)?;

    println!("The bytes: {:?}", &buffer[..len]);

    println!("{}", String::from_utf8_lossy(&buffer));

    Ok(())
}

#[test]
fn test_seek_and_buf_read() -> io::Result<()> {
    // Seek lets you control where the next bytes is coming from

    let mut file = File::open("1.log")?;
    let mut buffer = [0; 10];

    // skip to then last 10 bytes of the file
    file.seek(SeekFrom::End(-10))?;

    let len = file.read(&mut buffer)?;


    Ok(())
}



#[test]
fn test_buf_reader() -> io::Result<()>{
    // Byte-based interfaces are unwieldy and can be inefficient.

    let file = File::open("1.log")?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    // read a line into buffer
    reader.read_line(&mut buffer)?;


    println!("{}", buffer);

    Ok(())
}