#[cfg(all(feature="winit", feature="glium"))] #[macro_use] extern crate conrod;
#[cfg(all(feature="winit", feature="glium"))] mod support;
extern crate find_folder;
#[macro_use]
extern crate yoga;

fn main() {
    feature::main();
}

#[cfg(all(feature="winit", feature="glium"))]
mod feature {
    extern crate find_folder;
    use conrod::{self, widget, color, Widget};
    use conrod::backend::glium::glium;
    use conrod::backend::glium::glium::Surface;
    use support;

    use ::yoga::prelude::*;
    use ::yoga::types::{FlexDirection, Wrap};

    pub fn main() {
        const WIDTH: u32 = 200;
        const HEIGHT: u32 = 200;

        // Build the window.
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title("Click me!")
            .with_dimensions(WIDTH, HEIGHT);
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

        // Generate the widget identifiers.
        widget_ids!(struct Ids { canvas, item1, item2, item3, item4 });
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
                widget::Rectangle::fill_with([200.0,200.0], color::BLUE)
                    .layout(&make_styles![
                        Margin((-5) pt),
                        FlexDirection(FlexDirection::Row),
                        FlexWrap(Wrap::Wrap),
                        Padding(10 pt)
                    ])
                    .set(ids.canvas, ui);

                widget::Rectangle::fill_with([60.0,60.0], color::WHITE)
                    .parent(ids.canvas)
                    .layout(&make_styles![
                        Margin(5 pt),
                        Width(60 pt),
                        Height(60 pt)
                    ])
                    .set(ids.item1, ui);

                widget::Rectangle::fill_with([60.0,60.0], color::LIGHT_GRAY)
                    .parent(ids.canvas)
                    .layout(&make_styles![
                        Margin(5 pt),
                        Width(60 pt),
                        Height(60 pt)
                    ])
                    .set(ids.item2, ui);

                widget::Rectangle::fill_with([60.0,60.0], color::WHITE)
                    .parent(ids.canvas)
                    .layout(&make_styles![
                        Margin(5 pt),
                        Width(60 pt),
                        Height(60 pt)
                    ])
                    .set(ids.item3, ui);

                widget::Rectangle::fill_with([60.0,60.0], color::LIGHT_GRAY)
                    .parent(ids.canvas)
                    .layout(&make_styles![
                        Margin(5 pt),
                        Width(60 pt),
                        Height(60 pt)
                    ])
                    .set(ids.item4, ui);
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
