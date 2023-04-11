use miniquad::*;
use crate::drone::*;

mod drone; 

use crate::shader::*;
mod shader;
use crate::render::*;
mod render;



struct Stage {
    pipeline: Pipeline,
    drone: Drone
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


        &self.drone.render(ctx, &self.pipeline);

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
            _=>println!("unhandled key"),
            }
          
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        Box::new(Stage::new(&mut ctx))
    });
}
