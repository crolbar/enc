use rand::Rng;
use std::fs;
use std::io::{self, Read, Write};
use clap::Parser;
mod gui;

#[derive(Parser, Debug)]
struct Enc {
    /// specify the file you want to encode/decode
    #[clap()]
    file_path: Option<String>,
    
    /// specify where you want to put the encoded/decoded file (if this isn't used the output file will replace the original file)
    #[clap(long, short)]
    output_file_path: Option<String>,

    /// decode an encoded file with the key file (the key file has an extension of .key)
    #[clap(long, short)]
    key_file: Option<String>,

    /// use the gui
    #[clap(long, short)]
    gui: bool
}


fn main() {
    let args = Enc::parse();
    if args.file_path.is_none() { // if there is none file_path and if -g is passed run the gui 
        if args.gui {             
            let _ = gui::main();
            return;
        } else { // if -g is not passed print the help massage
            let out = std::process::Command::new("enc").arg("-h").output().expect("Error in running `enc` command:");
            println!("{}", String::from_utf8_lossy(&out.stdout));
            return;
        }
    };

    let cli_output = format_enc_dec(args).unwrap_or_else(|err| err.to_string());
    println!("{}",cli_output);
}

pub fn gui_main(file_path: &String, output_path: String, key_path: &String) -> String {
    let key:  Option<String> = if key_path.is_empty() {
        None
    } else {
        Some(key_path.to_string())
    };

    let args = Enc {
        file_path: Some(file_path.to_string()),
        output_file_path: Some(output_path),
        key_file: key,
        gui: false
    };


    let cli_output = format_enc_dec(args).unwrap_or_else(|err| err.to_string());
    println!("{}", cli_output);
    cli_output
}

fn format_enc_dec(args: Enc) -> io::Result<String> {
    let mut _cli_output;
    let mut key = Vec::new();
    let mut contents = Vec::new();
    let file_path = &args.file_path.unwrap();
    let output_file_path = args.output_file_path.unwrap_or_else(|| output_file_path());

    fs::File::open(file_path)?.read_to_end(&mut contents)?;
    

    // key reading/generation
    if let Some(key_path) = &args.key_file { // if there is an key file specified read it
        fs::File::open(key_path)?.read_to_end(&mut key)?;
    } else if encoded(&contents) { // if the file is encoded aka the first three chars are "enC"
        _cli_output = format!("File `{}` is already encoded.", file_path);
        return Ok(_cli_output);
    } else { // if there is not an key file specified and the file is not encoded generate an key file
        let mut key_file = output_file_path.clone();
        key_file.push_str(".key");
        for _ in 0..5 { // idk why im doing this but why not
            key.push(rand::thread_rng().gen_range(0..255));
        }
        fs::File::create(&key_file)?.write_all(&key)?;
    }

    // encoding/decoding
    if !encoded(&contents) && (args.key_file.is_none() || args.key_file.clone().unwrap().contains(".key")) { // if the file is not encoded and keyfile is either none or contains the .key file extention (just quick fix for bug that allowed encrypting an file with a file)
        contents = enc_dec(&contents, &key);                                                          // encode it and add "enC" in frot of the contents of the file along with the key id
        let id = usize::from(key_id(&key));
        contents.splice(0..0, vec![101, 110, 67, id as u8]);
        _cli_output = format!("File `{}` encoded", file_path);
    } else  { // if the file is encoded, decode it if we are using the right key and remove the temp values for identification
        if key_id(&key) == usize::from(contents[3]) && args.key_file.unwrap().contains(".key"){
            contents = enc_dec(&contents, &key);
            contents.drain(0..4);
            _cli_output = format!("File `{}` decoded", file_path);
        } else {
            _cli_output = format!("The key you provided is not the right key for the file `{}`", file_path);
            return Ok(_cli_output);
        }
    }


    fs::File::create(output_file_path)?.write_all(&contents)?;

    Ok(_cli_output)
}


fn key_id(key: &[u8]) -> usize { // used prevent from encoding an encoded file with another key
    let mut sum: usize = 0;      // make somewhat of an id for the key
    for &num in key {
        sum += usize::from(num);
    }

    sum / key.len()
}


fn enc_dec(chars: &[u8], key: &[u8]) -> Vec<u8> {
    let mut contents = chars.to_vec();

    for &key_byte in key.iter() {
        let mut temp = Vec::new();
        for byte in contents.iter() {
            temp.push(byte ^ key_byte);
        }
       contents = temp.clone();
    }

    contents
}

fn output_file_path() -> String {
    let file_path = Enc::parse().output_file_path.unwrap_or(Enc::parse().file_path.unwrap());
    file_path
}

fn encoded(contents: &Vec<u8>) -> bool {
    contents.iter().take(3).map(|&x| x as u32).sum::<u32>() == 278
}