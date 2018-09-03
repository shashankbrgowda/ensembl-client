use std::rc::Rc;
use arena::{ ArenaData, Arena };

use program::ProgramAttribs;
use coord::{ CPixel, Colour, RPixel };

use shape::Shape;
use shape::util::{ rectangle_p, rectangle_t, multi_gl, vertices_rect };

use drawing::{ Drawing };
use campaign::OnOffExpr;

/*
 * FixRect
 */

pub struct FixRect {
    points: RPixel,
    colour: Colour,
    geom: String
}

impl FixRect {
    pub fn new(points: RPixel, colour: Colour, geom: &str) -> FixRect {
        FixRect { points, colour, geom: geom.to_string() }
    }    
}

impl Shape for FixRect {
    fn into_objects(&self, _geom_name: &str, geom: &mut ProgramAttribs, _adata: &ArenaData) {
        let b = vertices_rect(geom,None);
        rectangle_p(b,geom,"aVertexPosition",&self.points);
        multi_gl(b,geom,"aVertexColour",&self.colour,4);
    }
    
    fn get_geometry(&self) -> &str { &self.geom }
}

fn rectangle(arena: &mut Arena, p: &RPixel, colour: &Colour, geom: &str, ooe: Rc<OnOffExpr>) {
    arena.add_shape(None,Box::new(FixRect::new(*p,*colour,geom)),ooe);
}

pub fn fix_rectangle(arena: &mut Arena, p: &RPixel, colour: &Colour, ooe: Rc<OnOffExpr>) {
    rectangle(arena, p, colour, "fix",ooe);
}

#[allow(dead_code)]
pub fn page_rectangle(arena: &mut Arena, p: &RPixel, colour: &Colour, ooe: Rc<OnOffExpr>) {
    rectangle(arena, p, colour, "page",ooe);
}

/*
 * FixTexture
 */

pub struct FixTexture {
    pos: CPixel,
    scale: CPixel,
    texpos: Option<RPixel>,
    geom: String
}

impl FixTexture {
    pub fn new(pos: &CPixel, scale: &CPixel, geom: &str) -> FixTexture {
        FixTexture {
            pos: *pos, scale: *scale, texpos: None, geom: geom.to_string()
        }
    }    
}

impl Shape for FixTexture {
    fn set_texpos(&mut self, data: &RPixel) {
        self.texpos = Some(*data);
    }
  
    fn into_objects(&self, _geom_name:&str, geom: &mut ProgramAttribs, adata: &ArenaData) {
        if let Some(tp) = self.texpos {
            let p = tp.at_origin() * self.scale + self.pos;
            let t = tp / adata.canvases.flat.size();
            let b = vertices_rect(geom,None);
            rectangle_p(b,geom,"aVertexPosition",&p);
            rectangle_t(b,geom,"aTextureCoord",&t);
        }
    }
    
    fn get_geometry(&self) -> &str { &self.geom }
}

fn texture(arena: &mut Arena,req: Drawing, origin: &CPixel, scale: &CPixel, geom: &str, ooe: Rc<OnOffExpr>) {
    let ri = FixTexture::new(origin,scale,geom);
    arena.add_shape(Some(req),Box::new(ri),ooe);
}


pub fn fix_texture(arena: &mut Arena,req: Drawing, origin: &CPixel, scale: &CPixel, ooe: Rc<OnOffExpr>) {
    texture(arena, req, origin, scale, "fixtex",ooe);
}

pub fn page_texture(arena: &mut Arena,req: Drawing, origin: &CPixel, scale: &CPixel, ooe: Rc<OnOffExpr>) {
    texture(arena, req, origin, scale, "pagetex",ooe);
}
