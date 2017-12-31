extern crate ssh2;

use std::net::TcpStream;
use std::io::Read;

use ssh2::Session;
use ssh2::Channel;

use super::confs::SshContext;

pub mod remotecommand;

/**
* Execute arbitrary shell command through ssh
*/
pub fn execute_ssh_command(command: String,ssh_context: &SshContext) -> String {
    let tcp = TcpStream::connect(&ssh_context.host).unwrap();
    let mut session = Session::new().unwrap();

    session.handshake(&tcp).unwrap();
    session.userauth_password(&ssh_context.user, &ssh_context.pass).unwrap();

    let mut channel = session.channel_session().unwrap();

    channel.exec(&command).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();

    s
}