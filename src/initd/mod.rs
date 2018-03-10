pub mod process;
pub mod services;

use self::services::ServiceConfig;
use self::process::Process;

pub struct InitD {
}

impl Default for InitD {
    fn default() -> Self {
        Self {}
    }
}

impl InitD {
    pub fn start_process<T: ServiceConfig>(&self, config: T) {
        let p = Process::new();
        p.start(config);
    }
}
