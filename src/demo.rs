use stdweb;
use canvasutil;

use shape::{
    fix_rectangle,
    fix_texture,
    pin_triangle,
    pin_texture,
    stretch_rectangle,
    stretch_texture,
};

use rand::Rng;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use stdweb::web::{
    window
};

use rand::seq;
use std::cell::RefCell;
use std::rc::Rc;

use arena::{
    Arena,
    ArenaSpec,
    Stage,
};

use coord::{
    CLeaf,
    CPixel,
    Colour,
};

use texture::{ text_texture, bitmap_texture };

use rand::distributions::Distribution;
use rand::distributions::range::Range;

struct State {
    arena: RefCell<Arena>,
    stage: Stage,
    zoomscale: f32,
    hpos: f32,
    old_time: f64,
    fpos: f32,
    call: i32,
    phase: u32,
    gear: u32,
    
    grace_next: (u32,u32),
    grace_at: f32,
    last_down: bool,
}

const MAX_GEAR : u32 = 6;
const MAX_GRACE: u32 = 300;
const JANK_WINDOW: f32 = 60.;

fn fib_inc(val: (u32,u32)) -> (u32,u32) {
    (val.1,val.0+val.1)
}

fn fib_dec(val: (u32,u32)) -> (u32,u32) {
    if val.1 > val.0 {
        (val.1-val.0,val.0)
    } else {
        (1,1)
    }
}

fn detect_jank(state : &mut State, delta: u32, time: f32) {
    if delta > state.gear as u32 * 20 {
        if state.gear < MAX_GEAR {
            /* Go up a gear */
            if state.last_down {
                /* Hunting */
                if time > state.grace_at {
                    /* Successful, long hunt. Shorten */
                    state.grace_next = fib_dec(state.grace_next);
                } else {
                    /* Failure, short hunt. Lengthen */
                    if state.grace_next.1 < MAX_GRACE {
                        state.grace_next = fib_inc(state.grace_next);
                    }
                }
            } else {
                /* Moving */
                state.grace_next = (1,1);
            }
            state.grace_at = time + state.grace_next.1 as f32;
            state.last_down = false;
            state.gear += 1;
            js! { console.log(">gear",@{state.gear},@{state.grace_next.1}); };
        }
    }
    if state.grace_at <= time && state.gear > 1 {
        /* Go down a gear */
        if state.last_down {
            /* Moving */
            state.grace_next = (1,1);
        }
        state.grace_at = time + JANK_WINDOW;
        state.last_down = true;
        state.gear -= 1;
        js! { console.log("<gear",@{state.gear},@{state.grace_next.1}); };
    }
}

fn animate(time : f64, s: Rc<RefCell<State>>) {
    {
        let mut state = s.borrow_mut();
        if state.old_time > 0.0 {
            let delta = ((time - state.old_time) / 5000.0) as f32;
            state.call += 1;
            state.zoomscale += delta* 5.0;
            state.hpos += delta *3.763;
            state.fpos += delta *7.21;
            state.stage.zoom = ((state.zoomscale.cos() + 1.5)/3.0) as f32;
            state.stage.pos.0 = ((state.hpos.cos())*1.5) as f32;
        }
        
        let d = time - state.old_time;
        state.old_time = time;
        let stage = state.stage;
        state.stage = stage;
        state.phase += 1;
        if state.phase >= state.gear {
            state.phase = 0;
        }
        if state.phase == 0 {
            detect_jank(&mut state,d as u32,time as f32/1000.0);
            state.arena.borrow_mut().animate(&state.stage);
        }
    }
    window().request_animation_frame(move |x| animate(x,s.clone()));
}

fn choose<R>(rng: &mut R, vals: &[&[&str]]) -> String
                    where R: Rng {
    let mut out = String::new();
    for val in vals {
        out += seq::sample_iter(rng,*val,1).unwrap()[0]
    }
    out
}

fn bio_daft<R>(rng: &mut R) -> String where R: Rng {
    let vals = [ "5'","3'","snp","ins","del",
                 "5'","3'","snp","ins","del",
                 "5'","3'","snp","ins","del",
                 "C","G","A","T" ];
    choose(rng,&[&vals[..]])
}

