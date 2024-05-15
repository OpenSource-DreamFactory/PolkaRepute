// privacy.rs
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use base64::{encode, decode};
use rand::{thread_rng, RngCore};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub struct Privacy {
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl Privacy {
    /// Creates a new instance of Privacy with optional key and IV.
    /// If key or IV is not provided, they are generated securely.
    pub fn new(key: Option<Vec<u8>>, iv: Option<Vec<u8>>) -> Result<Self, &'static str> {
        let mut final_key = vec![0u8; 32];
        let mut final_iv = vec![0u8; 16];

        match key {
            Some(k) => {
                if k.len() != 32 {
                    return Err("Key must be 32 bytes for AES256.");
                }
                final_key.copy_from_slice(&k);
            },
            None => thread_rng().fill_bytes(&mut final_key),
        }

        match iv {
            Some(i) => {
                if i.len() != 16 {
                    return Err("IV must be 16 bytes for AES256 CBC mode.");
                }
                final_iv.copy_from_slice(&i);
            },
            None => thread_rng().fill_bytes(&mut final_iv),
        }

        Ok(Privacy { key: final_key, iv: final_iv })
    }

    /// Encrypts the provided data using AES256 CBC mode.
    /// Returns a base64 encoded string of the encrypted data.
    pub fn encrypt_data(&self, data: &str) -> Result<String, &'static str> {
        let cipher = Aes256Cbc::new_var(&self.key, &self.iv).map_err(|_| "Encryption setup failed")?;
        let ciphertext = cipher.encrypt_vec(data.as_bytes());
        Ok(encode(&ciphertext))
    }

    /// Decrypts the provided base64 encoded data using AES256 CBC mode.
    /// Returns the decrypted data as a String.
    pub fn decrypt_data(&self, data: &str) -> Result<String, &'static str> {
        let cipher = Aes256Cbc::new_var(&self.key, &self.iv).map_err(|_| "Decryption setup failed")?;
        let decrypted_data = decode(data).map_err(|_| "Base64 decode failed")?;
        let decrypted_ciphertext = cipher.decrypt_vec(&decrypted_data).map_err(|_| "Decryption failed")?;
        String::from_utf8(decrypted_ciphertext).map_err(|_| "UTF-8 conversion failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let privacy_with_provided_key_iv = Privacy::new(Some(vec![0; 32]), Some(vec![0; 16])).expect("Failed to create Privacy instance with provided key and IV");
        let data = "Hello, world!";
        let encrypted_data = privacy_with_provided_key_iv.encrypt_data(data).expect("Encryption failed");
        let decrypted_data = privacy_with_provided_key_iv.decrypt_data(&encrypted_data).expect("Decryption failed");
        assert_eq!(data, decrypted_data);

        let privacy_with_generated_key_iv = Privacy::new(None, None).expect("Failed to create Privacy instance with generated key and IV");
        let encrypted_data_gen = privacy_with_generated_key_iv.encrypt_data(data).expect("Encryption failed with generated key and IV");
        let decrypted_data_gen = privacy_with_generated_key_iv.decrypt_data(&encrypted_data_gen).expect("Decryption failed with generated key and IV");
        assert_eq!(data, decrypted_data_gen);
    }
}
