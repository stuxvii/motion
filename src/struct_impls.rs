use raylib::{RaylibHandle, color::Color, ease::*, math::Vector2, prelude::{RaylibDraw, RaylibTextureMode}};

use crate::{Keyframe, Layout, Object, ObjectTransform, PolygonShape, Shape, TextShape};

impl Shape for TextShape {
    fn get_color(&self) -> &raylib::prelude::Color {
        &self.color
    }
    fn get_transform(&self) -> &ObjectTransform {
        &self.transform
    }
    fn clone_box(&self) -> Box<dyn Shape + 'static> {
        Box::new(TextShape::new(
            self.text.clone(),
            ObjectTransform::new(
                self.transform.rotation,
                self.transform.size,
                self.transform.position,
            ),
            self.color,
        ))
    }

    fn draw(&self, d: &mut RaylibTextureMode<'_, RaylibHandle>) {
        d.draw_text_pro(
            d.get_font_default(),
            &self.text,
            self.transform.position,
            Vector2::new(0., 0.),
            self.transform.rotation,
            self.transform.size.length(),
            1.,
            &self.color,
        );
    }

    fn set_transform(&mut self, new_transform: ObjectTransform) {
        self.transform = new_transform;
    }
}

impl Shape for PolygonShape {
    fn get_color(&self) -> &raylib::prelude::Color {
        &self.color
    }
    fn get_transform(&self) -> &ObjectTransform {
        &self.transform
    }
    fn clone_box(&self) -> Box<dyn Shape + 'static> {
        Box::new(PolygonShape::new(
            5,
            ObjectTransform::new(0., Vector2::new(100., 50.), Vector2::new(50., 100.)),
            Color::GREEN,
        ))
    }
    fn draw(&self, d: &mut RaylibTextureMode<'_, RaylibHandle>) {
        d.draw_poly(
            self.transform.position,
            self.points,
            self.transform.size.length(),
            self.transform.rotation,
            self.color,
        );
    }

    fn set_transform(&mut self, new_transform: ObjectTransform) {
        self.transform = new_transform;
    }
}

impl TextShape {
    pub fn new(text: String, transform: ObjectTransform, color: Color) -> Self {
        TextShape {
            text,
            transform,
            color,
        }
    }
}

impl PolygonShape {
    pub fn new(points: i32, transform: ObjectTransform, color: Color) -> Self {
        PolygonShape {
            points,
            transform,
            color,
        }
    }
}

impl ObjectTransform {
    pub fn new(rotation: f32, size: Vector2, position: Vector2) -> ObjectTransform {
        ObjectTransform {
            rotation,
            size,
            position,
        }
    }
}

impl Keyframe {
    pub fn new(frame: i32, new_state: ObjectTransform, easer_type: EaseFn) -> Self {
        Keyframe { frame, new_state, easer_type }
    }

    pub fn step(&self, c_s: &ObjectTransform, c_f_r: i32) -> ObjectTransform {
        let progress = (c_f_r as f32 - 1.0) / (self.frame as f32);
        let progress = progress.clamp(0.0, 1.0);
        
        let r = Tween::new(self.easer_type, c_s.rotation, self.new_state.rotation, 1.0).apply(progress);

        let p_x =
            Tween::new(self.easer_type, c_s.position.x, self.new_state.position.x, 1.0).apply(progress);
        let p_y =
            Tween::new(self.easer_type, c_s.position.y, self.new_state.position.y, 1.0).apply(progress);

        let s_x = Tween::new(self.easer_type, c_s.size.x, self.new_state.size.x, 1.0).apply(progress);
        let s_y = Tween::new(self.easer_type, c_s.size.y, self.new_state.size.y, 1.0).apply(progress);

        ObjectTransform {
            rotation: r,
            position: Vector2::new(p_x, p_y),
            size: Vector2::new(s_x, s_y),
        }
    }
}

impl Object {
    pub fn new(
        shape: Box<dyn Shape>,
        keyframes: Option<Vec<Keyframe>>,
        start_time: i32,
        length: i32,
        name: String,
    ) -> Self {
        Object {
            shape,
            keyframes,
            start_time,
            length,
            name,
        }
    }

    pub fn get_transform_at_frame(&self, current_frame: i32) -> ObjectTransform {
        let mut current_transform = self.shape.get_transform().clone();

        if current_frame < self.start_time {
            return current_transform.clone();
        }

        if let Some(keyframes) = &self.keyframes {
            let mut frames_passed = 0;
            let active_frame = current_frame - self.start_time;

            for keyframe in keyframes {
                let kf_duration = keyframe.frame;

                if active_frame >= frames_passed + kf_duration {
                    current_transform = keyframe.new_state.clone();
                    frames_passed += kf_duration;
                } else if active_frame > frames_passed {
                    let frames_into_kf = active_frame - frames_passed;
                    let tmp_transform = keyframe.step(&current_transform, frames_into_kf);
                    current_transform = tmp_transform;
                    break;
                } else {
                    break; 
                }
            }
        }

        current_transform.clone()
    }

    pub fn render(&self, d: &mut RaylibTextureMode<'_, RaylibHandle>, current_frame: i32) {
        let current_transform = self.get_transform_at_frame(current_frame);
        let mut temp_shape = self.shape.clone_box();
        temp_shape.set_transform(current_transform);
        temp_shape.draw(d);
    }
}

impl Layout {
    pub fn new(tl_f_w: i32, tl_l_h: i32, tl_b_h: i32, tl_b_w: i32) -> Layout {
        Layout {
            timeline_frame_width: tl_f_w.clamp(12, 256),
            timeline_layer_height: tl_l_h.clamp(12, 256),
            timeline_buttons_height: tl_b_h.clamp(11, 32),
            timeline_buttons_width: tl_b_w.clamp(11, 32)
        }
    }
}