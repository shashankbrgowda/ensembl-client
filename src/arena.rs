use webgl_rendering_context::{
    WebGLRenderingContext as glctx,
};

use std::cell::RefCell;
use std::rc::Rc;

use canvasutil;
use wglraw;
use geometry::Geometry;
use geometry::stretch::StretchGeometry;
use geometry::pin::PinGeometry;
use geometry::pintex::PinTexGeometry;
use geometry::fix::FixGeometry;

use canvasutil::FCFont;

use texture::text::TextTextureStore;
use texture::bitmap::BitmapTextureStore;

struct ArenaGeometries {
    stretch: StretchGeometry,
    pin: PinGeometry,
    fix: FixGeometry,
    pintex: PinTexGeometry,
}

struct ArenaTextures {
    text: TextTextureStore,
    bitmap: BitmapTextureStore,
}

impl ArenaTextures {
    pub fn new() -> ArenaTextures {
        ArenaTextures {
            text: TextTextureStore::new(),
            bitmap: BitmapTextureStore::new(),
        }
    }
}

use alloc::Allocator;

pub struct ArenaCanvases {
    pub flat_alloc: Allocator,
    pub flat: Rc<canvasutil::FlatCanvas>,
}

pub struct ArenaData {
    spec: ArenaSpec,
    textures: ArenaTextures,
    pub canvases: ArenaCanvases,
    pub ctx: glctx,
    pub aspect: f32,
    pub width_px: u32,
    pub height_px: u32,
}

impl ArenaData {
    pub fn prop_x(&self,x_px: u32) -> f32 {
        (x_px as f64 * 2.0 / self.width_px as f64) as f32
    }

    pub fn prop_y(&self,y_px: u32) -> f32 {
        (y_px as f64 * 2.0 / self.height_px as f64) as f32
    }
    
    fn nudge1(&self,val: f32, size: u32) -> f32 {
        let n = (val * size as f32 / 2.).round();
        n * 2. / size as f32
    }
    
    pub fn nudge(&self,input: [f32;2]) -> [f32;2] {
        [self.nudge1(input[0],self.width_px),
         self.nudge1(input[1],self.height_px)]
    }
    
    pub fn settle(&self, stage: &mut Stage) {
        // XXX settle to account for zoom
        let [hpos,vpos] = self.nudge([stage.hpos,stage.vpos]);
        stage.hpos = hpos;
        stage.vpos = vpos;
    }
    
    /* help the borrow checker by splitting a mut in a way that it
     * understands is disjoint.
     */
    fn burst_texture<'a>(&'a mut self) -> (&'a mut ArenaCanvases, &'a mut ArenaTextures) {
        (&mut self.canvases,&mut self.textures)
    }

}

pub struct ArenaSpec {
    pub flat_width: u32,
    pub flat_height: u32,
    pub debug: bool,
}

impl ArenaSpec {
    pub fn new() -> ArenaSpec {
        ArenaSpec {
            flat_width: 256,
            flat_height: 256,
            debug: false
        }
    }
}

pub struct Arena {
    data: Rc<RefCell<ArenaData>>,
    geom: ArenaGeometries,
}

impl Arena {
    pub fn new(selector: &str, mcsel: &str, spec: ArenaSpec) -> Arena {
        let canvas = canvasutil::prepare_canvas(selector,mcsel,spec.debug);
        let ctx = wglraw::prepare_context(&canvas);
        let flat = Rc::new(canvasutil::FlatCanvas::create(2,2));
        let data = Rc::new(RefCell::new(ArenaData {
            ctx, spec, 
            textures: ArenaTextures::new(),
            aspect: canvasutil::aspect_ratio(&canvas),
            width_px: canvas.width(),
            height_px: canvas.height(),
            canvases: ArenaCanvases {
                flat,
                flat_alloc: Allocator::new(16),
            }
        }));
        let data_g = data.clone();
        let data_b = data_g.borrow();
        let arena = Arena { data, geom: ArenaGeometries {
            stretch: StretchGeometry::new(&data_b),
            pin:     PinGeometry::new(&data_b),
            fix:     FixGeometry::new(&data_b),
            pintex:  PinTexGeometry::new(&data_b),
        }};
        arena
    }

    pub fn settle(&self, stage: &mut Stage) {
        self.data.borrow().settle(stage);
    }  

    pub fn triangle_stretch(&mut self, p: &[f32;6], c: &[f32;3]) {
        self.geom.stretch.triangle(p,c);
    }

    pub fn rectangle_stretch(&mut self, p: &[f32;4], c: &[f32;3]) {
        self.geom.stretch.rectangle(p,c);
    }

    pub fn triangle_pin(&mut self, r: &[f32;2], p: &[f32;6], c :&[f32;3]) {
        self.geom.pin.triangle(r,p,c);
    }

    pub fn text_pintex(&mut self, origin:&[f32;2],chars: &str,font: &FCFont) {
        let datam = &mut self.data.borrow_mut();
        let (canvases,textures) = datam.burst_texture();
        textures.text.add(&mut self.geom.pintex,canvases,origin,chars,font);
    }

    pub fn bitmap_pintex(&mut self, origin:&[f32;2], scale: &[f32;2], data: Vec<u8>, width: u32, height: u32) {
        let datam = &mut self.data.borrow_mut();
        let (canvases,textures) = datam.burst_texture();
        textures.bitmap.add(&mut self.geom.pintex,canvases,origin,scale,data,width,height);
    }

    pub fn triangle_fix(&mut self,points:&[f32;9],colour:&[f32;3]) {
        self.geom.fix.triangle(points,colour);
    }
    
    pub fn rectangle_fix(&mut self,p:&[f32;6],colour:&[f32;3]) {
        self.geom.fix.rectangle(p,colour);
    }

    pub fn populate(&mut self) {
        let datam = &mut self.data.borrow_mut();
        {
            let canvases = &mut datam.canvases;
            let (x,y) = canvases.flat_alloc.allocate();
            canvases.flat = Rc::new(canvasutil::FlatCanvas::create(x,y));
        }

        self.geom.stretch.populate(datam);
        self.geom.pin.populate(datam);
        self.geom.pintex.populate(datam);
        self.geom.fix.populate(datam);
    }

    pub fn animate(&mut self, stage: &Stage) {
        // prepare arena
        {
            let ctx = &self.data.borrow().ctx;
            ctx.enable(glctx::DEPTH_TEST);
            ctx.depth_func(glctx::LEQUAL);
        }
        // draw each geometry
        let datam = &mut self.data.borrow_mut();
        self.geom.stretch.draw(datam,&stage);
        self.geom.pin.draw(datam,&stage);
        self.geom.pintex.draw(datam,&stage);
        self.geom.fix.draw(datam,&stage);
    }
}



pub struct Stage {
    pub hpos: f32,
    pub vpos: f32,
    pub zoom: f32,
    pub cursor: [f32;2],
}

impl Stage {
    pub fn new() -> Stage {
        Stage { hpos: 0.0, vpos: 0.0, zoom: 1.0, cursor: [0.0,0.0] }
    }
}
