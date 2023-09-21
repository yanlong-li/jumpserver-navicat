use crypto::blowfish::Blowfish;
use crypto::symmetriccipher::BlockEncryptor;
use hex::ToHex;

pub fn encrypt(str: &str) -> String {
    let mut iv = b"\xd9\xc7\xc3\xc8\x87\x0d\x64\xbd".to_vec();

    let mut result: Vec<u8> = vec![];
    let str_vec = Vec::from(str);

    let round = str_vec.len() / 8;

    for i in 0..round {
        let temp = xor_bytes(&str_vec[i * 8..i * 8 + 8].to_vec(), &iv);
        let temp = encrypt_block(&temp);
        iv = xor_bytes(&iv, &temp);
        result.extend(temp.iter());
    }
    if str_vec.len() % 8 != 0 {
        iv = encrypt_block(&iv);
        let temp = xor_bytes(&str_vec[round * 8..round * 8 + str_vec.len() % 8].to_vec(), &iv);
        result.extend(temp.iter());
    }


    return result.encode_hex_upper();
}

fn xor_bytes(str1: &Vec<u8>, str2: &Vec<u8>) -> Vec<u8> {
    let mut str = vec![0u8; str1.len()];
    for i in 0..str1.len() {
        str[i] = str1[i] ^ str2[i];
    }

    return str;
}

fn encrypt_block(block: &Vec<u8>) -> Vec<u8> {
    let key = b"\x42\xce\xb2\x71\xa5\xe4\x58\xb7\x4a\xea\x93\x94\x79\x22\x35\x43\x91\x87\x33\x40";

    let bf = Blowfish::new(key);

    let mut output = vec![0u8; block.len()];

    bf.encrypt_block(block, &mut output);

    return output.to_vec();
}

#[cfg(test)]
mod test {
    use crate::encrypt::encrypt;

    #[test]
    fn test() {
        assert_eq!(encrypt("123456"), "15057D7BA390");
        assert_eq!(encrypt("1111111111"), "8663D171E18F1B37D7F9");
    }
}