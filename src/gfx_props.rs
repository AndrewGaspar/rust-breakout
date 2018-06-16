use gfx;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        awesome: gfx::TextureSampler<[f32; 4]> = "t_Awesome",
        switch: gfx::Global<i32> = "i_Switch",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}
