use compiler::GLProgram;
use arena::ArenaData;

use geometry::shader::{ shader_solid, shader_texture };

use wglprog::{
    GLSource,
    Statement,
    Uniform,
    Attribute,
    Phase,
};

fn fix_prog() -> GLSource {
    GLSource::new(vec! {
        Uniform::new("vec2","uSize",Phase::Vertex),
        Attribute::new(2,"aVertexPosition",Phase::Vertex),
        Statement::new("
            gl_Position = vec4(aVertexPosition.x / uSize.x - 1.0,
                               aVertexPosition.y / uSize.y - 1.0,
                               0.0, 1.0)",Phase::Vertex)
    })
}

pub fn fix_geom(adata: &ArenaData) -> GLProgram {
    GLProgram::new(adata,&shader_solid(&fix_prog()))

}

pub fn fixtex_geom(adata: &ArenaData) -> GLProgram {
    GLProgram::new(adata,&shader_texture(&fix_prog()))
}
