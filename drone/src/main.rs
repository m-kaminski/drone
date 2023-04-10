use miniquad::*;

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

struct Stage {
    pipeline: Pipeline,
    drone: Drone
}


/// TRAIT RENDER

trait  Render {
     fn render(&self);
}

struct Drone {
    bindings: Bindings,
    x: f32,
    y: f32,
}

impl Render for Drone {
     fn render(&self) {
        
        
    }
}

impl Drone {
    pub fn new(ctx: &mut Context) -> Drone {
       
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -0.5, y: -0.5 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  0.5, y: -0.5 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  0.5, y:  0.5 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -0.5, y:  0.5 }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);
        //TextureParams Nearest
        let pixels: [u8; 4 * 4 * 4] = [ 
            // 4 rows, 4 pixels each
            0x00, 0xFF, 0xFF, 0xFF, // bottom left corner
            0xFF, 0x00, 0x00, 0xFF,  
            0xFF, 0xFF, 0xFF, 0xFF,   0xFF, 0xFF, 0x00, 0xFF, // bottom right corner

            0xFF, 0x00, 0x00, 0xFF,   0xFF, 0xFF, 0xFF, 0xFF, 
            0xFF, 0x00, 0x00, 0xFF,   0xFF, 0xFF, 0xFF, 0xFF, 
            
            0xFF, 0xFF, 0xFF, 0xFF,   0xFF, 0x00, 0x00, 0x00, 
            0xFF, 0xFF, 0xFF, 0xFF,   0xFF, 0x00, 0x00, 0xFF, 
            
            0x00, 0xFF, 0x00, 0xFF, // top left corner
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0x00, 0x00, 0xFF,   0x00, 0x00, 0x00, 0xFF, // top right corner
        ];
        //let texture = Texture::from_rgba8(ctx, 4, 4, &pixels);

        let texture = Texture::from_data_and_format(
            ctx,
            &pixels,
            TextureParams {
                width: 4,
                height: 4,
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
        Drone { bindings, x:-0.3, y:0.4 }

    }
}



impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        #[rustfmt::skip]

        let drone = Drone::new(ctx);
        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );
        Stage { pipeline,drone }
    }
}


impl EventHandler for Stage {

    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {

        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);


        ctx.apply_bindings(&self.drone.bindings);
        // begining and length of index buffer (essentially number of vertices) and number of instances
        ctx.draw(0, 6, 1);

        ctx.apply_uniforms(&shader::Uniforms {
            offset: (*&self.drone.x, *&self.drone.y),
        });



        ctx.end_render_pass();

        ctx.commit_frame();
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool
    ) { 
        match _keycode{
            miniquad::KeyCode::Left=>self.drone.x -= 0.1,
            miniquad::KeyCode::Right=>self.drone.x += 0.1,
            miniquad::KeyCode::Up=>self.drone.y += 0.1,
            miniquad::KeyCode::Down=>self.drone.y -= 0.1,
            /*
            miniquad::KeyCode::A=>self.x -= 0.1,
            miniquad::KeyCode::D=>self.x += 0.1,
            miniquad::KeyCode::W=>self.y += 0.1,
            miniquad::KeyCode::S=>self.y -= 0.1,*/
            _=>println!("unhandled key"),
            }
          
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        Box::new(Stage::new(&mut ctx))
    });
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;
    uniform vec2 offset;
    varying lowp vec2 texcoord;
    void main() {
        gl_Position = vec4(pos + offset, 0, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;
    uniform sampler2D tex;
    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
    }
}
