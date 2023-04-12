use miniquad::*;
//#use rand::Rng;
use rand::Rng;


use crate::render::*;


pub struct  Background {
    bindings: Bindings,
    pub x: f32,
}

impl Render for Background {
    fn render(&self, ctx: &mut Context, pipeline: & Pipeline) {       
       ctx.apply_pipeline(pipeline);

       ctx.apply_bindings(&self.bindings);
       // begining and length of index buffer (essentially number of vertices) and number of instances
       ctx.apply_uniforms(&crate::shader::Uniforms {
        offset: (0.0, 0.0),
    });
       ctx.draw(0, 6, 1);

   }
}

impl Background {
    pub fn new(ctx: &mut Context) -> Background {
       
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -1., y: 1. }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  1., y: 1. }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  1., y: -1. }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -1., y: -1. }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let wh = 512;
        let mut pixels  = vec![ 0xFF; wh * wh * 4]; 
        let mut rng = rand::thread_rng();

        for y in 0..wh {
            for x in 0..wh {
                let i = y * 4 * wh + x * 4;
                if rng.gen_range(0..wh) < y/2 {
                    pixels[i] = 0x80;pixels[i+1] = 0x90; pixels[i+2] = 0xFF;
                } else if rng.gen_range(0..wh) > wh - (wh+y)/2 {
                    pixels[i] = 0x40;pixels[i+1] = 0x60; pixels[i+2] = 0xFF;
                } else {
                    pixels[i] = 0x20;pixels[i+1] = 0x30; pixels[i+2] = 0xFF;
                }
            }
        }

        let texture =  Texture::from_data_and_format(
            ctx,
            &pixels,
            TextureParams {
                width: wh as u32,
                height: wh as u32,
                format: TextureFormat::RGBA8,
                wrap: TextureWrap::Clamp,
                filter: FilterMode::Nearest, // use Linear for smooth
            },
        );        
        
        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture],
        }; 
        Background { bindings, x:0. }

    }
}


