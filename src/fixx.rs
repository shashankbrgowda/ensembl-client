use arena::{
    Geometry,
    ArenaData,
    StdGeometry,
    Stage,
};

use std::cell::RefCell;
use std::rc::Rc;

const V_SRC : &str = "
attribute vec3 aVertexPosition;
attribute vec3 aVertexColour;

uniform vec2 uCursor;
uniform float uAspect;

varying lowp vec3 vColour;

void main() {
    gl_Position = vec4(
        aVertexPosition.x - uCursor.x,
        ( aVertexPosition.y + aVertexPosition.z * uAspect ) - uCursor.y,
        0.0, 1.0
    );
    vColour = aVertexColour;
}
";

const F_SRC : &str = "
varying lowp vec3 vColour;

void main() {
      gl_FragColor = vec4(vColour, 1.0);
}
";

pub struct FixxGeometry {
    std : StdGeometry,
}

impl FixxGeometry {
    pub fn new(adata: Rc<RefCell<ArenaData>>) -> FixxGeometry {
        let std = StdGeometry::new(
            adata.clone(),&V_SRC,&F_SRC,
            &[("aVertexPosition",3,1),("aVertexColour",3,3)],3);        
        FixxGeometry {
            std,
        }
    }

    pub fn triangle(&mut self,points:[f32;9],colour:[f32;3]) {
        self.std.add(0,&points);
        self.std.add(1,&colour);
        self.std.advance();
    }
}
impl Geometry for FixxGeometry {    
    fn populate(&mut self) {
        self.std.populate();
    }

    fn draw(&self) {
        self.std.draw();
    }
    
    fn perspective(&self,stage:&Stage) {
        self.std.perspective(stage);
    }    
}
