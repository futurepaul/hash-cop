use digest::Digest;
use hex_view::HexView;
use std::fs::{self, File};
use std::io::{self, Read};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

// Lifted from https://github.com/pop-os/popsicle/blob/1685d8004b6d11a23f0fd0e1440cf0cc9c8e8d2a/gtk/src/hash.rs
pub(crate) fn hasher<H: Digest>(file_path: &PathBuf) -> io::Result<String> {
    let metadata = fs::metadata(file_path.clone())?;
    let file_size = metadata.len();
    let how_many_buffers_guess = file_size / 8192;

    File::open(file_path).and_then(move |mut file| {
        let mut buffer = [0u8; 8 * 1024];
        let mut hasher = H::new();

        let mut how_many_buffers = 0;

        loop {
            how_many_buffers += 1;
            let read = file.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            hasher.input(&buffer[..read]);
        }

        println!(
            "We did {} buffers. File size was {} bytes. My guess was {} buffers.",
            how_many_buffers, file_size, how_many_buffers_guess
        );

        Ok(format!("{:x}", HexView::from(hasher.result().as_slice())))
    })
}

