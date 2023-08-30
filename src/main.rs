use rand::Rng;
use std::fs::{File, self};
use std::io::{Read, Write};
use std::ops::Add;
use std::process::abort;
use clap::Parser;

#[derive(Parser, Debug)]
struct Enc {
    /// specify the file you want to encode/decode
    #[clap()]
    file_path: String,
    
    /// specify where you want to put the encoded/decoded file (if this isn't used the output file will replace the original file)
    #[clap(long, short)]
    output_file_path: Option<String>,

    /// decode an encoded file with the key file (the key file has an extension of .crolkey)
    #[clap(long, short)]
    key_file: Option<String>,
}


fn main() {
    let file_path = Enc::parse().file_path;
    if fs::metadata(&file_path).is_ok() {
        match Enc::parse().key_file {
            Some(key_path) => {
                if !key_path.contains(".crolkey") {
                    println!("Invalid key file name (the key file has to end with .crolkey)");
                    abort();
                } else {
                    create_file(decode(&key_path, &read_file(&file_path)));
                    if Enc::parse().output_file_path.is_some() {
                        println!("File `{}` decoded and saved as `{}`", file_path, output_file_path())
                    } else {
                        println!("File `{}` decoded", output_file_path())
                    }
                }
            }
            None => {
                create_file(encode(&read_file(&file_path)));
                println!("DO NOT:\nmodify the inner contents of the encoded file or the key (if you do you wont be able to decode it)\nencode the encoded file and output it with the same name (this overwrites the key so you will be able to decode it one time but the second time wont work)\nchange the .crolkey extention (if you change it the app wont work) if you want to change it just edit the source code");
                if Enc::parse().output_file_path.is_some() {
                    println!("File `{}` encoded and saved as `{}`", file_path, output_file_path())
                } else {
                    println!("File `{}` encoded", output_file_path())
                }
            }
        }
    } else {
        println!("File `{}` dosn't exist.", file_path)
    }
}


fn read_file(file_path: &String) -> String {
    let mut contents = String::new();
    match File::open(file_path) {
        Ok(mut file) => if let Err(err) = file.read_to_string(&mut contents) { eprintln!("Error reading file: {}", err) },
        Err(err) => eprintln!("Error reading file: {}", err)
    }

    contents
}


fn decode(key_path: &String, contents: &String) -> String {
    let mut dec_contents = String::new();
    let key = read_file(&key_path);

    if key.split_whitespace().count() == contents.bytes().count() {
        for byte in key.split_whitespace() {
            if let Ok(byte) = byte.parse::<u8>() {
                dec_contents.push(char::from(byte));
            } 
        }
    } else {
        println!("Wrong key for this file or you have modified the file or the key in some way.");
        abort();
    }

    dec_contents
}


fn encode(contents: &String) -> String {
    let mut enc_contents = String::new();
    let mut key = String::new();
    let bytes = &contents.bytes();


    for byte in bytes.clone() {
        let random = rand::thread_rng().gen_range(33..126);
        key.push_str(&byte.to_string().add(" "));
        enc_contents.push(char::from(random));
    }
    create_key_file(key);

    enc_contents
}


fn create_file(contents: String) {
    let mut file = match File::create(output_file_path()) {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating file: {}", e);
            return;
        }
    };

    if let Err(e) = write!(file, "{}", contents) {
            println!("Error writing to file: {}", e);
    }
}

fn create_key_file(key: String) {
    let mut file_path = output_file_path();
    file_path.push_str(".crolkey");

    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating file: {}", e);
            return;
        }
    };

    if let Err(e) = write!(file, "{}", key) {
            println!("Error writing to file: {}", e);
    }
}

fn output_file_path() -> String {
    let file_path = match Enc::parse().output_file_path {
        Some(file_path) => file_path,
        None => Enc::parse().file_path
    };
    file_path
}