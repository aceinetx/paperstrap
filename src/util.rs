use sha2::{Digest, Sha256};
use std::error::Error;
use std::path::Path;
use std::{fs, fs::File};
use std::{
    io,
    io::{BufReader, Read, Write, copy},
};

pub fn download(url: &str, destination: &Path) -> Result<(), Box<dyn Error>> {
    println!("downloading {} -> {}...", url, destination.display());

    let mut response = reqwest::blocking::get(url)?;

    let mut dest_file = File::create(destination)?;

    copy(&mut response, &mut dest_file)?;

    Ok(())
}

pub fn hash_file_sha256<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    // Open the file in read-only mode.
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Create a Sha256 object
    let mut hasher = Sha256::new();

    // Read the file in chunks so we don't need to load the entire file into memory.
    let mut buffer = [0; 1024 * 8]; // 8 KB buffer
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // end of file
        }
        hasher.update(&buffer[..bytes_read]);
    }

    // Retrieve hash digest as array of bytes
    let result = hasher.finalize();

    // Convert to hexadecimal string representation
    Ok(hex::encode(result))
}

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    _ = io::stdout().flush();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}

pub fn is_dir_empty_excluding_dotfiles(path: impl AsRef<Path>) -> io::Result<bool> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name();

        if !name.to_string_lossy().starts_with('.') {
            return Ok(false);
        }
    }

    Ok(true)
}
