extern crate nix;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use nix::sys::socket::{AddressFamily, InetAddr, SockType, SockFlag, SockAddr, MsgFlags, socket, sendto};
use nix::unistd::close;
use nix::c_int;

const MSG_FASTOPEN: c_int = 0x20000000;

fn main() {
    //let protocol = 0;
    let protocol = nix::sys::socket::SockLevel::Ip as i32;
    let sock = socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), protocol).expect("socket() error");
    let stdsockaddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9999);
    let sockaddr = SockAddr::new_inet(InetAddr::from_std(&stdsockaddr));
    let msgflags = MsgFlags::from_bits(MSG_FASTOPEN).expect("invalid msg flags");

    sendto(sock, b"1. First connection.", &sockaddr, msgflags).expect("sendto(1) error");
    close(sock).unwrap();

    let sock2 = socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), protocol).expect("socket() error");
    sendto(sock2, b"2. Sending data by using TCP fast open!!!", &sockaddr, msgflags).expect("sendto(2) error");

    close(sock2).unwrap();
}
