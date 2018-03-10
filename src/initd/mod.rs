pub mod process;
pub mod services;


pub struct InitD {
}

impl Default for InitD {
    fn default() -> Self {
        Self {}
    }
}

//impl<T: Processable> InitD {
//    pub fn start_process(&self, process_config: T) {
//        Process::new(process_config);
//    }
//}
