use raylib::{
    ease::{EaseFn, Tween},
    prelude::*,
};
use std::vec;

struct Keyframe {
    frame: i32,
    new_state: Transform,
}

impl Keyframe {
    fn new(frame: i32, new_state: Transform) -> Self {
        Keyframe { frame, new_state }
    }

    fn step(&self, c_s: &Transform, c_f_r: i32, easer_type: EaseFn) -> Transform {
        let progress = (c_f_r as f32 - 1.0) / (self.frame as f32);

        let r = Tween::new(easer_type, c_s.rotation, self.new_state.rotation, 1.0).apply(progress);

        let p_x =
            Tween::new(easer_type, c_s.position.x, self.new_state.position.x, 1.0).apply(progress);
        let p_y =
            Tween::new(easer_type, c_s.position.y, self.new_state.position.y, 1.0).apply(progress);

        let s_x = Tween::new(easer_type, c_s.size.x, self.new_state.size.x, 1.0).apply(progress);
        let s_y = Tween::new(easer_type, c_s.size.y, self.new_state.size.y, 1.0).apply(progress);

        Transform {
            rotation: r,
            position: Vector2::new(p_x, p_y),
            size: Vector2::new(s_x, s_y),
        }
    }
}

struct Transform {
    rotation: f32,
    size: Vector2,
    position: Vector2,
}

impl Transform {
    fn new(rotation: f32, size: Vector2, position: Vector2) -> Transform {
        Transform {
            rotation,
            size,
            position,
        }
    }
}

