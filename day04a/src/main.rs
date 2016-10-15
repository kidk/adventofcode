extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;


fn main() {
    let mut x = 0;

    let input = "ckczppom";
    let key = input.as_bytes();

    let mut hasher = Md5::new();

    while true {
        hasher.input(key);
        hasher.input(x.to_string().as_bytes());

        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);

        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            println!("{}", x);
            break;
        }
        hasher.reset();

        x += 1;
    }
}
