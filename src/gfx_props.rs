use gfx;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }

    vertex BallVertex {
        pos: [f32; 2] = "a_Pos",
    }

    pipeline ball_pipe {
        vbuf: gfx::VertexBuffer<BallVertex> = (),
        midpoint: gfx::Global<[f32; 2]> = "BallMidpoint",
        color: gfx::Global<[f32; 3]> = "BallColor",
        radius: gfx::Global<f32> = "BallRadius",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}
