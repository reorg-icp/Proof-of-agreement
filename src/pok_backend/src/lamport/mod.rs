use sha2::{Digest, Sha256};

use crate::agreement::{self, Agreement};

const KEY_SIZE: usize = 256;
const KEY_ELEMENT_SIZE: usize = 32;

#[derive(Debug)]
pub struct PrivateKey {
    key_pairs: Vec<(String, String)>,
}

pub struct PublicKey {
    key_pairs: Vec<(String, String)>,
}

#[derive(Clone, Debug, candid::CandidType, Serialize, Deserialize)]

pub struct Signature {
    signatures: Vec<String>,
}

impl PrivateKey {
    pub fn get_key(&self, i: usize) -> (String, String) {
        self.key_pairs[i].clone()
    }
}

impl PublicKey {
    pub fn get_key(&self, i: usize) -> (String, String) {
        self.key_pairs[i].clone()
    }
}

impl Signature {
    pub fn get_key(&self, i: usize) -> String {
        self.signatures[i].clone()
    }
}
/// Generates a random but cryptographically secure private key
///
/// # Examples
/// ```rust
///  
///  let private_key= lsig::random_private_key();
///  
///
///
/// ```
// pub fn random_private_key() -> PrivateKey {
//     let mut private_key: Vec<(String, String)> = Vec::with_capacity(KEY_SIZE);
//     for _i in 0..KEY_SIZE {
//         private_key.push((random_string(), random_string()));
//     }
//     PrivateKey {
//         key_pairs: private_key,
//     }
// }

pub fn random_private_key(principal: String, agreement: Agreement, nounce: String) -> PrivateKey {
    let hashed_principal = hash(&principal);

    let mut private_key: Vec<(String, String)> = Vec::with_capacity(KEY_SIZE);
    let mut hasher = Sha256::new();

    for i in 0..KEY_SIZE {
        hasher.update(&hashed_principal);
        hasher.update(&nounce);
        hasher.update(agreement.by_user.clone().identity);
        hasher.update(agreement.with_user.clone().identity);
        hasher.update(agreement.date.as_str());
        for term in agreement.terms.iter() {
            hasher.update(term);
        }
        hasher.update(&i.to_be_bytes());
        let hash_output = hasher.finalize_reset();
        let key_str1 = hex::encode(&hash_output[0..KEY_ELEMENT_SIZE / 2]);
        let key_str2 = hex::encode(&hash_output[KEY_ELEMENT_SIZE / 2..KEY_ELEMENT_SIZE]);
        private_key.push((key_str1, key_str2));
    }

    PrivateKey {
        key_pairs: private_key,
    }
}

/// Hash a string slice.

pub fn hash(str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(str);
    hex::encode(hasher.finalize())
}
/// Create a public key from the generated private key
///  # Example
/// ```rust
/// let private_key= lsig::random_private_key();
/// let public_key=lsig::create_public_key(&private_key);
///
/// ```
pub fn create_public_key(private_key: &PrivateKey) -> PublicKey {
    let mut public_key: Vec<(String, String)> = Vec::with_capacity(KEY_SIZE);
    for item in private_key.key_pairs.iter() {
        let (first_key, second_key) = item;
        public_key.push((hash(first_key), hash(second_key)));
    }
    PublicKey {
        key_pairs: public_key,
    }
}

fn hash_to_binary_array(hash_string: String) -> Vec<u8> {
    let message = hex::decode(hash_string);
    let mut str_binary_array: Vec<u8> = Vec::with_capacity(KEY_SIZE);
    match message {
        Ok(bytes) => {
            for byte in bytes.iter() {
                for i in (0..8).rev() {
                    let bit = (byte >> i) & 1;
                    str_binary_array.push(bit);
                }
            }
            str_binary_array
        }
        Err(_error) => str_binary_array,
    }
}

/// Sign a message using the private key and get a signature. A message must be hashed first as shown below.
/// # Example
/// ```rust
/// let private_key= lsig::random_private_key();
/// let message= lsig::hash("My confidential message");
/// let signature=lsig::sign(message, &private_key);
///
///
/// ```

