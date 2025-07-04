use std::fs::File;
use std::io::{Error, ErrorKind, Read};

// unwrap
// expect


// return Errors: return Ok(T) or Err(E)
fn get_file_contents(fname: &str) -> Result<String, Error> {
    let mut buff = String::new();
    match File::open(fname) {
        Ok(mut fh) => {
            match fh.read_to_string(&mut buff) {
                Ok(_size) => Ok(buff),
                Err(e) => Err(e),
            }
        },
        Err(error) => Err(error)
    }
}

// return Errors: return Ok(T) or Err(E) with ?
fn get_file_contents_v2(fname: &str) -> Result<String, Error> {
    let mut buff = String::new();
    File::open(fname)?.read_to_string(&mut buff)?;
    Ok(buff)
}

fn main (){

    let file_name = "/Users/mmv6113/RustProjects/hello_world/parse_xml.py";
    
    let mut file = match File::open(file_name) {
        Ok(fh) => fh,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create(file_name) { // returns a mutes reference
                    Ok(fc) => fc,
                    Err(e) => panic!("Error at file creation {:?}", e),
                }
            },
            other_error => {
                panic!("Problem opening file {:?}", other_error)
            },
        },
    };

    let mut contents:String = String::new();
    let _size = file.read_to_string(&mut contents);

    println!("{}\n\n\n\n",contents);

    // *************************************************************
    // alternative
    // *************************************************************
    let mut fh = File::open(file_name)
    .unwrap_or_else(
        |error| 
        if error.kind() == ErrorKind::NotFound { 
            File::create(file_name)
            .unwrap_or_else(|e| panic!("Error: {:?}", e)) 
        } else {
            panic!("Error: {:?}", error)
        } 
    );

    let mut contents:String = String::new();
    let _size = fh.read_to_string(&mut contents);

    println!("{}\n\n\n\n\n",contents);

    println!("{}\n\n\n\n\n", get_file_contents(file_name).unwrap());

    println!("{}\n\n\n\n\n", get_file_contents_v2(file_name).unwrap());

}