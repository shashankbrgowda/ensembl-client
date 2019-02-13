use std::rc::Rc;

use composit::{ 
    StateValue, StateFixed, SourceManager, StateAtom, ActiveSource
};
use controller::global::App;
use controller::input::{ Action, actions_run };
use debug::testcards::text::text_source;
use debug::testcards::leafcard::leafcard_source;
use debug::testcards::debugsource::{ DebugSource, DebugStickManager };
use debug::testcards::{ bs_source_main, bs_source_sub, polar_source, tá_source };
use tácode::Tácode;

fn debug_source_main(tc: &Tácode) -> DebugSource {
    let mut s = DebugSource::new();
    s.add_stick("polar",Box::new(polar_source(None)));
    s.add_stick("text",Box::new(text_source()));
    s.add_stick("leaf",Box::new(leafcard_source(true)));
    s.add_stick("ruler",Box::new(leafcard_source(false)));
    s.add_stick("button",Box::new(bs_source_main()));
    s.add_stick("tácode",Box::new(tá_source(tc)));
    s
}

fn debug_source_sub(even: bool) -> DebugSource {
    let mut s = DebugSource::new();
    s.add_stick("button",Box::new(bs_source_sub(even)));
    s.add_stick("polar",Box::new(polar_source(Some(even))));
    s
}

pub fn debug_stick_manager() -> DebugStickManager {
    let mut s = DebugStickManager::new();
    s.add_stick("polar",17000000,false);
    s.add_stick("text", 17000000,false);
    s.add_stick("leaf", 17000000,false);
    s.add_stick("ruler",17000000,false);
    s.add_stick("button",17000000,false);
    s.add_stick("tácode",17000000,false);
    s
}

pub fn testcard_base(a: &mut App, stick_name: &str) {
    actions_run(a,&vec! {
        Action::AddComponent("internal:debug-main".to_string()),
        Action::AddComponent("internal:debug-even".to_string()),
        Action::AddComponent("internal:debug-odd".to_string()),
        Action::SetStick(stick_name.to_string()),
        Action::SetState("even".to_string(),StateValue::On()),
        Action::SetState("odd".to_string(),StateValue::On()),
        Action::ZoomTo(-5.)
    });
}

fn component_debug_main(tc: &Tácode, name: &str) -> ActiveSource {
    let cs = debug_source_main(tc);
    ActiveSource::new(name,Rc::new(cs),Rc::new(StateFixed(StateValue::On())))    
}

fn component_debug_sub(name: &str, even: bool) -> ActiveSource {
    let cs = debug_source_sub(even);
    let state_name = if even { "even" } else { "odd" };
    let state = Rc::new(StateAtom::new(state_name));
    ActiveSource::new(name,Rc::new(cs),state)
}

pub struct DebugSourceManager {
    tc: Tácode
}

impl DebugSourceManager {
    pub fn new(tc: &Tácode) -> DebugSourceManager {
        DebugSourceManager {
            tc: tc.clone()
        }
    }
}

impl SourceManager for DebugSourceManager {
    fn get_component(&mut self, name: &str) -> Option<ActiveSource> {
        match name {
            "internal:debug-main" => Some(component_debug_main(&self.tc,name)),
            "internal:debug-even" => Some(component_debug_sub(name,true)),
            "internal:debug-odd" => Some(component_debug_sub(name,false)),
            _ => None
        }
    }
}
