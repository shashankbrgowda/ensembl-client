use std::rc::Rc;
use std::cell::RefCell;

use stdweb::web::{
    INode,
    TextBaseline,
    CanvasRenderingContext2d,
};

use stdweb::web::html_element::CanvasElement;
use stdweb::web::TypedArray;
use webgl_rendering_context::WebGLRenderingContext as glctx;

use types::{
    Colour, CPixel, RPixel, cpixel, Dot
};
use shape::{ CanvasIdx, ShapeContext };
use arena::{ ArenaPrograms };

use program::CanvasWeave;

#[derive(Clone,Copy,Debug)]
pub enum FontVariety {
    Normal,
    Bold
}

impl FontVariety {
    pub fn text(&self) -> &str {
        match self {
            FontVariety::Normal => "",
            FontVariety::Bold => "bold ",
        }
    }
}

#[derive(Clone,Eq,PartialEq,Hash)]
pub struct FCFont {
    spec: String,
    height: i32,
    xpad: i32,
    ypadtop: i32,
    ypadbot: i32
}

impl FCFont {
    pub fn new(size : i32, family: &str, v: FontVariety) -> FCFont {
        FCFont { spec: format!("{}{}px {}",v.text(),size,family),
                 height: size, ypadtop: 0, ypadbot: 4, xpad: 1 }
    }
    
    fn setup(&self, canvas : &CanvasRenderingContext2d) {
        canvas.set_font(&self.spec);
    }
}

pub struct FlatCanvasImpl {
    canvas: CanvasElement,
    context : CanvasRenderingContext2d,
    width: i32,
    height: i32,
    weave: CanvasWeave,
    index: CanvasIdx
}

impl FlatCanvasImpl {    
    pub fn create(canvas: CanvasElement, index: usize,
                  width: i32, height: i32, weave: CanvasWeave) -> FlatCanvasImpl {
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        let context : CanvasRenderingContext2d = canvas.get_context().unwrap();
        context.set_fill_style_color("black");
        FlatCanvasImpl {
            canvas, context, height, width,  weave,
            index: CanvasIdx::new(index)
        }
    }

    pub fn apply_context(&mut self, progs: &mut ArenaPrograms, ctx: &glctx) {
        self.index.reset();
        for (ref gk,ref mut geom) in progs.map.iter_mut() {
            self.index.into_objects(gk,&mut geom.data,ctx);
        }
    }
    
    pub fn remove(&self) {
        self.canvas.parent_node().unwrap().remove_child(&self.canvas).ok();
    }
    
    pub fn text(&self,text : &str, pos: CPixel, font: &FCFont, col: &Colour, bg: &Colour) -> (i32,i32) {
        font.setup(&self.context);
        let m = self.context.measure_text(text);
        let width_px = m.unwrap().get_width().ceil() as i32;
        let height_px = font.height;
        let fullwidth_px = width_px + 2*font.xpad;
        let fullheight_px = height_px + font.ypadtop + font.ypadbot;
        self.context.set_fill_style_color(&bg.to_css()[..]);
        self.context.fill_rect(pos.0 as f64,pos.1 as f64,
                               fullwidth_px as f64,
                               fullheight_px as f64);
        self.context.set_text_baseline(TextBaseline::Top);
        self.context.set_fill_style_color(&col.to_css()[..]);
        self.context.set_stroke_style_color(&col.to_css()[..]);
        self.context.fill_text(text,(pos.0+font.xpad).into(),(pos.1+font.ypadtop).into(),None);
        (fullwidth_px,fullheight_px)
    }
    
    pub fn bitmap(&self, data: &Vec<u8>, coords: RPixel) {
        let pixels: TypedArray<u8> = data[..].into();
        let Dot(x,y) = coords.offset();
        let Dot(w,h) = coords.area();
        js! {
            var id = @{&self.context}.createImageData(@{w},@{h});
            id.data.set(@{pixels});
            @{&self.context}.putImageData(id,@{x},@{y});
        };
    }
    
    pub fn rectangle(&self, coords: RPixel, col: &Colour) {
        let Dot(x,y) = coords.offset();
        let Dot(w,h) = coords.area();
        self.context.set_fill_style_color(&col.to_css()[..]);
        self.context.fill_rect(x as f64,y as f64,w as f64,h as f64);
    }

    pub fn measure(&self,text : &str, font: &FCFont) -> CPixel {
        font.setup(&self.context);
        let m = self.context.measure_text(text);
        let width_px = m.unwrap().get_width().ceil() as i32;
        let height_px = font.height;
        cpixel(width_px+2*font.xpad,height_px+font.ypadtop+font.ypadbot)
    }
    
    pub fn element(&self) -> &CanvasElement {
        &self.canvas
    }
    
    pub fn size(&self) -> CPixel {
        cpixel(self.width,self.height)
    }
    
    pub fn index(&self) -> &CanvasIdx { &self.index }
    pub fn weave(&self) -> &CanvasWeave { &self.weave }
}

#[derive(Clone)]
pub struct FlatCanvas(Rc<RefCell<FlatCanvasImpl>>);

impl FlatCanvas {
    pub fn create(canvas: CanvasElement, index: usize,
                  width: i32, height: i32, weave: CanvasWeave) -> FlatCanvas {
        FlatCanvas(Rc::new(RefCell::new(
            FlatCanvasImpl::create(canvas,index,width,height,weave)
        )))
    }

    pub fn apply_context(&mut self, progs: &mut ArenaPrograms, ctx: &glctx) {
        self.0.borrow_mut().apply_context(progs,ctx);
    }
    
    pub fn remove(&self) {
        self.0.borrow().remove();
    }
    
    pub fn text(&self,text : &str, pos: CPixel, font: &FCFont, col: &Colour, bg: &Colour) -> (i32,i32) {
        self.0.borrow().text(text,pos,font,col,bg)
    }
    
    pub fn bitmap(&self, data: &Vec<u8>, coords: RPixel) {
        self.0.borrow().bitmap(data,coords);
    }
    
    pub fn rectangle(&self, coords: RPixel, col: &Colour) {
        self.0.borrow().rectangle(coords,col);
    }

    pub fn measure(&self,text : &str, font: &FCFont) -> CPixel {
        self.0.borrow().measure(text,font)
    }
    
    pub fn element(&self) -> CanvasElement {
        self.0.borrow().element().clone()
    }
    
    pub fn size(&self) -> CPixel {
        self.0.borrow().size()
    }
    
    pub fn index(&self) -> CanvasIdx { self.0.borrow().index().clone() }
    pub fn weave(&self) -> CanvasWeave { self.0.borrow().weave.clone() }
}
