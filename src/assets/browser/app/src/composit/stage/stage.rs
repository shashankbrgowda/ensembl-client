use std::collections::HashMap;

use composit::{ Leaf, Wrapping };
use controller::output::{ Report, ViewportReport };
use drivers::webgl::program::UniformValue;
use types::{
    CPixel, Move, Dot, Direction,
    LEFT, RIGHT, UP, DOWN, IN, OUT, XPosition, YPosition, Placement
};
use super::position::Position;

// XXX TODO avoid big-minus-big type calculations which accumulate error

#[derive(Debug)]
pub struct Stage {
    dims: Dot<f64,f64>,
    mouse_pos: CPixel,
    base: f64,
    apos: Position,
    ipos: Position,
    valid_pos_intent: bool
}

impl Stage {
    pub fn new() -> Stage {
        let size = Dot(0.,0.);
        Stage {
            apos: Position::new(Dot(0.,0.),size),
            ipos: Position::new(Dot(0.,0.),size),
            mouse_pos: Dot(0,0),
            base: 0.,
            dims: size,
            valid_pos_intent: false
        }
    }

    pub fn intend_here(&mut self) {
        self.ipos.set_middle(&self.apos.get_middle());
        self.ipos.set_zoom(self.apos.get_zoom());
    }

    pub fn set_pos_intent(&mut self, valid: bool) {
        self.valid_pos_intent = valid;
    }

    fn bumped(&self, direction: &Direction) -> bool {
        let mul : f64 = direction.1.into();
        self.apos.get_edge(direction,true).floor() * mul >= self.apos.get_limit_of_edge(direction).floor() * mul
    }

    pub fn update_report(&self, report: &Report) {
        let (aleft,aright) = (self.apos.get_edge(&LEFT,false),self.apos.get_edge(&RIGHT,false));
        report.set_status("a-start",&aleft.floor().to_string());
        report.set_status("a-end",&aright.ceil().to_string());
        let (ileft,iright) = (self.ipos.get_edge(&LEFT,false),self.ipos.get_edge(&RIGHT,false));
        report.set_status("i-start",&ileft.floor().to_string());
        report.set_status("i-end",&iright.ceil().to_string());
        report.set_status_bool("bumper-left",self.bumped(&LEFT));
        report.set_status_bool("bumper-right",self.bumped(&RIGHT));
        report.set_status_bool("bumper-top",self.bumped(&UP));
        report.set_status_bool("bumper-bottom",self.bumped(&DOWN));
        report.set_status_bool("bumper-in",self.bumped(&IN));
        report.set_status_bool("bumper-out",self.bumped(&OUT));
    }

    pub fn update_viewport_report(&self, report: &ViewportReport) {
        report.set_delta_y(-self.apos.get_edge(&UP,false) as i32);
    }

    pub fn set_wrapping(&mut self, w: &Wrapping) {
        self.apos.set_bumper(&LEFT,w.get_bumper(&LEFT));
        self.apos.set_bumper(&RIGHT,w.get_bumper(&RIGHT));
        self.ipos.set_bumper(&LEFT,w.get_bumper(&LEFT));
        self.ipos.set_bumper(&RIGHT,w.get_bumper(&RIGHT));
    }

    pub fn set_mouse_pos(&mut self, c: &CPixel) {
        self.mouse_pos = *c;
    }
    
    pub fn set_screen_in_bp(&mut self, zoom: f64) {
        self.apos.set_screen_in_bp(zoom);
        self.ipos.set_screen_in_bp(zoom);
    }
 
    pub fn settle(&mut self) { self.apos.settle(); }
    
    pub fn get_mouse_pos_prop(&self) -> f64 {
        self.mouse_pos.0 as f64 / self.get_size().0 as f64
    }

    fn get_pos_prop_bp(&self, prop: f64) -> f64 {
        let start = self.get_pos_middle().0 - self.apos.get_linear_zoom() / 2.;
        start + prop * self.apos.get_linear_zoom()
    }

    pub fn get_mouse_pos_bp(&self) -> f64 {
        self.get_pos_prop_bp(self.get_mouse_pos_prop())
    }

    pub fn pos_prop_bp_to_origin(&self, pos: f64, prop: f64) -> f64 {
        let start = pos - prop * self.apos.get_linear_zoom();
        start + self.apos.get_linear_zoom()/2.
    }

