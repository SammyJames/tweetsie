
use std::string::String;
use rand::{OsRng,Rng};
use rustc_serialize::base64::{self,ToBase64};
use crypto::hmac::Hmac;
use crypto::sha1::Sha1;
use crypto::mac::Mac;
use time;
use regex::Regex;
use url::Url;
use url::percent_encoding::{EncodeSet, PercentEncode, utf8_percent_encode};
use std::collections::BTreeMap;

use constants::CONSUMER_SEC;
use constants::CONSUMER_KEY;


static NONCE_SIZE: usize = 32;

lazy_static! {
	static ref CONFIG: base64::Config = base64::Config { 
		char_set: base64::CharacterSet::UrlSafe, 
		newline: base64::Newline::LF, 
		pad: false, 
		line_length: None 
	};

	static ref REPLACE_WITH_AND: Regex = Regex::new(r"(,\s)").unwrap();
	static ref REPLACE_ALL_QUOTES: Regex = Regex::new(r"(\x22)").unwrap();
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct OAUTH_ENCODE_SET;

impl EncodeSet for OAUTH_ENCODE_SET {
    fn contains(&self, byte: u8) -> bool {
        !((byte >= 0x30 && byte <= 0x39)
        || (byte >= 0x41 && byte <= 0x5A)
        || (byte >= 0x61 && byte <= 0x7A)
        || byte == 0x2D || byte == 0x2E
        || byte == 0x5F || byte == 0x7E)
    }
}


pub fn percent_encode(input: &str) -> PercentEncode<OAUTH_ENCODE_SET> {
	utf8_percent_encode(input, OAUTH_ENCODE_SET)
}


pub fn header_combine<F>(input: &BTreeMap<&'static str, String>, combiner: F) -> String 
	where F: Fn(&'static str, &String) -> String {
	let mut result = String::new();
	for (key, value) in input.iter() {
		let value = combiner(key, value);
    	result.push_str(&value);   
    }

    result
}

pub fn make_nonce() -> String {
	let mut raw: Vec<u8> = Vec::with_capacity(NONCE_SIZE);
	unsafe {
		raw.set_len(NONCE_SIZE);
	}

	match OsRng::new() {
		Ok(mut gen) => gen.fill_bytes(raw.as_mut_slice()),
		Err(_) => ::rand::thread_rng().fill_bytes(raw.as_mut_slice())
	};

	raw.to_base64( *CONFIG )
}

pub fn hmac_sha1(to_hash: String, tok: String) -> String {
	let secret_insecure = CONSUMER_SEC.unsecure();
	let secret: String = unsafe { 
		String::from_utf8_unchecked(secret_insecure.to_vec())
	};

	let to_combine: (String, String) = (percent_encode(&secret).collect(), percent_encode(&tok).collect());
	let key: String = format!("{}&{}", to_combine.0, to_combine.1);
	let mut hmac = Hmac::new(Sha1::new(), key.as_bytes());

	hmac.input(to_hash.as_bytes());

	let mut mac = [0u8; 20];
	hmac.raw_result(&mut mac);

	mac.to_base64(*CONFIG)
}

fn gen_timestamp() -> u64 {
    let x = time::now_utc().to_timespec().sec;
    assert!(x > 0);
    return x as u64;
}

pub fn timestamp() -> String {
	gen_timestamp().to_string()
}
