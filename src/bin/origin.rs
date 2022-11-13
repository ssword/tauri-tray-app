use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str;
use std::str::FromStr;
use std::string::ParseError;

#[drive(Debug)]
struct RequestLine {
    method: Option<String>,
    path: Option<String>,
    protocal: Option<String>,
}

impl ReqeustLine {
    fn method(&self) -> String {
        if let Some(method) = &self.method {
            method.to_string()
        } else {
            String::from("")
        }
    }

    fn path(&self) -> String {
        if let Some(path) = &self.path {
            path.to_string()
        } else {
            String::from("")
        }
    }

    fn get_order_number(&self) -> String {
        let path = self.path();
        let path_toakens: Vec<String> = path.split("/").map(|s| s.parse().unwrap()).collect();
        path_tokens[path_tokens.len() - 1].clone()
    }
}