fn daft<R>(rng: &mut R) -> String where R: Rng {    
    let onset = [ "bl", "br", "ch", "cl", "cr", "dr", "fl",
                       "fr", "gh", "gl", "gr", "ph", "pl", "pr",
                       "qu", "sc", "sh", "sk", "sl", "sm", "sn", "sp",
                       "st", "sw", "th", "tr", "tw", "wh", "wr",
                       "sch", "scr", "shr", "spl", "spr", "squ",
                       "str", "thr", "b", "c", "d", "f", "g", "h", "j",
                       "k", "l", "m", "n", "p", "r", "s", "t", "u", "v",
                       "w", "x", "y", "z" ];
    let nuc = [ "ai", "au", "aw", "ay", "ea", "ee", "ei", "eu",
                    "ew", "ey", "ie", "oi", "oo", "ou", "ow", "oy",
                    "a", "e", "i", "o", "u" ];
    let coda = [  "ch", "ck", "gh", "ng", "ph", "sh", "sm", "sp",
                       "st", "th",  "nth", 
                       "b", "c", "d", "f", "g", "h", "j",
                       "k", "l", "m", "n", "p", "r", "s", "t", "u", "v",
                       "w", "x", "y", "z" ];
    let num_gen = Range::new(1,8);
    let mut out = String::new();
    let num = num_gen.sample(rng);
    for _i in 0..num {
        out += &choose(rng,&[&onset[..],&nuc[..],&coda[..]])[..];
        let sp: bool = rng.gen();
        if sp { out += " "; }
    }
    out
}

pub fn demo() {
    stdweb::initialize();

    let seed = 12345678;
    let s = seed as u8;
    let t = (seed/256) as u8;
    let mut rng = SmallRng::from_seed([s,s,s,s,s,s,s,s,t,t,t,t,t,t,t,t]);
    let fc_font = canvasutil::FCFont::new(12,"Roboto");
    let mut stage = Stage::new();
    stage.zoom = 0.1;

    let mut a_spec = ArenaSpec::new();
    a_spec.debug = false;
    let mut arena = Arena::new("#glcanvas","#managedcanvasholder",a_spec);
    let middle = arena.dims().height_px / 120;
    
    let len_gen = Range::new(0.,0.2);
    let thick_gen = Range::new(0,13);
    let showtext_gen = Range::new(0,10);

    {
        let col = Colour(200,200,200);
        let a = &mut arena;
        for yidx in 0..20 {
            let y = yidx * 60;
            let val = daft(&mut rng);
            let tx = text_texture(a,&val,&fc_font,&col);
            fix_texture(a, tx, &CPixel(0,y+18), &CPixel(1,1));
            if yidx == middle {
                let tx = bitmap_texture(a,
                                    vec! { 0,0,255,255,
                                             255,0,0,255,
                                             0,255,0,255,
                                             255,255,0,255 },4,1);
                stretch_texture(a,tx,&[CLeaf(-10.,y-5),CLeaf(10.,y+5)]);
                let tx = bitmap_texture(a,
                                    vec! { 0,0,255,255,
                                             255,0,0,255,
                                             0,255,0,255,
                                             255,255,0,255 },2,2);
                pin_texture(a,tx,&CLeaf(0.,y-25),&CPixel(10,10));
            } else {
                for idx in -100..100 {
                    let v1 = (idx as f32) * 0.1;
                    let v2 = (idx as f32)+10.0*(yidx as f32) * 0.1;
                    let dx = len_gen.sample(&mut rng);
                    let x = v1 * 1.0 + (yidx as f32).cos();
                    let colour = Colour(
                        (128.*v2.cos()+128.) as u32,
                        (128.*v2.sin()+128.) as u32,
                        (128.*(v2+1.0).sin()+128.) as u32,
                    );
                    let h = if thick_gen.sample(&mut rng) == 0 { 1 } else { 5 };
                    stretch_rectangle(a,&[CLeaf(x,y-h),
                                              CLeaf(x+dx,y+h)],&colour);
                    if idx %5 == 0 {
                        let colour = Colour(colour.2,colour.0,colour.1);
                        pin_triangle(a,&CLeaf(x,y),
                                           &[CPixel(0,0),
                                             CPixel(-5,10),
                                             CPixel(5,10)],
                                           &colour);
                    }
                    if showtext_gen.sample(&mut rng) == 0 {
                        let val = bio_daft(&mut rng);
                        let tx = text_texture(a,&val,&fc_font,&col);
                        pin_texture(a, tx, &CLeaf(x,y-24), &CPixel(1,1));
                    }
                }
            }
        }

        let dims = a.dims();
        let (sw,sh) = (dims.width_px,dims.height_px);
        
        fix_rectangle(a,&[CPixel(sw/2,0),CPixel(sw/2+1,sh)],
                            &Colour(0,0,0));
        let tx = bitmap_texture(a, vec! { 0,0,255,255,
                                     255,0,0,255,
                                     0,255,0,255,
                                     255,255,0,255 },1,4);
        fix_texture(a, tx, &CPixel(99,0),&CPixel(1,sh));
        a.populate();
        a.animate(&stage);
    }

    let state = Rc::new(RefCell::new(State {
        arena: RefCell::new(arena),
        stage,
        hpos: 0.0,
        fpos: 0.0,
        zoomscale: 0.0,
        old_time: -1.0,
        call: 0,
        phase: 0,
        gear: 1,

        grace_next: (1,1),
        grace_at: 0.,
        last_down: true,
    }));

    animate(0.,state);
    stdweb::event_loop();
}

