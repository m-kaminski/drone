use miniquad::*;
use crate::drone::*;
use crate::missile::*;
use crate::background::*;

mod drone; 
mod missile;
mod background;
mod shader;
use crate::render::*;
mod render;

struct Stage {
    pipeline: Pipeline,
    drone: Drone,
    background: Background,
    //missile: Missile,
    dynamic_objects: Vec<Box<dyn Render>>,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        #[rustfmt::skip]

        let drone = Drone::new(ctx);
        //let missile = Missile::new(ctx);
        let background = Background::new(ctx);
        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();
        let mut dynamic_objects : Vec<Box<dyn Render>> = Vec::new();
        dynamic_objects.push(Box::new(Missile::new(ctx)));
        BlendState::new(
            Equation::Add,
            BlendFactor::Value(BlendValue::SourceAlpha),
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha)
        );
        let pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
            PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
        );
        Stage { pipeline,drone , background,dynamic_objects}
    }
}

impl EventHandler for Stage {

    fn update(&mut self, _ctx: &mut Context) {

        self.drone.animate(0.1);
        //self.missile.animate(0.1);

        for ob in self.dynamic_objects.iter_mut() {
            //println!("RM");
             &ob.animate(0.1);
         }
    }

    fn draw(&mut self, ctx: &mut Context) {

        ctx.begin_default_pass(Default::default());
        
        let _ = &self.background.render(ctx, &self.pipeline);
        let _ = &self.drone.render(ctx, &self.pipeline);
        //let _ = &self.missile.render(ctx, &self.pipeline);

        for ob in self.dynamic_objects.iter_mut() {
          // println!("RM {}", self.dynamic_objects.len());
          let _ = &ob.render(ctx, &self.pipeline);
        }
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
            miniquad::KeyCode::Space=>self.dynamic_objects.push(Box::new(Missile::new(_ctx))),
            _=>println!("unhandled key"),
            }
          
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        Box::new(Stage::new(&mut ctx))
    });
}
