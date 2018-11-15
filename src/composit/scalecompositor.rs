use std::cmp::{ max, min };
use std::collections::{ HashMap, HashSet };

use composit::{ Compositor, Leaf, LeafComponent, StateManager, vscale_bp_per_leaf };
use composit::state::ComponentRedo;

const MAX_FLANK : i32 = 10;

pub struct ScaleCompositor {
    vscale: i32,
    train_flank: i32,
    middle_leaf: i64,
    leafcomps: HashMap<Leaf,HashMap<u32,LeafComponent>>,
    done_seen: HashMap<Leaf,u32>,
    done_now: u32
}

impl ScaleCompositor {
    pub fn new(vscale: i32) -> ScaleCompositor {
        ScaleCompositor {
            vscale,
            train_flank: 10,
            middle_leaf: 0,
            leafcomps: HashMap::<Leaf,HashMap<u32,LeafComponent>>::new(),
            done_seen: HashMap::<Leaf,u32>::new(),
            done_now: 0
        }
    }
    
    pub fn set_position(&mut self, position_bp: f64) {
        self.middle_leaf = (position_bp / vscale_bp_per_leaf(self.vscale) as f64) as i64;
        debug!("trains","set position leaf={}",self.middle_leaf);
    }
    
    pub fn set_zoom(&mut self, bp_per_screen: f64) {
        let leaf_per_screen = bp_per_screen / vscale_bp_per_leaf(self.vscale);
        self.train_flank = min(max((3.*leaf_per_screen) as i32,1),MAX_FLANK);
        debug!("trains","set  bp_per_screen={} bp_per_leaf={} leaf_per_screen={}",
            bp_per_screen,vscale_bp_per_leaf(self.vscale),leaf_per_screen);
    }

    pub fn add_lcomps_to_leaf(&mut self, leaf: Leaf, mut lcomps: Vec<LeafComponent>) {
        let mut lcc = self.leafcomps.entry(leaf).or_insert_with(||
            HashMap::<u32,LeafComponent>::new());
        for lc in lcomps.drain(..) {
            lcc.insert(lc.get_component_index(),lc);
        }
        self.done_now += 1;
    }
        
    fn remove_leaf(&mut self, leaf: &Leaf) {
        self.leafcomps.remove(leaf);
    }

    fn get_missing_leafs(&mut self) -> Vec<Leaf> {
        let mut out = Vec::<Leaf>::new();
        for idx in -self.train_flank..self.train_flank+1 {
            let hindex = self.middle_leaf + idx as i64;
            let leaf = Leaf::new(hindex,self.vscale);
            if !self.leafcomps.contains_key(&leaf) {
                debug!("trains","adding {}",hindex);
                out.push(leaf);
            }
        }
        return out;
    }
    
    fn remove_unused_leafs(&mut self) {
        let mut doomed = HashSet::new();
        for leaf in self.leafcomps.keys() {
            if (leaf.get_index()-self.middle_leaf).abs() > self.train_flank as i64 {
                doomed.insert(leaf.clone());
            }
        }
        for d in doomed {
            debug!("trains","removing {}",d.get_index());
            self.remove_leaf(&d);
        }
    }

    pub fn manage_leafs(&mut self) -> Vec<Leaf> {
        self.remove_unused_leafs();
        return self.get_missing_leafs();
    }
    
    pub fn leafs(&self) -> Vec<Leaf> {
        self.leafcomps.keys().map(|s| s.clone()).collect()
    }
    
    fn is_done(&mut self) -> bool {
        for leaf in self.leafcomps.keys() {            
            if let Some(lcc) = self.leafcomps.get(leaf) {
                for lc in lcc.values() {
                    if !lc.is_done() { return false; }
                }
            }
        }
        return true;
    }
    
    pub fn get_components(&mut self, leaf: &Leaf) -> Option<Vec<&mut LeafComponent>> {
        if !self.is_done() { return None; }
        let lcc = self.leafcomps.get_mut(leaf);
        Some(if let Some(lcc) = lcc {
            lcc.values_mut().collect()
        } else {
            vec!{}
        })
    }

    pub fn calc_level(&mut self, leaf: &Leaf, oom: &StateManager) -> ComponentRedo {
        /* Any change due to component changes? */
        let mut redo = ComponentRedo::None;
        if let Some(ref mut lcc) = self.leafcomps.get_mut(leaf) {
            for c in lcc.values_mut() {
                redo = redo | c.update_state(oom);
            }
        }
        /* Any change due to availability? */
        let done_seen = *self.done_seen.entry(*leaf).or_insert(0);
        if done_seen < self.done_now {
            if self.is_done() {
                self.done_seen.insert(*leaf,self.done_now);
                return ComponentRedo::Major;
            }
        }
        redo
    }
}
