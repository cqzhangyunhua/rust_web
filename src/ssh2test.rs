use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;

// Connect to the local SSH server

fn main() {

    let tcp = TcpStream::connect("114.55.129.207:68").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
  //  sess.userauth_agent("username").unwrap();
    sess.userauth_password("tigerdev", "CcKFOq05cq9ryCcf_n$*)").unwrap();
    let mut channel = sess.channel_session().unwrap();
    channel.exec("curl https://m.51kcwc.com").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());
}