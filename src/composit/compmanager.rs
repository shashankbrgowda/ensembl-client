use std::collections::HashMap;
use std::collections::hash_map::Entry;

use composit::{
    Component, Leaf, LeafComponent
};

pub struct ComponentManager {
    components: HashMap<String,Component>    
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            components: HashMap::<String,Component>::new()
        }
    }
        
    pub fn add(&mut self, c: Component) {
        let name = c.get_name().to_string();
        if let Entry::Vacant(e) = self.components.entry(name) {
            e.insert(c);
        }
    }

    pub fn remove(&mut self, k: &str) {
        self.components.remove(k);
    }
    
    pub fn make_leafcomps(&self, leaf: Leaf) -> Vec<LeafComponent> {
        let mut lcomps = Vec::<LeafComponent>::new();        
        for (k,c) in &self.components {
            lcomps.push(c.make_leafcomp(&leaf));
        }
        lcomps
    }    
}
