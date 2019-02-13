use std::collections::HashSet;
use std::sync::{ Arc, Mutex };

use core::BinaryCode;
use util::ValueStore;
use super::environment::Environment;
use super::interproc::InterpProcess;
use super::procconf::{ ProcessConfig, PROCESS_CONFIG_DEFAULT };

#[derive(Debug,PartialEq,Clone)]
pub enum ProcessState { Killed(String), Halted, Gone, Running, Sleeping }

impl ProcessState {
    pub fn alive(&self) -> bool {
        *self == ProcessState::Running || *self == ProcessState::Sleeping
    }
}

#[derive(Debug,Clone)]
pub struct ProcessStatus {
    pub state: ProcessState,
    pub cycles: i64
}

const STATUS_GONE : ProcessStatus = ProcessStatus {
    state: ProcessState::Gone,
    cycles: 0
};

#[derive(Clone)]
pub struct Signals {
    awoke: Arc<Mutex<Vec<usize>>>
}

impl Signals {
    pub fn new() -> Signals {
        Signals {
            awoke: Arc::new(Mutex::new(Vec::<usize>::new()))
        }
    }
    
    pub fn awoke(&self, pid: usize) {
        self.awoke.lock().unwrap().push(pid);
    }
    
    pub fn drain_awoken(&self) -> Vec<usize> {
        let out = &mut self.awoke.lock().unwrap();
        let out : Vec<usize> = out.drain(..).collect();
        out.clone()
    }
}

#[derive(PartialEq)]
enum RunResult { Timeout, Empty, Finished }

#[derive(Clone)]
pub struct InterpConfig {
    pub cycles_per_run: i64,
}

pub const DEFAULT_CONFIG : InterpConfig = InterpConfig {
    cycles_per_run: 100,
};

pub struct Interp {
    env: Box<Environment>,
    config: InterpConfig,
    procs: ValueStore<InterpProcess>,
    runq: HashSet<usize>,
    nextq: HashSet<usize>,
    signals: Signals
}

impl Interp {
    pub fn new(env: Box<Environment>, config: InterpConfig) -> Interp {
        Interp {
            env, config,
            procs: ValueStore::<InterpProcess>::new(),
            runq: HashSet::<usize>::new(),
            nextq: HashSet::<usize>::new(),
            signals: Signals::new()
        }
    }
    
    pub fn exec(&mut self, bc: &BinaryCode, start: Option<&str>, pc: Option<&ProcessConfig>) -> Result<usize,String> {
        let pc = pc.unwrap_or(&PROCESS_CONFIG_DEFAULT);
        match bc.exec(start,Some(self.signals.clone()),pc) {
            Ok(p) => {
                let pid = self.procs.store(InterpProcess::new(p,pc,&mut self.env));
                self.procs.get_mut(pid).unwrap().set_pid(&mut self.env,pid);
                self.runq.insert(pid);
                Ok(pid)
            },
            Err(e) => Err(e)
        }
    }
    
    fn add_awoken(&mut self) {
        for pid in self.signals.drain_awoken() {
            self.runq.insert(pid);
        }
    }
    
    fn drain_runq(&mut self, end: i64) -> RunResult {
        if self.runq.is_empty() { return RunResult::Empty; }
        let runnable : Vec<usize> = self.runq.drain().collect();
        for pid in runnable {
            let status = {
                let mut ip = self.procs.get_mut(pid).unwrap();
                println!("one");
                ip.run_proc(&mut self.env,self.config.cycles_per_run);
                ip.status()
            };
            if status.state == ProcessState::Running {
                self.nextq.insert(pid);
            }
            if !status.state.alive() {
                self.procs.unstore(pid);
            }
            if self.env.get_time() >= end {
                return RunResult::Timeout;
            }
        }
        RunResult::Finished
    }
    
    pub fn run(&mut self, delta: i64) -> bool {
        let end = self.env.get_time() + delta;
        loop {
            self.add_awoken();
            let r = self.drain_runq(end);
            self.runq = self.runq.union(&self.nextq).cloned().collect();
            self.nextq.clear();
            if r == RunResult::Finished { continue; }
            return r == RunResult::Timeout;
        }
    }
    