    pub fn set_limit(&mut self, which: &Direction, val: f64) {
        self.apos.set_limit(which,val);
        self.ipos.set_limit(which,val);
    }
    
    pub fn get_screen_in_bp(&self) -> f64 {
        self.apos.get_screen_in_bp()
    }
    
    pub fn get_pos_middle(&self) -> Dot<f64,f64> {
        self.apos.get_middle()
    }

    pub fn set_zoom(&mut self, v: f64) {
        self.apos.set_zoom(v);
    }
    
    pub fn get_zoom(&self) -> f64 {
        self.apos.get_zoom()
    }

    pub fn get_linear_zoom(&self) -> f64 {
        self.apos.get_linear_zoom()
    }

    pub fn set_pos_middle(&mut self, pos: &Dot<f64,f64>) {
        self.apos.set_middle(pos);
    }

    pub fn get_size(&self) -> Dot<f64,f64> {
        self.dims
    }

    pub fn set_size(&mut self, size: &Dot<f64,f64>) {
        self.dims = *size;
        self.apos.inform_screen_size(size);
        self.ipos.inform_screen_size(size);
    }

    pub fn get_uniforms(&self, leaf: &Leaf, opacity: f32) -> HashMap<&str,UniformValue> {
        let bp_per_screen = self.apos.get_linear_zoom();
        let bp_per_leaf = leaf.total_bp();
        let leaf_per_screen = bp_per_screen as f64 / bp_per_leaf;
        let middle_bp = self.apos.get_middle();
        let middle_leaf = middle_bp.0/bp_per_leaf; // including fraction of leaf
        let current_leaf_left = leaf.get_index() as f64;
        hashmap! {
            "uOpacity" => UniformValue::Float(opacity),
            "uStageHpos" => UniformValue::Float((middle_leaf - current_leaf_left) as f32),
            "uStageVpos" => UniformValue::Float(middle_bp.1 as f32),
            "uStageZoom" => UniformValue::Float((2_f64/leaf_per_screen) as f32),
            "uSize" => UniformValue::Vec2F(
                self.dims.0 as f32/2.,
                self.dims.1 as f32/2.)
        }
    }
    
    pub fn intersects(&self, pos: Dot<i32,i32>, area: &Placement) -> bool {
        let screen_bp = self.get_screen_in_bp();
        let screen_px = self.dims;
        let bp_px = screen_bp / screen_px.0;
        let left_bp = self.apos.get_edge(&LEFT,false);
        let top_px = self.apos.get_edge(&UP,false);
        match area {
            Placement::Stretch(r) => {
                let pos_bp = left_bp + pos.0 as f64 * bp_px;
                let nw = r.offset();
                let se = r.far_offset();
                bb_log!("zmenu","Q {:?}<={:?}[{:?}+{:?}*{:?}]<={:?} {:?}<={:?}<={:?}",
                            nw.0,pos_bp,left_bp,pos.0,bp_px,se.0,
                            nw.1,pos.1+top_px as i32,se.1);
                nw.0 as f64 <= pos_bp && se.0 as f64 >= pos_bp &&
                nw.1 <= pos.1+top_px as i32 && se.1 >= pos.1+top_px as i32
            },
            Placement::Placed(x,y) => {
                let (x0,x1) = match x {
                    XPosition::Base(bp,s,e) => {
                        let px = (bp-left_bp) / bp_px;
                        (px+*s as f64,px+*e as f64)
                    },
                    XPosition::Pixel(s,e) => {
                        (s.min_dist(screen_px.0 as i32) as f64,
                         e.min_dist(screen_px.0 as i32) as f64)
                    }
                };
                let (y0,y1) = match y {
                    YPosition::Page(s,e) => {
                        (*s as f64-top_px, *e as f64-top_px)
                    }
                    YPosition::Pixel(s,e) => {
                        (s.min_dist(screen_px.1 as i32) as f64,
                         e.min_dist(screen_px.1 as i32) as f64)
                    }
                };
                bb_log!("zmenu","P {:?}<={:?}<={:?} {:?}<={:?}<={:?}",
                            x0,pos.0,x1, y0,pos.1,y1);
                x0 <= pos.0 as f64 && x1 >= pos.0 as f64 &&
                y0 <= pos.1 as f64 && y1 >= pos.1 as f64
            }
        }
    }
}