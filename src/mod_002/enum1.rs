
#[derive(Debug)]
enum IPKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IPAddress {
    kind: IPKind,
    address: String,
}

#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IPAddr2 {
    V4(u8, u8, u8, u8), // TODO: Not possible with struct enums
    V6(String),
}

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {

    let home = IPAddress {
        kind: dbg!(IPKind::V4),
        address: String::from("127.0.0.1")
    };

    let loopback = IPAddress {
        kind: dbg!(IPKind::V6),
        address: String::from("::1")
    };

    dbg!(home);
    dbg!(loopback);


    let _home = IPAddr::V4(String::from("127.0.0.1"));
    let _loopback = IPAddr::V6(String::from("::1"));



}