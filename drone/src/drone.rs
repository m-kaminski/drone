use miniquad::*;


//use crate::shader::*;

use crate::render::*;


#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

pub struct  Drone {
    bindings: Bindings,
    pub x: f32,
    pub y: f32,
}

impl Render for Drone {
    fn render(&self, ctx: &mut Context, pipeline: & Pipeline) {
       
       ctx.apply_pipeline(pipeline);


       ctx.apply_bindings(&self.bindings);
       // begining and length of index buffer (essentially number of vertices) and number of instances
       ctx.draw(0, 6, 1);

       ctx.apply_uniforms(&crate::shader::Uniforms {
           offset: (*&self.x, *&self.y),
       });
       
   }
}

impl Drone {
    pub fn new(ctx: &mut Context) -> Drone {
       
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -0.2, y: 0.3 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  0.2, y: 0.3 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  0.2, y: -0.3 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -0.2, y: -0.3 }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);
        let texture = gen_texture(ctx, 64,64,
r#"________________________________________________________________
________________________________________________________________
________________________________________________________________
________________________________________________________________
________________________________________________________________
________________________________________________________________
________________________________________________________________
__GGGGGG________________________________________________________
__GGGGGG________________________________________________________
_GGGGGGG________________________________________________________
_GGGGGGG_____________G__________________________________________
_GGGGGGG_____________G__________________________________________
_GGGGGGG_____________G__________________________________________
GGGGGGGG_____________G__________________________________________
GGGGGGGG_____________GGGGGGGGG__________________________________
GGGGGGGG____________GGGGGGGGGGGGGGGGGGGGGGG_____________________
GGGGGGGGGGGG_______GGGGGGGGGGGGGGGGGGGGGGGGGGGGG________________
GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG____________
___________GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG________
_____________________G_GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG_______
_____________________G__GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG
_____________________G__GGG__GGGGGGGGGGGGGGGGGGGGGGGGGGGG_______
_____________________G_________GG________GGGGGGGGGGGGG__________
_______________________________GG________GGGGG_GG_______________
_______________________________GG_________GGG__GG_______________
______________________________GGGG______________GG______________
______________________________GGGG_____________GGGG_____________
_______________________________GG______________GGGG_____________
________________________________________________GG______________
________________________________________________________________
________________________________________________________________
________________________________________________________________
"#);
        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture],
        }; 
        Drone { bindings, x:-0.3, y:0.4 }

    }
}


