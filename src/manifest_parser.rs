use std::collections::HashMap;

use anyhow::Result;
pub fn parse_manifest(manifest: String, is_asc: bool) -> Result<HashMap<String, String>> {
    let mut filename_hash_pairs = HashMap::<String, String>::new();
    let lines = manifest.lines();
    for line in lines {
        match line {
            "" => {
                continue;
            }
            "-----BEGIN PGP SIGNED MESSAGE-----" => {
                continue;
            }
            "-----BEGIN PGP SIGNATURE-----" => {
                break;
            }
            "Hash: SHA256" => {
                continue;
            }
            _ => {
                let split_line: Vec<&str> = line.trim().split_whitespace().collect();
                if split_line.len() == 2 {
                    let maybe_hash = split_line[0];
                    println!("hash: {}", maybe_hash);
                    if maybe_hash.as_bytes().len() != 64 {
                        println!("that's not a hash bro");
                        continue;
                    }
                    let maybe_filename = split_line[1].trim_start_matches("*");
                    println!("filename: {}", maybe_filename);
                    filename_hash_pairs.insert(maybe_filename.to_string(), maybe_hash.to_string());
                } else {
                    println!("didn't split right: {}", line);
                }
            }
        }
    }

    Ok(filename_hash_pairs)
}
