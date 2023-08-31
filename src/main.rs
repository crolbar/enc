use std::fs; //asdddd
use std::io::{self, Read, Write};
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


fn main() -> io::Result<()> {
    let args = Enc::parse();
    let mut key = Vec::new();
    let mut contents = Vec::new();
    fs::File::open(&args.file_path)?.read_to_end(&mut contents)?;
    

    
    if let Some(key_path) = &args.key_file { // if there is an key file specified read it
        fs::File::open(key_path)?.read_to_end(&mut key)?;
    } else if encoded(&contents) { // if the file is encoded aka the first thre chars are "enC"
        println!("File `{}` is already encoded.", &args.file_path);
        std::process::exit(0);
    } else { // if there is not an key file specified and the file is not encoded generate an key file
        let mut key_file = args.file_path.clone();
        key_file.push_str(".key");
        // for _ in 0..5 { // idk why im doing this but why not
        //     key.push(rand::thread_rng().gen_range(0..255));
        // }
        key = vec![5];
        fs::File::create(&key_file)?.write_all(&key)?;
    }

    // prevent from encoding an encoded file
    if !encoded(&contents) { // if the file is not encoded, encode it and add "enC" in frot of the contents of the file 
        contents = enc_dec(&contents, &key);
        contents.splice(0..0, vec![101, 110, 67]);
    } else { // if the file is encoded remove "enC" and output the original file
        contents = enc_dec(&contents, &key);
        contents.drain(0..3);
    }


    fs::File::create(&output_file_path())?.write_all(&contents)?;
    Ok(())
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
    let file_path = Enc::parse().output_file_path.unwrap_or(Enc::parse().file_path);
    file_path
}

fn encoded(contents: &Vec<u8>) -> bool {
    contents.iter().take(3).map(|&x| x as u32).sum::<u32>() == 278
}
