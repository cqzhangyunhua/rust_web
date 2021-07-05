use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;

// Connect to the local SSH server

fn main() {

    let tcp = TcpStream::connect("45646").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
  //  sess.userauth_agent("username").unwrap();
    sess.userauth_password("4564", "123456)").unwrap();
    let mut channel = sess.channel_session().unwrap();
    channel.exec("curl https://m.51kcwc.com").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());
}
