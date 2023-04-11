use miniquad::*;


pub trait  Render {
    fn render(&self, ctx: &mut Context, pipeline: & Pipeline);
}



pub fn gen_texture(ctx: &mut Context, w: usize, h: usize, tex: &str) -> Texture {

    println!("{}",tex);
        //TextureParams Nearest
        //let  AS : usize = ;
        let mut pixels  = vec![ 0xFF; w*h*4]; 

        let mut i = 0;
        for c in tex.chars() { 
            match c {
                'R' =>{ pixels[i] = 0xff;pixels[i+1] = 0x00;pixels[i+2] = 0x00; i+=4},
                'B' =>{ pixels[i] = 0xFF;pixels[i+1] = 0x00;pixels[i+2] = 0xFF; i+=4},
                'G' =>{ pixels[i] = 0x88;pixels[i+1] = 0x88;pixels[i+2] = 0x88; i+=4},
                '_' =>{ pixels[i+3] = 0x00; i+=4},
                _=>println!("unhandled key"),
                }
        }
        
        return Texture::from_data_and_format(
            ctx,
            &pixels,
            TextureParams {
                width: w as u32,
                height: h as u32,
                format: TextureFormat::RGBA8,
                wrap: TextureWrap::Clamp,
                filter: FilterMode::Nearest, // use Linear for smooth
            },
        );
}
