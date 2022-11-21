use std::{net::TcpListener, io::{BufReader, BufRead, Write}};

use crypto::{sha2::Sha256, digest::Digest};
use rand::{seq::SliceRandom, thread_rng};
use reqwest::Url;

const GOOGLE_CLIENT_ID: &'static str = env!("BMNGGCI");
const GOOGLE_CLIENT_SECRET: &'static str = env!("BMNGGCS");

fn main() {
    let mut rng = thread_rng();
    let mut state: [u8;32] = Default::default();
    {
        const CHARS: &'static [u8] = b"abcdefghijklmnopqrstuvxwyz-0123456789_ABCDEFGHIJKLMNOPQRSTUVXWYZ";
        for (i, v) in CHARS.choose_multiple(&mut rng, state.len()).enumerate() {
            state[i] = *v;
        }
    }
    let state_str = String::from_utf8_lossy(&state);

    let mut hasher = Sha256::new();
    hasher.input(&state);

    let code_challenge = hasher.result_str();
    
    println!("\nstate: {state_str}\ncode_challenge: {code_challenge}");
    
    let login_url = format!("https://accounts.google.com/o/oauth2/v2/auth?\
scope=openid%20email&\
access_type=offline&\
include_granted_scopes=true&\
response_type=code&\
state={state_str}&\
code_challenge={code_challenge}&\
code_challenge_method=S256&\
redirect_uri=http%3A//127.0.0.1%3A7878&\
client_id={GOOGLE_CLIENT_ID}");
    println!("\nLogin url: {}", login_url);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut stream = listener.incoming().next().unwrap().unwrap();
    let mut req = String::new();
    { BufReader::new(&stream).read_line(&mut req).unwrap(); }
    stream.flush().unwrap();
    {
        let msg = "OK";
        write!(stream, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{msg}", msg.len()).unwrap();
    }
    let req_path = req.split_whitespace().nth(1).unwrap();
    let req_params = req_path.split('&');
    println!("\nRequest:");
    for line in req_params {
        println!("\n{:?}", line);
    }
}