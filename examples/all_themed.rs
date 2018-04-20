#[cfg(all(feature="winit", feature="glium"))] #[macro_use] extern crate conrod;
#[cfg(all(feature="winit", feature="glium"))] mod support;
extern crate find_folder;

fn main() {
    feature::main();
}

#[cfg(all(feature="winit", feature="glium"))]
mod feature {
    extern crate find_folder;
    use conrod::{self, widget, Labelable, Positionable, Sizeable, Widget, Scalar};
    use conrod::backend::glium::glium;
    use conrod::backend::glium::glium::Surface;
    use conrod::arc_theme::{ArcTheme, DangerButton};
    use support;

    widget_ids!(struct Ids {
        canvas,
        button,
        toggle,
        text_box,
        drop_down_list,
        warn_button,
        number_dialer,
        slider,
    });

    #[derive(Debug, Default)]
    struct State {
        button_count: usize,
        toggle_state: bool,
        text_box_text: String,
        drop_down_list_idx: usize,
        number_dialer_value: f64,
    }

    pub fn main() {
        const AMOUNT       : Scalar =  10.0;
        const ITEM_WIDTH   : Scalar = 200.0;
        const ITEM_HEIGHT  : Scalar =  35.0;
        const ITEM_PADDING : Scalar =  10.0;

        const WIDTH : Scalar = ITEM_WIDTH + 2.0 * ITEM_PADDING;
        const HEIGHT: Scalar = ITEM_PADDING + AMOUNT * (ITEM_HEIGHT + ITEM_PADDING);

        // Build the window.
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title("All themed widgets")
            .with_min_dimensions(WIDTH as u32, HEIGHT as u32)
            .with_max_dimensions(WIDTH as u32, HEIGHT as u32)
            .with_dimensions    (WIDTH as u32, HEIGHT as u32);
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([WIDTH, HEIGHT])
                                        .theme(ArcTheme::light()
                                                .accent(conrod::color::DARK_PURPLE)
                                                .unwrap())
                                        .build();

        // Generate the widget identifiers.
        let ids = Ids::new(ui.widget_id_generator());

        // Add a `Font` to the `Ui`'s `font::Map` from file.
        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();

        // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
        // for drawing to the glium `Surface`.
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        // The image map describing each of our widget->image mappings (in our case, none).
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        let mut state = State::default();

        // Poll events from the window.
        let mut event_loop = support::EventLoop::new();
        'main: loop {

            // Handle all events.
            for event in event_loop.next(&mut events_loop) {

                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                    ui.handle_event(event);
                    event_loop.needs_update();
                }

                match event {
                    glium::glutin::Event::WindowEvent { event, .. } => match event {
                        // Break from the loop upon `Escape`.
                        glium::glutin::WindowEvent::Closed |
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => break 'main,
                        _ => (),
                    },
                    _ => (),
                }
            }

            // Instantiate all widgets in the GUI.
            {
                let ui = &mut ui.set_widgets();

                // Create a background canvas upon which we'll place the button.
                widget::Canvas::new()
                    .pad(ITEM_PADDING)
                    .scroll_kids()
                    .set(ids.canvas, ui);

                for _click in widget::Button::new()
                    .parent(ids.canvas)
                    .top_left_of(ids.canvas)
                    .w_h(ITEM_WIDTH, ITEM_HEIGHT)
                    .label(&state.button_count.to_string())
                    .set(ids.button, ui)
                {
                    state.button_count += 1;
                }

                // Draw the button and increment `count` if pressed.
                for event in widget::Toggle::new(state.toggle_state)
                    .down_from(ids.button, ITEM_PADDING)
                    .w_h(ITEM_WIDTH, ITEM_HEIGHT)
                    .label(if state.toggle_state { "On" } else { "Off" })
                    .set(ids.toggle, ui)
                {
                    state.toggle_state = event;
                }

                for event in widget::TextBox::new(&state.text_box_text)
                    .parent(ids.canvas)
                    .down_from(ids.toggle, ITEM_PADDING)
                    .w_h(ITEM_WIDTH, ITEM_HEIGHT)
                    .set(ids.text_box, ui)
                {
                    use self::widget::text_box::Event::*;
                    match event {
                        Update(new_text) => state.text_box_text = new_text,
                        Enter => (),
                    }
                }

                for selected_idx in widget::DropDownList::new(
                        &["First", "Second", "Third", "Fourth", "Fifth"],
                        Some(state.drop_down_list_idx)
                    )
                    .parent(ids.canvas)
                    .down_from(ids.text_box, ITEM_PADDING)
                    .w_h(ITEM_WIDTH, ITEM_HEIGHT)
                    .max_visible_items(3)
                    .scrollbar_next_to()
                    .set(ids.drop_down_list, ui)
                {
                    state.drop_down_list_idx = selected_idx
                }

                for _click in widget::Button::new()
                    .kind(DangerButton)
                    .parent(ids.canvas)
                    .down_from(ids.drop_down_list, ITEM_PADDING)
                    .w_h(ITEM_WIDTH, ITEM_HEIGHT)
                    .label(&state.button_count.to_string())
                    .set(ids.warn_button, ui)
                {
                    state.button_count += 1;
                }

                for new_number in widget::NumberDialer::new(state.number_dialer_value, 0.0, 500.0, 0)
                    .parent(ids.canvas)
                    .down_from(ids.warn_button, ITEM_PADDING)
                    .w_h(ITEM_WIDTH, ITEM_HEIGHT)
                    .label("Power")
                    .set(ids.number_dialer, ui)
                {
                    state.number_dialer_value = new_number;
                }

                for new_number in widget::Slider::new(state.number_dialer_value, 0.0, 500.0)
                    .parent(ids.canvas)
                    .down_from(ids.number_dialer, ITEM_PADDING)
                    .w_h(ITEM_WIDTH, ITEM_HEIGHT)
                    .label("Power")
                    .set(ids.slider, ui)
                {
                    state.number_dialer_value = new_number;
                }

            }

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }
}

#[cfg(not(all(feature="winit", feature="glium")))]
mod feature {
    pub fn main() {
        println!("This example requires the `winit` and `glium` features. \
                 Try running `cargo run --release --features=\"winit glium\" --example <example_name>`");
    }
}
