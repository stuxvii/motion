use raylib::prelude::*;

mod struct_impls;
mod structs;
use crate::structs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut rl, rt) = init()
        .width(800)
        .height(768)
        .title("animation")
        .resizable()
        .build();
    rl.set_target_fps(60);
    rl.set_window_min_size(640, 640);
    let name_field = rl.measure_text(&"W".repeat(16), 10);

    let mut layout = Layout::new(32, 32, 18, 18);
    let mut project = Project::new(vec![], 24., 256, layout, None, Vector2::new(320., 240.));
    let mut current_frame: i32 = 0;
    let mut frame_timer: f32 = 0.;
    let mut timeline_view_offset: i32 = 0;
    let mut frame_each_second: f32 = 1. / project.frame_rate;
    let mut viewport: RenderTexture2D = rl.load_render_texture(&rt, 512, 512)?;
    let mut playing: bool = false;
    let mut swap_request: Option<(usize, usize)> = None;
    let keys = [
        KeyboardKey::KEY_PERIOD,
        KeyboardKey::KEY_COMMA,
        KeyboardKey::KEY_SPACE,
    ];

    while !rl.window_should_close() {
        {
            let mut viewport_surface: RaylibTextureMode<'_, RaylibHandle> =
                rl.begin_texture_mode(&rt, &mut viewport);
            viewport_surface.clear_background(Color::BLACK);

            for o in &project.objects {
                let is_active =
                    current_frame >= o.start_time && current_frame <= o.start_time + o.length;
                if is_active {
                    o.render(&mut viewport_surface, current_frame);
                }
            }

            // Reason we put it here and not below using `d`, is so it keeps sync with the drawn objects and isnt delayed by 1 frame (jussa nitpick tho)
            for key in keys {
                if viewport_surface.is_key_pressed(key)
                    || viewport_surface.is_key_pressed_repeat(key)
                {
                    match key {
                        KeyboardKey::KEY_PERIOD => {
                            current_frame += 1;
                        }
                        KeyboardKey::KEY_COMMA => {
                            if current_frame >= 1 {
                                current_frame -= 1;
                            }
                        }
                        KeyboardKey::KEY_SPACE => playing = !playing,
                        _ => {}
                    }
                }
            }
        }

        let mut d = rl.begin_drawing(&rt);
        if d.get_mouse_wheel_move() > 0. || timeline_view_offset > 0 {
            timeline_view_offset += d.get_mouse_wheel_move() as i32;
        }

        let screen_width = d.get_render_width();
        let screen_height = d.get_render_height();
        d.clear_background(Color::GRAY);

        // show the render output on screen
        d.draw_texture_rec(
            viewport.texture(),
            rrect(
                0,
                0,
                viewport.texture.width as f32,
                -viewport.texture.height as f32,
            ),
            Vector2::new((screen_width as f32) - (viewport.texture.width as f32), 0.),
            Color::WHITE,
        );

        let objects_count = project.objects.len() as i32;
        // draw timeline and objects
        let mut timeline_height =
            screen_height - (project.layout.timeline_layer_height * objects_count);
        timeline_height -= project.layout.timeline_buttons_height;

        for (i, o) in project.objects.iter().enumerate().rev() {
            let mut current_x = project.layout.timeline_frame_width * -timeline_view_offset;
            let mut inner_timeline_height = timeline_height;
            inner_timeline_height += i as i32 * project.layout.timeline_layer_height;
            current_x += name_field;

            // timeline bg
            d.draw_rectangle(
                current_x,
                inner_timeline_height,
                screen_width - current_x,
                project.layout.timeline_layer_height,
                Color::DARKGRAY,
            );

            current_x += o.start_time * project.layout.timeline_frame_width;
            d.draw_rectangle(
                current_x,
                inner_timeline_height,
                (o.length + 1) * project.layout.timeline_frame_width,
                project.layout.timeline_layer_height,
                o.shape.get_color(),
            );

            let opposite_color = Color::new(
                255 - o.shape.get_color().r,
                255 - o.shape.get_color().g,
                255 - o.shape.get_color().b,
                255,
            );

            if let Some(k) = &o.keyframes {
                let mut frame_offset = 0;
                for key in k.iter() {
                    frame_offset += key.frame;
                    let local_x = current_x
                        + (project.layout.timeline_frame_width * frame_offset)
                        + project.layout.timeline_frame_width / 2;
                    d.draw_poly(
                        Vector2::new(
                            local_x as f32,
                            (inner_timeline_height + (project.layout.timeline_layer_height / 2))
                                as f32,
                        ),
                        4,
                        5.,
                        0.,
                        Color::WHITE,
                    );
                    d.draw_poly_lines(
                        Vector2::new(
                            local_x as f32,
                            (inner_timeline_height + (project.layout.timeline_layer_height / 2))
                                as f32,
                        ),
                        4,
                        5.,
                        0.,
                        opposite_color,
                    );
                }
            }

            for i in 0..project.maximum_frames {
                d.draw_rectangle(
                    (name_field) + (i * project.layout.timeline_frame_width) - 1,
                    inner_timeline_height,
                    1,
                    project.layout.timeline_layer_height,
                    Color::GRAY,
                );
            }

            d.draw_text(
                &o.name,
                current_x,
                inner_timeline_height + (project.layout.timeline_layer_height / 3),
                10,
                opposite_color,
            );

            d.draw_rectangle_lines(
                current_x,
                (timeline_height + 1) + (project.layout.timeline_layer_height * i as i32),
                (o.length + 1) * project.layout.timeline_frame_width,
                project.layout.timeline_layer_height - 1,
                opposite_color,
            );
        }

        d.draw_text(
            format!(
                "frame: {} | sec: {:.3}",
                current_frame + 1,
                current_frame as f32 / (1. / frame_each_second)
            )
            .as_str(),
            0,
            timeline_height - 10,
            10,
            Color::WHITE,
        );
        if d.is_key_down(KeyboardKey::KEY_LEFT_ALT) {
            for i in 0 - current_frame..project.maximum_frames - current_frame {
                let counter = format!("{i}");
                let x = ((project.layout.timeline_frame_width * -timeline_view_offset)
                    + name_field)
                    + (i * project.layout.timeline_frame_width)
                    + current_frame * project.layout.timeline_frame_width
                    + ((project.layout.timeline_frame_width - d.measure_text(&counter, 10)) / 2);
                d.draw_text(&counter, x, timeline_height - 10, 10, Color::WHITE);
            }
        } else {
            for i in 0..project.maximum_frames {
                let counter = format!("{i}");
                if timeline_view_offset >= i {
                    continue;
                }
                let x = ((project.layout.timeline_frame_width * -timeline_view_offset)
                    + name_field)
                    + (i * project.layout.timeline_frame_width)
                    - project.layout.timeline_frame_width
                    + ((project.layout.timeline_frame_width - d.measure_text(&counter, 10)) / 2);
                d.draw_text(&counter, x, timeline_height - 10, 10, Color::WHITE);
            }
        }

        // scrubber
        let scrubber_x = name_field
            + ((current_frame - timeline_view_offset) * project.layout.timeline_frame_width);
        if timeline_view_offset > current_frame {
            d.draw_poly(
                Vector2::new(name_field as f32 - 6., timeline_height as f32 - 6.),
                3,
                6.,
                180.,
                Color::new(0, 0, 255, 255),
            );
        }
        if scrubber_x >= screen_width {
            d.draw_poly(
                Vector2::new(screen_width as f32 - 6., timeline_height as f32 - 6.),
                3,
                6.,
                0.,
                Color::new(0, 0, 255, 255),
            );
        }

        if playing {
            frame_timer += d.get_frame_time();
            while frame_timer >= frame_each_second {
                current_frame += 1;
                frame_timer -= frame_each_second
            }
        }

        if timeline_view_offset <= current_frame {
            d.draw_rectangle(
                scrubber_x,
                timeline_height,
                project.layout.timeline_frame_width,
                objects_count * project.layout.timeline_layer_height,
                Color::new(0, 0, 255, 64),
            );
        }

        let icon_arrow_up = d.gui_icon_text(GuiIconName::ICON_ARROW_UP_FILL, "");
        let icon_arrow_down = d.gui_icon_text(GuiIconName::ICON_ARROW_DOWN_FILL, "");
        // object labels
        for (i, o) in project.objects.iter().enumerate().rev() {
            let mut y = i as i32 * project.layout.timeline_layer_height;
            y += timeline_height;

            let rect = rrect(0, y, name_field, project.layout.timeline_layer_height);

            // label
            if let Some(ref o) = project.selected_object {
                if i == *o {
                    d.draw_rectangle_rec(rect, Color::GRAY);
                } else {
                    d.draw_rectangle_rec(rect, Color::BLACK);
                }
            } else {
                d.draw_rectangle_rec(rect, Color::BLACK);
            }

            d.draw_line(
                0,
                y + project.layout.timeline_layer_height,
                name_field,
                y + project.layout.timeline_layer_height,
                Color::WHITE,
            );
            d.draw_text(
                format!("{i}. {}", &o.name).as_str(),
                1,
                y + (project.layout.timeline_layer_height / 3),
                10,
                Color::WHITE,
            );

            let rect = rrect(0, y, name_field - 16, project.layout.timeline_layer_height);
            if rect.check_collision_point_rec(d.get_mouse_position()) {
                if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    project.selected_object = Some(i);
                }
            }

            if ((i as i32) != 0)
                && d.gui_button(
                    rrect(
                        name_field - 16,
                        y,
                        16,
                        project.layout.timeline_layer_height / 2,
                    ),
                    &icon_arrow_up,
                )
            {
                if i > 0 {
                    swap_request = Some((i, i - 1));
                }
            }

            if (i as i32) != objects_count - 1
                && d.gui_button(
                    rrect(
                        name_field - 16,
                        y + project.layout.timeline_layer_height / 2,
                        16,
                        project.layout.timeline_layer_height / 2,
                    ),
                    &icon_arrow_down,
                )
            {
                if i < (objects_count - 1).try_into().unwrap() {
                    swap_request = Some((i + 1, i));
                }
            }
        }

        if let Some((a, b)) = swap_request {
            if let Some(ref o) = project.selected_object {
                if *o == a {
                    project.selected_object = Some(b);
                } else if *o == b {
                    project.selected_object = Some(a);
                }
            }
            project.objects.swap(a, b);
            swap_request = None;
        }

        if let Some(ref o) = project.selected_object {
            let temp_obj = project.objects.get(*o);
            if let Some(tmp_obj_info) = temp_obj {
                let text_name = format!("{o}. {}", tmp_obj_info.name);
                d.draw_text(&text_name, 1, 1, 10, Color::WHITE);
            }
        }

        let tlbh = timeline_height + (project.layout.timeline_layer_height * objects_count);
        if d.gui_button(
            rrect(
                0,
                tlbh,
                project.layout.timeline_buttons_width,
                project.layout.timeline_buttons_height,
            ),
            "-",
        ) {
            if project.layout.timeline_frame_width >= 19 {
                project.layout.timeline_frame_width -= 1;
            }
        }
        if d.gui_button(
            rrect(
                project.layout.timeline_buttons_width,
                tlbh,
                project.layout.timeline_buttons_width,
                project.layout.timeline_buttons_height,
            ),
            "+",
        ) {
            project.layout.timeline_frame_width += 1;
        }

        d.draw_text(
            format!("Zoom: {}", project.layout.timeline_frame_width).as_str(),
            project.layout.timeline_buttons_width * 2,
            tlbh + (project.layout.timeline_buttons_height / 2) - 4,
            10,
            Color::WHITE,
        );

        if d.gui_button(
            rrect(
                screen_width - 16,
                tlbh,
                project.layout.timeline_buttons_width,
                project.layout.timeline_buttons_height,
            ),
            "#30#",
        ) {
            let text = TextShape::new(
                String::from("Lorem Ipsum"),
                ObjectTransform::new(0., Vector2::new(20., 0.), Vector2::new(200., 384.)),
                Color::RED,
            );
            let temp_object =
                Object::new(Box::new(text), None, 0, 64, String::from("Lorem Text Demo"));

            project.objects.push(temp_object);
        }
    }
    Ok(())
}
