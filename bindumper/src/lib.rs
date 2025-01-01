#![feature(ascii_char)]

use std::path::Path;
use std::io::Read;
use cbc::cipher::{KeyIvInit, BlockDecryptMut};
use cbc::cipher::block_padding::Pkcs7;
use flate2::read::GzDecoder;

type DesCbc = cbc::Decryptor<des::Des>;

const KEY: [u8; 8] = [48, 239, 114, 71, 66, 242, 4, 50];
const IV: [u8; 8] = [14, 166, 220, 137, 219, 237, 220, 79];

#[derive(Debug)]
pub enum DecryptError {
    IoError(std::io::Error),
    CipherError(cbc::cipher::InvalidLength),
    PaddingError,
    DecompressionError(std::io::Error),
}

impl From<std::io::Error> for DecryptError {
    fn from(err: std::io::Error) -> Self {
        DecryptError::IoError(err)
    }
}

impl From<cbc::cipher::InvalidLength> for DecryptError {
    fn from(err: cbc::cipher::InvalidLength) -> Self {
        DecryptError::CipherError(err)
    }
}

pub fn decrypt_bytes(input: &mut [u8]) -> Result<Vec<u8>, DecryptError> {
    // Decrypt the bytes
    let cipher = DesCbc::new_from_slices(&KEY, &IV)?;
    let decrypted = cipher.decrypt_padded_mut::<Pkcs7>(input)
        .map_err(|_| DecryptError::PaddingError)?;

    // // Decompress the bytes
    let mut decompressor = GzDecoder::new(decrypted);
    let mut decompressed = Vec::new();
    decompressor.read_to_end(&mut decompressed)
        .map_err(DecryptError::DecompressionError)?;

    Ok(decompressed)
}

pub fn load_file<P: AsRef<Path>>(input: P) -> Result<Vec<u8>, DecryptError> {
    let mut bytes = std::fs::read(input).map_err(DecryptError::IoError)?;
    decrypt_bytes(&mut bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_settings() {
        let mut encrypted = include_bytes!("../test/settings.bin").to_vec();
        let decrypted = include_bytes!("../test/decoded/settings.xml");
        let dump = decrypt_bytes(&mut encrypted).unwrap();
        assert!(dump == decrypted);
    }

    #[test]
    fn validate_mobfactions() {
        let mut encrypted = include_bytes!("../test/mobfactions.bin").to_vec();
        let decrypted = include_bytes!("../test/decoded/mobfactions.xml");
        let dump = decrypt_bytes(&mut encrypted).unwrap();
        assert!(dump == decrypted);
    }

    #[test]
    fn validate_spells() {
        let mut encrypted = include_bytes!("../test/spells.bin").to_vec();
        let decrypted = include_bytes!("../test/decoded/spells.xml");
        let dump = decrypt_bytes(&mut encrypted).unwrap();
        assert!(dump == decrypted);
    }
}
