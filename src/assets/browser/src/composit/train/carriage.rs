use std::fmt;
use std::rc::Rc;

use print::{ Programs, PrintEdition };
use composit::{ Source, SourceResponse, Leaf };
use composit::state::{ StateManager, StateExpr, StateValue, ComponentRedo };
use drawing::DrawingSession;

pub struct Carriage {
    prev_value: StateValue,
    cur_value: StateValue,
    source: Rc<Source>,
    ooe: Rc<StateExpr>,
    response: SourceResponse,
    comp_name: String,
    leaf: Leaf
}

impl Carriage {
    pub fn new(ooe: &Rc<StateExpr>, comp_name: &str, leaf: &Leaf,
               source: &Rc<Source>) -> Carriage {
        Carriage {
            response: SourceResponse::new(),
            prev_value: StateValue::OffCold(),
            cur_value: StateValue::OffCold(),
            ooe: ooe.clone(),
            comp_name: comp_name.to_string(),
            leaf: leaf.clone(),
            source: source.clone()
        }
    }
    
    pub fn is_on(&self) -> bool { self.cur_value.on() }
    
    pub fn update_state(&mut self, m: &StateManager) -> ComponentRedo {
        self.prev_value = self.cur_value;
        self.cur_value = self.ooe.is_on(m);
        if self.prev_value == self.cur_value {
            ComponentRedo::None // no change => Noop
        } else if self.prev_value.on() && self.cur_value.on() {
            ComponentRedo::None // was on, is on => Noop
        } else if self.prev_value.offcold() || self.cur_value.offcold() {
            ComponentRedo::Major // was/now off-cold => Major
        } else {
            ComponentRedo::Minor // was/is off-warm, is/was on => Minor
        }
    }
         
    pub fn is_done(&self) -> bool { self.response.is_done() }
    pub fn get_max_y(&self) -> i32 { self.response.get_max_y() }
        
    pub fn draw_drawings(&mut self, ds: &mut DrawingSession){
        self.response.each_shape(|s| s.redraw(ds));
    }

    pub fn into_objects(&mut self, 
                        progs: &mut Programs,
                        ds: &mut DrawingSession, e: &mut PrintEdition) {
        self.response.each_shape(|s| s.into_objects(progs,ds,e));
    }
    
    pub fn get_component_name(&self) -> &str { &self.comp_name }
    
    pub fn populate(&self) {
        self.source.populate(&mut self.response.clone(),&self.leaf);
    }
}

impl fmt::Debug for Carriage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}:{:?}",self.comp_name,self.leaf)
    }
}
