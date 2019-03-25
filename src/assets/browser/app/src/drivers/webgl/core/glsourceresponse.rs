use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::{ Hash, Hasher };
use std::rc::Rc;

use super::{ DrawnResponse, PrintEditionAll, WebGLPrinter };
use drawing::CarriageCanvases;
use shape::ShapeSpec;
use composit::{ Leaf, SourceResponseData, StateValue };
use model::driver::SourceResponse;

#[derive(Clone)]
pub struct GLSourceResponse {
    printer: WebGLPrinter,
    idx: usize,
    dr: Rc<RefCell<Option<DrawnResponse>>>,
    state: Rc<RefCell<StateValue>>,
    leaf: Leaf
}

impl PartialEq for GLSourceResponse {
    fn eq(&self, other: &GLSourceResponse) -> bool {
        self.idx == other.idx
    }
}
impl Eq for GLSourceResponse {}

impl Hash for GLSourceResponse {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}

impl GLSourceResponse {    
    /* train/partyresponses */
    pub(in super) fn new(printer: &WebGLPrinter, idx: usize, leaf: &Leaf) -> GLSourceResponse {
        GLSourceResponse {
            printer: printer.clone(),
            idx,
            dr: Rc::new(RefCell::new(None)),
            state: Rc::new(RefCell::new(StateValue::OffCold())),
            leaf: leaf.clone()
        }
    }
        
    /* train/partyresponses */
    pub fn set(&mut self, result: SourceResponseData) {
        *self.dr.borrow_mut() = Some(DrawnResponse::new(result));
    }
    
    /* train/traveller */
    pub fn take(&mut self) -> Option<DrawnResponse> {
        self.dr.borrow_mut().take()
    }
        
    pub fn get_leaf(&self) -> &Leaf { &self.leaf }
    pub fn get_state(&self) -> StateValue {
        self.state.borrow().clone()
    }
    
    pub fn redraw_drawings(&self, cc: &mut CarriageCanvases) {
        //console!("drawings {:?}",self.leaf);
        let mut dr = self.dr.borrow_mut();
        if dr.is_some() {
            dr.as_mut().unwrap().redraw(cc);
        }
    }
    
    pub fn redraw_objects(&self, e: &mut PrintEditionAll) {
        console!("objects {:?}",self.leaf);
        if self.get_state().on() {
            let mut dr = self.dr.borrow_mut();
            if dr.is_some() {
                dr.as_mut().unwrap().into_objects(e);
            }
        }
    }
}

impl SourceResponse for GLSourceResponse {
    /* train/traveller */
    fn check(&self) -> bool {
        self.dr.borrow().is_some()
    }

    fn set_state(&mut self, state: StateValue) {
        *self.state.borrow_mut() = state;
    }
    
    fn destroy(&mut self) {
        let mut p = self.printer.clone();
        p.destroy_partial(self);
    }
}

impl fmt::Debug for GLSourceResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:?}[{}]",self.leaf,self.idx)
    }
}
