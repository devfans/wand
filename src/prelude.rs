pub use crate::core::{Application, State, FpsCounter};
pub use crate::scene::Scene;
pub use crate::container::Container;
pub use crate::section::Section;
pub use crate::content::Content;
pub use crate::span::{Span, SpanTrait, TextSpan, WorldSpan};

pub mod renderer {
    pub use web_sys::{ 
        window,
        Window, Document,
        HtmlCanvasElement,
        WebGlProgram, WebGlShader, WebGlRenderingContext as ContextGL,
        CanvasRenderingContext2d as Context2D,
        TextMetrics
    };

    pub use crate::core::RendererContext;
}

pub mod js {
    pub use wasm_bindgen::{self, JsCast};
    pub use wasm_bindgen::prelude::*;
}
