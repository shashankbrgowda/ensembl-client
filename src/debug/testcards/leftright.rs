use std::collections::HashSet;
use std::ops::Range;
use std::rc::Rc;
use std::sync::{ Arc, Mutex };

use separator::Separatable;

use debug::testcards::closuresource::{ ClosureSource, closure_add, closure_done };
use composit::{ StateFixed, Component, StateValue, vscale_bp_per_leaf };
use controller::global::Global;
use controller::input::Event;
use drawing::{ FCFont, FontVariety, text_texture };
use shape::{ ColourSpec, pin_texture, stretch_rectangle, pin_rectangle };
use types::{ Colour, cleaf, cpixel, A_TOP, area, area_size };

const TARGET: i32 = 20;
const MARK_TARGET: i32 = 1000;

const BY125 : &[f64] = &[0.05,0.1,0.2,0.5,1.,2.,5.,10.,20.];
const BY12 : &[f64] =  &[     0.1,0.2,    1.,   5.,10.    ];
const BY15 : &[f64] =  &[0.05,0.1,    0.5,1.,2.,   10.,20.];
const BY1 : &[f64] =   &[     0.1,        1.,      10.    ];

fn goby(start: f64, bp_leaf: f64, target: i32, by: &[f64]) -> f64 {
    let approx = 10_i64.pow((bp_leaf/5.).log10().round() as u32) as f64;
    for mul in by {
        let step = approx*mul;
        let end = start+bp_leaf;
        let start_round = (start/step).round();
        let end_round = (end/step).round();
        let num = (end_round-start_round+1.).floor();
        if num < target.into() { return step; }
    }
    approx
}

fn range(start: f64, bp_leaf: f64, target: i32, by: &[f64]) -> impl Iterator<Item=i64> {
    let step = goby(start,bp_leaf,target,by);
    let end = start+bp_leaf;
    ((start/step).ceil() as i64..(end/step).floor() as i64+1).map(move |x| x*step as i64)
}

pub fn testcard_leftright(g: Arc<Mutex<Global>>) {
    let g = &mut g.lock().unwrap();
    let font = FCFont::new(16,"Lato",FontVariety::Normal);
    let cs = ClosureSource::new(0.2,move |lc,leaf| {
        let i = leaf.get_index();
        let mut mul = vscale_bp_per_leaf(leaf.get_vscale());
        let start = leaf.get_index() as f64 * mul;
        for v in range(start,mul,TARGET,BY12) {
            let offset = (v as f64-start)/mul;
            let tx = text_texture(&format!("{}",v.separated_string()),&font,&Colour(199,208,213),&Colour(255,255,255));
            closure_add(lc,&pin_texture(tx,&cleaf(offset as f32,1),
                        &cpixel(0,40),&cpixel(1,1).anchor(A_TOP)));
        }
        let big_mark : HashSet<i64> = range(start,mul,TARGET,BY1).collect();
        let mid_mark : HashSet<i64> = range(start,mul,TARGET,BY15).collect();
        let small_mark : HashSet<i64> = range(start,mul,TARGET,BY125).collect();
        for v in range(start,mul,MARK_TARGET,BY1) {
            let h = if big_mark.contains(&v) {
                30
            } else if mid_mark.contains(&v) {
                20
            } else if small_mark.contains(&v) {
                15
            } else {
                10
            };
            let offset = (v as f64-start)/mul;
            closure_add(lc,&pin_rectangle(&cleaf(offset as f32,10),
                                          &area_size(cpixel(0,0),cpixel(1,h)),
                                          &ColourSpec::Colour(Colour(199,208,213))));
        }
        let (colour,offset) = if i % 2 == 0 {
            (ColourSpec::Colour(Colour(255,0,0)),0)
        } else {
            (ColourSpec::Colour(Colour(0,255,0)),10)
        };
        /*
        closure_add(lc,&stretch_rectangle(&area(
                        cleaf(0.,offset),
                        cleaf(1.,offset+10),
                     ),&colour));
        */
        closure_done(lc,60);
    });

    let c = Component::new(Box::new(cs.clone()),Rc::new(StateFixed(StateValue::On())));

    g.with_state(|s| {
        s.with_compo(|co| {
            co.add_component(c);
        });
        s.run_events(vec!{ Event::Zoom(-1.) });
    });
    
    
    
}
