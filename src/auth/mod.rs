
use constants::*;

use hyper;
use hyper::Client;
use hyper::header::Headers;
use hyper::header::Authorization;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use std::io::Read;
use std::collections::BTreeMap;

use util::make_nonce;
use util::hmac_sha1;
use util::timestamp;
use util::percent_encode;
use util::header_combine;

pub struct PinAuth {
    client: Client,
}

impl PinAuth {
    pub fn new() -> PinAuth {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        PinAuth { client: client }
    }

    pub fn authenticate(&self) {
        let mut res = self.client.get(&*AUTHORIZE_URL).send().unwrap();
        let mut buffer: String = String::new();
        res.read_to_string(&mut buffer);

        println!("{}", buffer);
    }

    pub fn get_request_token(&self) {
        let key_insecure = CONSUMER_KEY.unsecure();
        let key: String = unsafe { String::from_utf8_unchecked(key_insecure.to_vec()) };

        let mut params: BTreeMap<&'static str, String> = BTreeMap::new();
        params.insert("oauth_callback", String::from("oob"));
        params.insert("oauth_consumer_key", key);
        params.insert("oauth_nonce", make_nonce());
        params.insert("oauth_signature_method", String::from("HMAC-SHA1"));
        params.insert("oauth_timestamp", timestamp());
        params.insert("oauth_version", String::from("1.0"));

        let oauth_str: String = header_combine(&params, |key, value| {
            return format!("{}={}&", key, percent_encode(value));
        });

        let final_oauth_str = oauth_str.trim_left_matches("&");

        let to_hash: String = format!("POST&{}&{}",
                                      percent_encode(&*REQ_TOKEN_URL),
                                      percent_encode(final_oauth_str));
        println!("to hash: {}", to_hash);

        let hashed: String = hmac_sha1(to_hash.clone(), String::new());

        params.insert("oauth_signature", hashed);
        let header: String = header_combine(&params, |key, value| {
            return format!("{}=\"{}\", ", key, percent_encode(value));
        });

        let final_header: String = format!("OAuth {}", header.trim_right_matches(", "));

        println!("auth header: {}", final_header);

        let mut headers = Headers::new();

        headers.set(Authorization(final_header.to_owned()));


        let mut res = self.client
            .post(&*REQ_TOKEN_URL)
            .headers(headers)
            .send()
            .unwrap();

        let mut buffer: String = String::new();
        res.read_to_string(&mut buffer);

        println!("{}", buffer);
    }
}