trait Shape {
    fn draw(&self, d: &mut RaylibTextureMode<'_, RaylibHandle>);
    fn get_transform(&self) -> &Transform;
    fn set_transform(&mut self, new_transform: Transform);
    fn get_color(&self) -> &Color;
    fn clone_box(&self) -> Box<dyn Shape>;
}

struct TextShape {
    text: String,
    transform: Transform,
    color: Color,
}

impl TextShape {
    fn new(text: String, transform: Transform, color: Color) -> Self {
        TextShape {
            text,
            transform,
            color,
        }
    }
}

struct PolygonShape {
    points: i32,
    transform: Transform,
    color: Color,
}

impl PolygonShape {
    fn new(points: i32, transform: Transform, color: Color) -> Self {
        PolygonShape {
            points,
            transform,
            color,
        }
    }
}

impl Shape for TextShape {
    fn get_color(&self) -> &raylib::prelude::Color {
        &self.color
    }
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
    fn clone_box(&self) -> Box<dyn Shape + 'static> {
        Box::new(TextShape::new(
            self.text.clone(),
            Transform::new(
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

    fn set_transform(&mut self, new_transform: Transform) {
        self.transform = new_transform;
    }
}

impl Shape for PolygonShape {
    fn get_color(&self) -> &raylib::prelude::Color {
        &self.color
    }
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
    fn clone_box(&self) -> Box<dyn Shape + 'static> {
        Box::new(PolygonShape::new(
            5,
            Transform::new(0., Vector2::new(100., 50.), Vector2::new(50., 100.)),
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

    fn set_transform(&mut self, new_transform: Transform) {
        self.transform = new_transform;
    }
}

struct Object {
    shape: Box<dyn Shape>,
    keyframes: Option<Vec<Keyframe>>,
    start_time: i32,
    length: i32,
    name: String,
}
impl Object {
    fn new(
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

    fn render(&self, d: &mut RaylibTextureMode<'_, RaylibHandle>) {
        self.shape.draw(d);
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut rl, rt) = init()
        .width(800)
        .height(768)
        .title("animation")
        .resizable()
        .build();
    rl.set_target_fps(30);
    let name_field = rl.measure_text(&"W".repeat(20), 10);

    let circular_polygon = PolygonShape::new(
        32,
        Transform::new(0., Vector2::new(100., 100.), Vector2::new(150., 150.)),
        Color::WHEAT,
    );

    let coolio_trianglio = PolygonShape::new(
        3,
        Transform::new(0., Vector2::new(100., 100.), Vector2::new(75., 75.)),
        Color::RED,
    );

    let pentagonne = PolygonShape::new(
        5,
        Transform::new(0., Vector2::new(100., 50.), Vector2::new(50., 100.)),
        Color::GREEN,
    );

    let text = TextShape::new(
        String::from("hi this is my cool program for shit"),
        Transform::new(0., Vector2::new(20., 0.), Vector2::new(100., 384.)),
        Color::WHITE,
    );

    let object = Object::new(
        Box::new(circular_polygon),
        None,
        4,
        32,
        String::from("Cirlce"),
    );
    let object_2 = Object::new(
        Box::new(coolio_trianglio),
        None,
        16,
        32,
        String::from("Trianglio"),
    );
    let keyframe_3 = Keyframe::new(
        30,
        Transform::new(70., Vector2::new(20., 40.), Vector2::new(400., 384.)),
    );
    let object_3 = Object::new(
        Box::new(pentagonne),
        Some(vec![keyframe_3]),
        8,
        48,
        String::from("Pentagonne"),
    );
    let object_4 = Object::new(Box::new(text), None, 1, 64, String::from("Text Demo"));

    // HEY DINGUS PAY ATTENTION RIGHT NOW.
    // THE FOLLOWING VARIABLE IS A LIST CONTAINING THE OBJECTS THAT WILL BE DRAWN ON SCREEN
    // THE ORDER IS BACK TO FRONT, FIRST ITEM GETS DRAWN FURTHER BACK, LAST ITEM GETS DRAWN CLOSER UP
    let mut objects: Vec<Object> = vec![];
    objects.push(object);
    objects.push(object_2);
    objects.push(object_3);
    objects.push(object_4);
    let mut current_frame: i32 = 0;
    let mut timeline_view_offset: i32 = 0;
    let pixels_per_frame: i32 = 16;
    let mut viewport: RenderTexture2D = rl.load_render_texture(&rt, 512, 512)?;
    let mut playing: bool = false;

    let keys = [
        KeyboardKey::KEY_PERIOD,
        KeyboardKey::KEY_COMMA,
        KeyboardKey::KEY_LEFT,
        KeyboardKey::KEY_RIGHT,
        KeyboardKey::KEY_SPACE,
    ];

    while !rl.window_should_close() {
        // Object rendering
        {
            let mut viewport_surface: RaylibTextureMode<'_, RaylibHandle> =
                rl.begin_texture_mode(&rt, &mut viewport);
            viewport_surface.clear_background(Color::BLACK);

            // Reason we put it here and not below using `d`, is so it keeps sync with the drawn objects and isnt delayed by 1 frame (jussa nitpick tho)
            for key in keys {
                if viewport_surface.is_key_pressed(key)
                    || viewport_surface.is_key_pressed_repeat(key)
                {
                    match key {
                        KeyboardKey::KEY_PERIOD => current_frame += 1,
                        KeyboardKey::KEY_COMMA => {
                            if current_frame >= 1 {
                                current_frame -= 1
                            }
                        }
                        KeyboardKey::KEY_LEFT => timeline_view_offset += 1,
                        KeyboardKey::KEY_RIGHT => {
                            if timeline_view_offset >= 1 {
                                timeline_view_offset -= 1
                            }
                        }
                        KeyboardKey::KEY_SPACE => playing = !playing,
                        _ => {}
                    }
                }
            }

            for o in &objects {
                if o.start_time >= current_frame + 1 {
                    continue;
                }
                if current_frame - 1 >= o.start_time + o.length {
                    continue;
                }

                if let Some(keyframes) = &o.keyframes {
                    for k in keyframes {
                        let mut obj_shape: Box<dyn Shape> = o.shape.clone_box();
                        let new_transform: Transform = k.step(
                            o.shape.get_transform(),
                            current_frame - o.start_time,
                            ease::quad_in_out,
                        );
                        obj_shape.set_transform(new_transform);
                        let obj_for_display =
                            Object::new(obj_shape, None, o.start_time, o.length, "".to_string());
                        obj_for_display.render(&mut viewport_surface);
                    }
                } else {
                    o.render(&mut viewport_surface);
                }
            }
        }

        if playing {
            current_frame += 1;
        }

        let mut d = rl.begin_drawing(&rt);

        let render_width = d.get_render_width();
        d.clear_background(Color::GRAY);

        // show the render output on screen
        d.draw_texture_rec(
            viewport.texture(),
            rrect(0, 0, viewport.texture.width, -viewport.texture.height),
            Vector2::new((render_width - viewport.texture.width) as f32 - 0., 0.),
            Color::WHITE,
        );

        // draw timeline and objects

        for i in 0..72 {
            let counter = format!("{i}");
            if timeline_view_offset >= i {
                continue;
            }
            let x = ((pixels_per_frame * -timeline_view_offset) + name_field)
                + (i * pixels_per_frame)
                - pixels_per_frame;
            d.draw_text(&counter, x, viewport.texture.height + 24, 10, Color::WHITE);
        }

        for (i, o) in objects.iter().enumerate() {
            let mut current_x = pixels_per_frame * -timeline_view_offset;
            let mut y = i as i32 * 12;
            y += viewport.texture.height + 32;
            current_x += name_field + 0;

            // timeline bg
            d.draw_rectangle(current_x, y, render_width - current_x, 12, Color::DARKGRAY);

            current_x += o.start_time * pixels_per_frame;
            d.draw_rectangle(
                current_x,
                y,
                (o.length + 1) * pixels_per_frame,
                12,
                o.shape.get_color(),
            );
            
            let opposite_color = Color::new(
                255 - o.shape.get_color().r,
                255 - o.shape.get_color().g,
                255 - o.shape.get_color().b,
                255,
            );

            // you get 64 frames for now
            for i in 0..128 {
                d.draw_rectangle(
                    (name_field + 0) + (i * pixels_per_frame) - 1,
                    y,
                    1,
                    12,
                    Color::GRAY,
                );
                if o.start_time <= i && i <= o.start_time + o.length {
                    let counter = format!("{}", i-o.start_time);
                    d.draw_text(&counter, current_x+(pixels_per_frame*(i-o.start_time)), y+2, 10, opposite_color);
                }
            }

            d.draw_rectangle_lines(
                current_x,
                y + 1,
                (o.length + 1) * pixels_per_frame,
                11,
                opposite_color,
            );
        }

        // scrubber
        let scrubber_x =
            name_field + ((current_frame - timeline_view_offset) * pixels_per_frame) + 0. as i32;
        d.draw_rectangle(
            scrubber_x,
            viewport.texture.height + 32,
            pixels_per_frame,
            objects.len() as i32 * 12,
            Color::new(0, 0, 255, 64),
        );

        d.draw_line(
            scrubber_x,
            viewport.texture.height + 32,
            scrubber_x,
            viewport.texture.height + (objects.len() as i32 * 20),
            Color::BLUE,
        );

        // object labels
        for (i, o) in objects.iter().enumerate() {
            let mut y = i as i32 * 12;
            y += viewport.texture.height + 32;

            // label
            d.draw_rectangle(0. as i32, y, name_field, 12, Color::BLACK);
            d.draw_text(&o.name.as_str(), 0. as i32 + 1, y + 1, 10, Color::WHITE);
        }

        d.draw_text(
            format!("Current frame: {}", current_frame + 1).as_str(),
            (render_width - viewport.texture.width) - 0. as i32,
            viewport.texture.height + 0. as i32 + 1,
            20,
            Color::WHITE,
        );
    }
    Ok(())
}
