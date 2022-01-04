use sha1::Digest;
use std::io::{BufRead, BufReader};
use std::{env, error::Error, fs::File};

const SHA1_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1>");
        return Ok(());
    }

    let sha1_to_crack = args[2].trim();
    if sha1_to_crack.len() != SHA1_LENGTH {
        return Err(format!("sha1 '{}' is invalid.", sha1_to_crack).into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        let hexed = &hex::encode(sha1::Sha1::digest(common_password.as_bytes()));

        if hexed == sha1_to_crack {
            println!("Success: {}", &common_password);
            return Ok(());
        }
    }
    println!("None matched :(");

    Ok(())
}
