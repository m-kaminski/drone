use miniquad::*;

use crate::render::*;


pub struct Missile {
    bindings: Bindings,
    pub x: f32,
    pub y: f32,
    pub frame: i32,
}

impl Render for Missile {
    fn render(&self, ctx: &mut Context, pipeline: & Pipeline) {
       
       ctx.apply_pipeline(pipeline);


       ctx.apply_bindings(&self.bindings);
       // begining and length of index buffer (essentially number of vertices) and number of instances

       ctx.apply_uniforms(&crate::shader::Uniforms {
        offset: (*&self.x, *&self.y),
    });       
    
    ctx.draw( ((self.frame /10)%4)*6, 6, 1);
   }

   fn animate(&mut self, tdelta: f32) {
     self.x+=0.01;
     self.frame += 1;
   }

}

impl Missile {
    pub fn new(ctx: &mut Context) -> Missile {
       
        #[rustfmt::skip]
        let vertices: [Vertex; 16] = [
            // first frame of animation
            Vertex { pos : Vec2 { x: -0.25/2., y: 0.125/4. }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  0.25/2., y: 0.125/4. }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  0.25/2., y: -0.125/4. }, uv: Vec2 { x: 1., y: 0.25 } },
            Vertex { pos : Vec2 { x: -0.25/2., y: -0.125/4. }, uv: Vec2 { x: 0., y: 0.25 } },

            // second frame of animation
            Vertex { pos : Vec2 { x: -0.25/2., y: 0.125/4. }, uv: Vec2 { x: 0., y: 0.25 } },
            Vertex { pos : Vec2 { x:  0.25/2., y: 0.125/4. }, uv: Vec2 { x: 1., y: 0.25 } },
            Vertex { pos : Vec2 { x:  0.25/2., y: -0.125/4. }, uv: Vec2 { x: 1., y: 0.5 } },
            Vertex { pos : Vec2 { x: -0.25/2., y: -0.125/4. }, uv: Vec2 { x: 0., y: 0.5 } },
            // first frame of animation
            Vertex { pos : Vec2 { x: -0.25/2., y: 0.125/4. }, uv: Vec2 { x: 0., y: 0.5 } },
            Vertex { pos : Vec2 { x:  0.25/2., y: 0.125/4. }, uv: Vec2 { x: 1., y: 0.5 } },
            Vertex { pos : Vec2 { x:  0.25/2., y: -0.125/4. }, uv: Vec2 { x: 1., y: 0.75 } },
            Vertex { pos : Vec2 { x: -0.25/2., y: -0.125/4. }, uv: Vec2 { x: 0., y: 0.75 } },

            // second frame of animation
            Vertex { pos : Vec2 { x: -0.25/2., y: 0.125/4. }, uv: Vec2 { x: 0., y: 0.75 } },
            Vertex { pos : Vec2 { x:  0.25/2., y: 0.125/4. }, uv: Vec2 { x: 1., y: 0.75 } },
            Vertex { pos : Vec2 { x:  0.25/2., y: -0.125/4. }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -0.25/2., y: -0.125/4. }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 24] = [0, 1, 2, 0, 2, 3,
                                 4, 5, 6, 4, 6, 7,
                                 8, 9, 10, 8, 10, 11,
                                 12, 13, 14, 12, 14, 15];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);
        let texture = gen_texture(ctx, 32,32,
r#"________________________________
________________________________
________________________________
_____________DDDDDDDDDDDDDDDD___
____RRRRRRR__GGGGGGGGGGGGGGGGDDD
_____________GGGGGGGGGGGGGGGG___
________________________________
________________________________

________________________________
________________________________
________________________________
________RR___DDDDDDDDDDDDDDDD___
____RRRYYYR__GGGGGGGGGGGGGGGGDDD
________RR___GGGGGGGGGGGGGGGG___
________________________________
________________________________


________________________________
________________________________
________________________________
_____RRRRR___DDDDDDDDDDDDDDDD___
__RRRYWWYYR__GGGGGGGGGGGGGGGGDDD
_____RRRRR___GGGGGGGGGGGGGGGG___
________________________________
________________________________

________________________________
________________________________
________________________________
_____RR______DDDDDDDDDDDDDDDD___
__RRRYRRRRR__GGGGGGGGGGGGGGGGDDD
_____RR______GGGGGGGGGGGGGGGG___
________________________________
________________________________

"#);
        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture],
        }; 
        Missile { bindings, x:-0.1, y:0.4, frame: 0 }

    }
}


