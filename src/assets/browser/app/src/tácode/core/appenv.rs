use stdweb::unstable::TryInto;

use tánaiste::{ Environment, ProcessState };

use tácode::TáContext;

pub struct AppEnv {
    tc: TáContext
}

impl AppEnv {
    pub fn new(tc: TáContext) -> AppEnv {
        AppEnv { tc }
    }
}

impl Environment for AppEnv {
    fn get_time(&mut self) -> i64 {
        let t : f64 = js! { return +new Date(); }.try_into().unwrap();
        t as i64
    }
    
    fn started(&mut self, pid: usize) {}
    
    fn finished(&mut self, pid: usize, state: ProcessState, codes: Vec<f64>, string: String) {
        self.tc.finished(pid);
    }
}
