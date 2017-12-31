use super::SshContext;
use super::execute_ssh_command;

//Reboot server through ssh
pub fn reboot_server(ssh_ctx : &SshContext) -> String{
    let reboot_command = String::from("sudo reboot");
    execute_ssh_command(reboot_command,ssh_ctx)
}