use miniquad::*;


//use crate::shader::*;

use crate::render::*;




pub struct  Drone {
    bindings: Bindings,
    pub x: f32,
    pub y: f32,
    pub frame: i32,
}

impl Render for Drone {
    fn render(&self, ctx: &mut Context, pipeline: & Pipeline) {
       
       ctx.apply_pipeline(pipeline);


       ctx.apply_bindings(&self.bindings);
       // begining and length of index buffer (essentially number of vertices) and number of instances

       ctx.apply_uniforms(&crate::shader::Uniforms {
        offset: (*&self.x, *&self.y),
    });       
    
    ctx.draw( ((self.frame /10)%2)*6, 6, 1);

       
   }


   fn animate(&mut self, tdelta: f32) {
    self.frame += 1;
   }
}

impl Drone {
    pub fn new(ctx: &mut Context) -> Drone {
       
        #[rustfmt::skip]
        let vertices: [Vertex; 8] = [
            // first frame of animation
            Vertex { pos : Vec2 { x: -0.25, y: 0.125 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  0.25, y: 0.125 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  0.25, y: -0.125 }, uv: Vec2 { x: 1., y: 0.5 } },
            Vertex { pos : Vec2 { x: -0.25, y: -0.125 }, uv: Vec2 { x: 0., y: 0.5 } },

            // second frame of animation
            Vertex { pos : Vec2 { x: -0.25, y: 0.125 }, uv: Vec2 { x: 0., y: 0.5 } },
            Vertex { pos : Vec2 { x:  0.25, y: 0.125 }, uv: Vec2 { x: 1., y: 0.5 } },
            Vertex { pos : Vec2 { x:  0.25, y: -0.125 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -0.25, y: -0.125 }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 12] = [0, 1, 2, 0, 2, 3,
                                 4, 5, 6, 4, 6, 7];
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
_GGGGBBB________________________________________________________
_GGGGYYY_____________K__________________________________________
_GGDDGGG_____________K__________________________________________
_GGGDGGG_____________K__________________________________________
GGGGGGGG_____________K__________________________________________
GGGGGGGG_____________K_GGGGGGG__________________________________
GGGGGGGGDDD__________KGGGGGGGGGGGGGGGGGGGGGG____________________
GGGGGGGGGDDDDDD____GGGGGGGGGGGGGGGGGGGGGGGGGGGGG________________
GGGGGGGGGGGGGGGDDDDDDDDDDDDDDDDDDDDDDDDGGGGGGGGGGGGG____________
___________GGGGGGGGGGGGGGGGGGGGGGGGGGGDDDGGGGGGGGGGGGGGG________
_____________________K_GGGGGGGGKKKKKKKKKKKRGGGGGGGGGGGGGG_______
_____________________K__GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGDDDDDDD
_____________________K__GGG__GGGGGGGGGGGGGGGGGGGGGGGGGGGG_______
_____________________K_________GD________GGGGGGGGGGGGG__________
_______________________________GD________GGGGR_GD_______________
_______________________________KK_________GGR__GD_______________
______________________________KGGK______________KK______________
______________________________KGGK_____________KGGK_____________
_______________________________KK______________KGGK_____________
________________________________________________KK______________
________________________________________________________________
________________________________________________________________
________________________________________________________________


________________________________________________________________
________________________________________________________________
________________________________________________________________
________________________________________________________________
________________________________________________________________
________________________________________________________________
________________________________________________________________
__GGGGGG________________________________________________________
__GGGGGG________________________________________________________
_GGGGBBB________________________________________________________
_GGGGYYY________________________________________________________
_GGDDGGG________________________________________________________
_GGGDGGG________________________________________________________
GGGGGGGG________________________________________________________
GGGGGGGG_______________GGGGGGG__________________________________
GGGGGGGGDDD___________GGGGGGGGGGGGGGGGGGGGGG____________________
GGGGGGGGGDDDDDD____GGGGGGGGGGGGGGGGGGGGGGGGGGGGG________________
GGGGGGGGGGGGGGGDDDDDDDDDDDDDDDDDDDDDDDDGGGGGGGGGGGGG____________
___________GGGGGGGGGGGGGGGGGGGGGGGGGGGDDDGGGGGGGGGGGGGGG________
_______________________GGGGGGGGKKKKKKKKKKKRGGGGGGGGGGGGGG_______
________________________GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGDDDDDDD
________________________GGG__GGGGGGGGGGGGGGGGGGGGGGGGGGGG_______
_______________________________GD________GGGGGGGGGGGGG__________
_______________________________GD________GGGGR_GD_______________
_______________________________KK_________GGR__GD_______________
______________________________KGGK______________KK______________
______________________________KGGK_____________KGGK_____________
_______________________________KK______________KGGK_____________
________________________________________________KK______________
________________________________________________________________
________________________________________________________________
________________________________________________________________


"#);
        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture],
        }; 
        Drone { bindings, x:-0.3, y:0.4, frame: 0 }

    }
}


