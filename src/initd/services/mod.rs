pub mod openvpn;


pub trait ServiceConfig {
    fn start_command(&self) -> String;
}
