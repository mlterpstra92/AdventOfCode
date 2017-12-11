extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
	let mut hasher = Md5::new();
	for i in 0..std::u64::MAX{
		let key = "ckczppom";
		fn concatted(key: &str, num: u64) -> String {
			format!("{}{}", key, num)
		}
		
		hasher.input(concatted(key, i).as_bytes());
		let mut out = [0; 16]; // An MD5 is 16 bytes

		hasher.result(&mut out);
		if out[0] as i32 + out[1] as i32 + out[2] as i32 == 0 {
			println!("{}", i);
			break;
		}
		hasher.reset();
	}
}