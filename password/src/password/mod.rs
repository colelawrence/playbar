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
use openssl::{bn, pkey, rsa, sha};
use rpassword::prompt_password_stderr;

mod util;

pub struct EncryptedPw(Vec<u8>);

impl EncryptedPw {
    pub fn to_url_safe_base64(&self) -> String {
        base64::encode(&self.0).replace('+', "-").replace('/', "_")
    }
}

pub fn input_password_and_encrypt(username: &str) -> EncryptedPw {
    let password: String = prompt_password_stderr("Password: ").unwrap();
    encrypt_login(username.to_string(), password)
}

// RSA operations

const GOOGLE_DEFAULT_PUBLIC_KEY: &str = "AAAAgMom/1a/v0lblO2Ubrt60J2gcuXSljGFQXgcyZWveWLEwo6prwgi3iJIZdodyhKZQrNWp5nKJ3srRXcUW+F1BD3baEVGcmEgqaLZUNBjm057pKRI16kB0YppeGx5qIQ5QjKzsR8ETQbKLNWgRY0QRNVz34kMJR3P/LgHax/6rmf5AAAAAwEAAQ==";

/// Decompose Google's public key into modulus and exponent components into RSA Public key
fn decompose(public_key: &[u8]) -> rsa::Rsa<pkey::Public> {
    let u32_len = 4;
    let mod_len = util::read_u32_be(&public_key) as usize;
    let mod_offset = u32_len;
    let mod_bin = &public_key[mod_offset..mod_offset + mod_len];
    let exp_offset = mod_offset + mod_len + u32_len;
    let exp_len = util::read_u32_be(&public_key[mod_len + mod_offset..]) as usize;
    let exp_bin = &public_key[exp_offset..exp_offset + exp_len];

    let n_mod = bn::BigNum::from_slice(mod_bin).unwrap();
    let e_exp = bn::BigNum::from_slice(exp_bin).unwrap();

    rsa::Rsa::<pkey::Public>::from_public_components(n_mod, e_exp).unwrap()
}

fn encrypt_login(username: String, password: String) -> EncryptedPw {
    let public_key: Vec<u8> = base64::decode(GOOGLE_DEFAULT_PUBLIC_KEY).unwrap();

    let signature: Vec<u8> = {
        let mut context = sha::Sha1::new();
        context.update(&public_key);
        let digest = context.finish();
        let digest_ref = digest.as_ref();

        vec![
            0u8,
            digest_ref[0],
            digest_ref[1],
            digest_ref[2],
            digest_ref[3],
        ]
    };

    let rsa = decompose(&public_key);

    // ${username}\x00${password}
    let username_password: Vec<u8> = {
        let mut res = vec![];
        res.append(&mut username.into_bytes());
        res.push(0u8);
        res.append(&mut password.into_bytes());
        res
    };

    let encrypted_data = {
        // to store the encrypted contents
        let mut encrypted_buf = vec![0; rsa.size() as usize];

        // encrypt into the space after the signature
        let encrypted_len = rsa
            .public_encrypt(
                &username_password,
                &mut encrypted_buf,
                rsa::Padding::PKCS1_OAEP,
            )
            .expect("public encrypt into buf works");

        &mut encrypted_buf[..encrypted_len].into()
    };

    let mut res = signature;
    res.append(encrypted_data);

    EncryptedPw(res.into())
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
