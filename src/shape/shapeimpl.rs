use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use arena::ArenaData;
use program::{ ProgramAttribs, DataGroup };
use coord::{ Colour, RPixel };
use campaign::Campaign;

pub trait Shape {
    fn into_objects(&self, geom_name:&str, geom: &mut ProgramAttribs, adata: &ArenaData);
    fn set_texpos(&mut self, _data: &RPixel) {}
    fn get_geometry(&self) -> &str;
}

pub trait ShapeContext {
    fn reset(&mut self);
    fn into_objects(&mut self, geom_name: &str, geom: &mut ProgramAttribs, adata: &ArenaData);
}

const SPOTS : [&str;4] = [
    "stretchspot","pinspot","pinstripspot","stretchstrip"];

pub struct SpotImpl {
    colour: Colour,
    group: HashMap<String,DataGroup>
}

#[derive(Clone)]
pub struct Spot(Rc<RefCell<SpotImpl>>);

impl SpotImpl {
    pub fn new(colour: &Colour) -> SpotImpl {
        SpotImpl {
            group: HashMap::<String,DataGroup>::new(),
            colour: *colour
        }
    }

    pub fn get_group(&self, name: &str) -> DataGroup {
        self.group[name]
    }
}

impl ShapeContext for SpotImpl {
    fn reset(&mut self) {
        self.group.clear();
    }

    fn into_objects(&mut self, geom_name: &str, geom: &mut ProgramAttribs, _adata: &ArenaData) {
        if SPOTS.contains(&geom_name) {
            let group = geom.new_group();
            self.group.insert(geom_name.to_string(),group);
            if let Some(obj) = geom.get_object("uColour") {
                obj.set_uniform(Some(group),self.colour.to_uniform());
            }
        }
    }    
}

impl Spot {
    pub fn new(camps: &mut Campaign, colour: &Colour) -> Spot {
        let s = Spot(Rc::new(RefCell::new(SpotImpl::new(colour))));
        camps.add_context(Box::new(s.clone()));
        s
    }

    pub fn get_group(&self, name: &str) -> DataGroup {
        self.0.borrow().get_group(name)
    }
}

impl ShapeContext for Spot {
    fn reset(&mut self) {
        self.0.borrow_mut().reset();
    }

    fn into_objects(&mut self, geom_name: &str, geom: &mut ProgramAttribs, adata: &ArenaData) {
        self.0.borrow_mut().into_objects(geom_name,geom,adata);
    }
}

#[derive(Clone)]
pub enum ColourSpec {
    Colour(Colour),
    Spot(Spot)
}

impl ColourSpec {
    pub fn to_group(&self, gn: &str) -> Option<DataGroup> {
        match self {
            ColourSpec::Spot(s) => Some(s.get_group(gn)),
            ColourSpec::Colour(_) => None
        }
    }
}