    pub fn status(&mut self, pid: usize) -> ProcessStatus {
        if let Some(ref mut ip) = self.procs.get_mut(pid) {
            ip.status()
        } else {
            STATUS_GONE
        }
    }
}

#[cfg(test)]
mod test {
    use std::{ thread, time };
    use super::{ Interp, ProcessState, DEFAULT_CONFIG };
    use test::{ command_compile, DebugEnvironment, TestContext };
        
    #[test]
    fn noprocs() {
        let t_env = DebugEnvironment::new();
        let mut t = Interp::new(t_env.make(),DEFAULT_CONFIG);
        assert!(!t.run(1000));
    }
    
    #[test]
    fn multi_proc() {
        let t_env = DebugEnvironment::new();
        let mut t = Interp::new(t_env.make(),DEFAULT_CONFIG);
        let tc = TestContext::new();
        let bin1 = command_compile("multi-1",&tc);
        let bin2 = command_compile("multi-2",&tc);
        t.exec(&bin1,None,None).ok().unwrap();
        t.exec(&bin2,None,None).ok().unwrap();
        for _ in 0..40 {
            while t.run(1000) {}
            thread::sleep(time::Duration::from_millis(50));
            while t.run(1000) {}        
        }
        assert_eq!(vec!{vec![100.],vec![200.]},t_env.get_exit_float());
        assert_eq!(vec!["multi-1","multi-2"],t_env.get_exit_str());
    }
    
    #[test]
    fn smoke() {
        let t_env = DebugEnvironment::new();
        let mut t = Interp::new(t_env.make(),DEFAULT_CONFIG);
        let tc = TestContext::new();
        let bin = command_compile("interp-smoke",&tc);
        t.exec(&bin,None,None).ok().unwrap();
        while t.run(1000) {}
        assert_eq!("Success!",t_env.get_exit_str()[0]);
        assert_eq!([0.,200.].to_vec(),t_env.get_exit_float()[0]);
    }
    
    #[test]
    fn sleep_wake() {
        let t_env = DebugEnvironment::new();
        let mut t = Interp::new(t_env.make(),DEFAULT_CONFIG);
        let tc = TestContext::new();
        let bin = command_compile("interp-sleep-wake",&tc);
        t.exec(&bin,None,None).ok().unwrap();
        while t.run(1000) {}
        thread::sleep(time::Duration::from_millis(500));
        while t.run(1000) {}
        assert_eq!("awoke",t_env.get_exit_str()[0]);
    }
    
    #[test]
    fn status() {
        let t_env = DebugEnvironment::new();
        let mut t = Interp::new(t_env.make(),DEFAULT_CONFIG);
        let tc = TestContext::new();
        let bin = command_compile("interp-status",&tc);
        let pid = t.exec(&bin,None,None).ok().unwrap();
        assert_eq!(ProcessState::Running,t.status(pid).state);
        while t.run(1000) {}
        assert_eq!(ProcessState::Sleeping,t.status(pid).state);
        thread::sleep(time::Duration::from_millis(500));
        while t.run(1000) {}
        assert_eq!(t_env.get_exit_state().unwrap(),ProcessState::Halted);
        assert_eq!(pid,t_env.get_pid().unwrap());
    }
    
    #[test]
    fn cycle_count() {
        let t_env = DebugEnvironment::new();
        let mut t = Interp::new(t_env.make(),DEFAULT_CONFIG);
        let tc = TestContext::new();
        let bin = command_compile("cycle-count",&tc);
        let pid = t.exec(&bin,None,None).ok().unwrap();
        t.run(0);
        assert_eq!(192,t.status(pid).cycles);
        assert_eq!(ProcessState::Running,t.status(pid).state);
        t.run(0);
        assert_eq!(384,t.status(pid).cycles);
        assert_eq!(ProcessState::Running,t.status(pid).state);
        t.run(1000);
        assert_eq!(t_env.get_exit_state().unwrap(),ProcessState::Halted);
    }
}
