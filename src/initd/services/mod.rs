pub mod openvpn;


pub trait ServiceConfig {
    fn name(&self) -> &'static str;
    fn executable(&self) -> String;
}
