/*
 * Encrypt the username/password for use in `EncryptedPasswd`.
 * refs:
 * - https://github.com/yeriomin/play-store-api/blob/master/src/main/java/com/github/yeriomin/playstoreapi/PasswordEncrypter.java
 * - https://github.com/subtletech/google_play_store_password_encrypter/blob/master/google_play_store_password_encrypter.rb
 *
 *	The result is something like the below
 *	-----------------------------------------------------------------------------
 *	|00|4 bytes of sha1(publicKey)|rsaEncrypt(publicKeyPem, "login\x00password")|
 *	-----------------------------------------------------------------------------
 */

use base64;
use openssl::{bn, pkey, rsa};
use ring::{self, digest};
use rpassword::prompt_password_stderr;

pub struct EncryptedPw(Vec<u8>);

impl EncryptedPw {
    pub fn to_url_safe_base64(&self) -> String {
        base64::encode(&self.0).replace('+', "-").replace('/', "_")
    }
}
const GOOGLE_DEFAULT_PUBLIC_KEY: &str = "AAAAgMom/1a/v0lblO2Ubrt60J2gcuXSljGFQXgcyZWveWLEwo6prwgi3iJIZdodyhKZQrNWp5nKJ3srRXcUW+F1BD3baEVGcmEgqaLZUNBjm057pKRI16kB0YppeGx5qIQ5QjKzsR8ETQbKLNWgRY0QRNVz34kMJR3P/LgHax/6rmf5AAAAAwEAAQ==";

pub fn input_password_and_encrypt(username: &str) -> EncryptedPw {
    let password: String = String::from("examplepassword"); // prompt_password_stderr("Password: ").unwrap();
    encrypt_login(username, password)
}

/// Decompose Google's public key into modulus and exponent components into RSA Public key
fn decompose(public_key: &[u8]) -> rsa::Rsa<pkey::Public> {
    let u32_len = 4;
    let mod_len = read_u32_be(&public_key) as usize;
    let mod_offset = u32_len;
    let mod_bin = &public_key[mod_offset..mod_offset + mod_len];
    let exp_offset = mod_offset + mod_len + u32_len;
    let exp_len = read_u32_be(&public_key[mod_len + mod_offset..]) as usize;
    let exp_bin = &public_key[exp_offset..exp_offset + exp_len];

    let n_mod = bn::BigNum::from_slice(mod_bin).unwrap();
    let e_exp = bn::BigNum::from_slice(exp_bin).unwrap();

    rsa::Rsa::<pkey::Public>::from_public_components(n_mod, e_exp).unwrap()
}

fn encrypt_login(username: &str, password: String) -> EncryptedPw {
    let public_key: Vec<u8> = base64::decode(GOOGLE_DEFAULT_PUBLIC_KEY).unwrap();

    let mut data: Vec<u8> = username.to_string().into_bytes();
    data.push(0u8);
    data.append(&mut password.into_bytes());

    let mut context = digest::Context::new(&digest::SHA1);
    context.update(&public_key);
    let digest = context.finish();
    let digest_ref = digest.as_ref();
    let signature: Vec<u8> = vec![
        0u8,
        digest_ref[0],
        digest_ref[1],
        digest_ref[2],
        digest_ref[3],
    ];
    let signature_len = signature.len();

    let rsa = decompose(&public_key);

    let mut encrypted_buf = vec![0u8; rsa.size() as usize];
    let mut res = signature;
    res.append(&mut encrypted_buf);

    let encrypted_len = rsa
        .public_encrypt(&data, &mut res[signature_len..], rsa::Padding::PKCS1)
        .expect("public encrypt into buf works");

    EncryptedPw(res[..signature_len + encrypted_len].into())
}

/// Read the first 4 bits as a u32 big endian
///
///  # Panics
///
/// Panics if slice is smaller than 4 elements
fn read_u32_be(slice: &[u8]) -> u32 {
    as_u32_be(&pop4(&slice).unwrap())
}

// Source: https://stackoverflow.com/a/36676814/2096729
fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + ((array[3] as u32) << 0)
}

fn pop4(four: &[u8]) -> Result<[u8; 4], &'static str> {
    if four.len() < 4 {
        Err("pop4: slice length less than 4")
    } else {
        Ok([four[0], four[1], four[2], four[3]])
    }
}

#[cfg(test)]
mod test {
    const GOOGLE_DEFAULT_PEM: &str = r#"-----BEGIN RSA PUBLIC KEY-----
MIGJAoGBAMom/1a/v0lblO2Ubrt60J2gcuXSljGFQXgcyZWveWLEwo6prwgi3iJI
ZdodyhKZQrNWp5nKJ3srRXcUW+F1BD3baEVGcmEgqaLZUNBjm057pKRI16kB0Ypp
eGx5qIQ5QjKzsR8ETQbKLNWgRY0QRNVz34kMJR3P/LgHax/6rmf5AgMBAAE=
-----END RSA PUBLIC KEY-----
"#;

    use super::*;

    #[test = "Check decompose returns expected value"]
    fn decompose_test() {
        let public_key = base64::decode(GOOGLE_DEFAULT_PUBLIC_KEY).unwrap();
        let rsa = decompose(&public_key);
        let rsa_pem_string =
            String::from_utf8(rsa.public_key_to_pem_pkcs1().expect("to pem")).expect("from utf8");
        assert_eq!(
            &rsa_pem_string, GOOGLE_DEFAULT_PEM,
            "PEM from decompose matches expected PEM"
        );
    }
}
