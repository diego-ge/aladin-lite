use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use super::color::ColorRGB;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MOC {
    uuid: String,
    opacity: f32,
    line_width: f32,
    is_showing: bool,
    color: ColorRGB,
    adaptative_display: bool,
}
use std::convert::TryInto;
use crate::Abort;
#[wasm_bindgen]
impl MOC {
    #[wasm_bindgen(constructor)]
    pub fn new(uuid: String, opacity: f32, line_width: f32, is_showing: bool, color: JsValue, adaptative_display: bool) -> Self {
        let color = color.try_into().unwrap_abort();
        Self {
            uuid,
            opacity,
            line_width,
            color,
            is_showing,
            adaptative_display
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_is_showing(&mut self, is_showing: bool) {
        self.is_showing = is_showing;
    }
}

impl MOC {
    pub fn get_uuid(&self) -> &String {
        &self.uuid
    }

    pub fn get_color(&self) -> &ColorRGB {
        &self.color
    }

    pub fn get_opacity(&self) -> f32 {
        self.opacity
    }
    
    pub fn get_line_width(&self) -> f32 {
        self.line_width
    }

    pub fn is_showing(&self) -> bool {
        self.is_showing
    }

    pub fn is_adaptative_display(&self) -> bool {
        self.adaptative_display
    }
}

impl Default for MOC {
    fn default() -> Self {
        Self {
            uuid: String::from("moc"),
            opacity: 1.0,
            line_width: 1.0,
            is_showing: true,
            color: ColorRGB {r: 1.0, g: 0.0, b: 0.0},
            adaptative_display: true,
        }
    }
}