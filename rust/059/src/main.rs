use std::char;
use std::fs;

fn main() {
    let ciphertext = fs::read_to_string("cipher.txt").unwrap();
    let ciphertext = ciphertext.split(',').map(|code| code.parse::<u32>().unwrap()).collect::<Vec<_>>();

    const ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";

    'a_loop: for a in ASCII_LOWERCASE.chars() {
        for b in ASCII_LOWERCASE.chars() {
            for c in ASCII_LOWERCASE.chars() {
                let key = &format!("{}{}{}", a, b, c).repeat((ciphertext.len() + 1) / 3)[..ciphertext.len()];

                let plaintext = key.bytes()
                                   .enumerate()
                                   .map(|(i, k_i)| char::from_u32(ciphertext[i] ^ k_i as u32).unwrap())
                                   .collect::<String>();

                if plaintext.matches(" the ").collect::<Vec<_>>().len() > 0 {
                    println!("{}", plaintext.bytes().map(|b| b as usize).sum::<usize>());
                    break 'a_loop;
                }
            }
        }
    }
}
