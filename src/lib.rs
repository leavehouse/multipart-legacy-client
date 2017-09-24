extern crate hyper;
extern crate multipart;

use hyper::client::Request;
use hyper::method::Method;
pub use multipart::client::Multipart;
use std::io::{self, Read};

#[derive(Debug)]
pub enum RequestError {
    ParseError(hyper::error::ParseError),
    HyperError(hyper::Error),
    IoError(io::Error),
}

impl From<hyper::error::ParseError> for RequestError {
    fn from(e: hyper::error::ParseError) -> RequestError {
        RequestError::ParseError(e)
    }
}

impl From<hyper::Error> for RequestError {
    fn from(e: hyper::Error) -> RequestError {
        RequestError::HyperError(e)
    }
}

impl From<io::Error> for RequestError {
    fn from(e: io::Error) -> RequestError {
        RequestError::IoError(e)
    }
}

pub fn send_new_post_request<T>(url: T, file: &std::path::Path)
                                -> Result<Vec<u8>, RequestError>
                                where T: AsRef<str> {
    let parsed_url = url.as_ref().parse()?;
    let request = Request::new(Method::Post, parsed_url)?;
    let mut mp_request = Multipart::from_request(request)?;
    mp_request.write_file("what_does_this_do", file)?;
    let mut response = mp_request.send()?;

    if !response.status.is_success() {
        let mut res = String::new();
        response.read_to_string(&mut res).expect("failed to read response");
        println!("response reported unsuccessful: {:?}\n {}", response, res);
    }

    let mut buf = vec![];
    response.read_to_end(&mut buf)?;
    Ok(buf)
}
