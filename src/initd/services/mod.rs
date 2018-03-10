pub mod openvpn;


pub trait ServiceConfig {
    fn executable(&self) -> String;
}
