use raylib::{color::Color, ease, prelude::*};

pub struct Project {
    pub objects: Vec<Object>,
    pub frame_rate: f32,
    pub maximum_frames: i32,
    pub layout: Layout,
    pub selected_object: Option<usize>,
    pub viewport_dimensions: Vector2,
}

pub trait Shape {
    fn draw(&self, d: &mut RaylibTextureMode<'_, RaylibHandle>);
    fn get_transform(&self) -> &ObjectTransform;
    fn set_transform(&mut self, new_transform: ObjectTransform);
    fn get_color(&self) -> &Color;
    fn clone_box(&self) -> Box<dyn Shape>;
}

pub struct Keyframe {
    pub frame: f32,
    pub new_state: ObjectTransform,
    pub easer_type: ease::EaseFn,
}

#[derive(Clone)]
pub struct ObjectTransform {
    pub rotation: f32,
    pub size: Vector2,
    pub position: Vector2,
}

pub struct TextShape {
    pub text: String,
    pub transform: ObjectTransform,
    pub color: Color,
}

pub struct PolygonShape {
    pub points: i32,
    pub transform: ObjectTransform,
    pub color: Color,
}

pub struct Layout {
    pub timeline_frame_width: i32,
    pub timeline_layer_height: i32,
    pub timeline_buttons_height: i32,
    pub timeline_buttons_width: i32,
}

pub struct Object {
    pub shape: Box<dyn Shape>,
    pub keyframes: Option<Vec<Keyframe>>,
    pub start_time: f32,
    pub length: f32,
    pub name: String,
}