pub fn sign(message_hash: String, private_key: &PrivateKey) -> Signature {
    let message_binary_array = hash_to_binary_array(message_hash);
    let mut signature_array: Vec<String> = Vec::with_capacity(KEY_SIZE);
    for (index, item) in message_binary_array.iter().enumerate() {
        let (first_key, second_key) = private_key.get_key(index);
        if *item == 0 {
            signature_array.push(first_key);
        } else {
            signature_array.push(second_key);
        }
    }
    Signature {
        signatures: signature_array,
    }
}

/// Verify a message using the the signature and the public key
/// # Example
/// ```rust
/// let private_key= lsig::random_private_key();
/// let public_key=lsig::create_public_key(&private_key);
/// let message= lsig::hash("My confidential message");
/// let signature=lsig::sign(message.clone(), &private_key);
///let message_is_authentic= lsig::verify(message.clone(), &signature,&public_key);
/// let not_authentic=lsig::hash("Not authentic");
/// let message_is_not_authentic= lsig::verify(not_authentic.clone(), &signature,&public_key);
/// assert_eq!(true, message_is_authentic);
/// assert_eq!(false,message_is_not_authentic);
///
///
/// ```

pub fn verify(message_hash: String, signature: &Signature, public_key: &PublicKey) -> bool {
    let message_binary_array = hash_to_binary_array(message_hash);
    for (index, item) in message_binary_array.iter().enumerate() {
        let sig = signature.get_key(index);
        let private_key_hash = hash(&sig);
        let (first_pub_key_hash, second_pub_key_hash) = public_key.get_key(index);
        if *item == 0 {
            if private_key_hash != first_pub_key_hash {
                return false;
            }
        } else if private_key_hash != second_pub_key_hash {
            return false;
        }
    }
    true
}
#[cfg(test)]
mod tests {
    use crate::_create_new_agreement;

    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let terms: Vec<String> = vec![
        "I am the Lord thy God".to_string(),
        "Thou shalt have no other gods before me".to_string(),
        "Thou shalt not make unto thee any graven image".to_string(),
        "Thou shalt not take the name of the Lord thy God in vain".to_string(),
        "Remember the sabbath day, to keep it holy".to_string(),
        "Honour thy father and thy mother".to_string(),
        "Thou shalt not kill".to_string(),
        "Thou shalt not commit adultery".to_string(),
        "Thou shalt not steal".to_string(),
        "Thou shalt not bear false witness against thy neighbour".to_string(),
        "Thou shalt not covet thy neighbour's house".to_string(),
        "Thou shalt not covet thy neighbour's wife, nor his manservant, nor his maidservant, nor his ox, nor his ass, nor any thing that is thy neighbour's".to_string(),
    ];
        let agreement = _create_new_agreement(terms, String::from("God"), 1);

        let private_key = random_private_key(
            String::from("amschel"),
            agreement,
            String::from("My secret"),
        );

        let public_key = create_public_key(&private_key);
        let message_hash = hash("Hello, world!");
        let message_hash2 = hash("Hello");
        let signature = sign(message_hash.clone(), &private_key);
        dbg!(private_key.key_pairs);
        assert_eq!(verify(message_hash.clone(), &signature, &public_key), true);
        assert_eq!(
            verify(message_hash2.clone(), &signature, &public_key),
            false
        );
        // Change a bit in the message hash to make verification fail
        let mut message_hash_bytes = hex::decode(message_hash).unwrap();
        message_hash_bytes[0] ^= 1; // Flipping first bit
        let modified_message_hash = hex::encode(message_hash_bytes);
        assert_eq!(
            verify(modified_message_hash, &signature, &public_key),
            false
        );
    }

    #[test]

    fn test_hash_to_binary_array() {
        let hash_string = "1234567890abcdef";
        let binary_array = hash_to_binary_array(hash_string.to_string());

        // Ensure binary array has correct length and values
        assert_eq!(binary_array.len(), hash_string.len() * 4);

        for (i, hex_char) in hash_string.chars().enumerate() {
            let hex_value = hex_char.to_digit(16).unwrap();
            for j in (0..4).rev() {
                let bit = (hex_value >> j) & 1;
                assert_eq!(binary_array[i * 4 + (3 - j) as usize], bit as u8);
            }
        }
    }
}
