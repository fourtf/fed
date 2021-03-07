use crate::model::{EditorStateRef, TextModel, Selection};
use crate::input::{VimInput, Location};
use glutin::event::ModifiersState;
use skia_safe as skia;
use clipboard::{ClipboardProvider, ClipboardContext};
use super::editor::Editor;
use super::Widget;

// ![allow(dead_code)]
// cargo 1.45.1 / rustfmt 1.4.17-stable fails to process the relative path on Windows.
// #[rustfmt::skip]
// #[path = "../icon/renderer.rs"]
// mod renderer;

// #[cfg(target_os = "android")]
// pub fn run() {
//     run!("This example is not supported on Android (https://github.com/rust-windowing/winit/issues/948).")
// }

// #[cfg(all(not(target_os = "android"), not(feature = "gl")))]
// fn main() {
//     println!("To run this example, invoke cargo with --feature \"gl\".")
// }

#[cfg(all(not(target_os = "android")))]
pub fn run(state: EditorStateRef) {
    use skia_safe::gpu::gl::FramebufferInfo;
    use skia_safe::gpu::{BackendRenderTarget, SurfaceOrigin};
    use skia_safe::{Color, ColorType, Surface};
    use std::convert::TryInto;

    use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
    use glutin::event_loop::{ControlFlow, EventLoop};
    use glutin::window::WindowBuilder;
    use glutin::GlProfile;
    type WindowedContext = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;
    use gl::types::*;

    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(1500, 1000))
        .with_title("fed");

    let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_stencil_buffer(8)
        .with_pixel_format(24, 8)
        .with_double_buffer(Some(true))
        .with_gl_profile(GlProfile::Core);

    let windowed_context = cb.build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    // let pixel_format = windowed_context.get_pixel_format();

    // println!(
    //     "Pixel format of the window's GL context: {:?}",
    //     pixel_format
    // );

    gl::load_with(|s| windowed_context.get_proc_address(&s));

    let mut gr_context = skia_safe::gpu::DirectContext::new_gl(None, None).unwrap();

    let fb_info = {
        let mut fboid: GLint = 0;
        unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

        FramebufferInfo {
            fboid: fboid.try_into().unwrap(),
            format: skia_safe::gpu::gl::Format::RGBA8.into(),
        }
    };

    fn create_surface(
        windowed_context: &WindowedContext,
        fb_info: &FramebufferInfo,
        gr_context: &mut skia_safe::gpu::DirectContext,
    ) -> skia_safe::Surface {
        let pixel_format = windowed_context.get_pixel_format();
        let size = windowed_context.window().inner_size();
        let backend_render_target = BackendRenderTarget::new_gl(
            (
                size.width.try_into().unwrap(),
                size.height.try_into().unwrap(),
            ),
            pixel_format.multisampling.map(|s| s.try_into().unwrap()),
            pixel_format.stencil_bits.try_into().unwrap(),
            *fb_info,
        );
        Surface::from_backend_render_target(
            gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        )
        .unwrap()
    };

    let mut surface = create_surface(&windowed_context, &fb_info, &mut gr_context);
    // let sf = windowed_context.window().scale_factor() as f32;
    // surface.canvas().scale((sf, sf));

    let tf = skia::Typeface::new(
        "Consolas",
        skia::FontStyle::new(
            skia::font_style::Weight::MEDIUM,
            skia::font_style::Width::NORMAL,
            skia::font_style::Slant::Upright,
        ),
    )
    .unwrap();
    let font = skia::Font::new(tf, Some(20.));

    let mut input = VimInput::new();

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        let map_text_model = |f: &dyn Fn(TextModel) -> TextModel| {
            let new = state.borrow().open_file.model.clone();
            let new = f(new);
            let mut b = state.borrow_mut();
            b.open_file.model = new.clone();
            b.open_file.undo_stack.push_front(new);
            if b.open_file.undo_stack.len() > 100 {
                b.open_file.undo_stack.pop_back();
            }
        };

        #[allow(deprecated)]
        match event {
            Event::LoopDestroyed => {
                *control_flow = ControlFlow::Exit;
                std::process::exit(0);
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    surface = create_surface(&windowed_context, &fb_info, &mut gr_context);
                    windowed_context.resize(physical_size)
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode,
                            modifiers,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    if modifiers.logo() {
                        if let Some(VirtualKeyCode::Q) = virtual_keycode {
                            *control_flow = ControlFlow::Exit;
                        }
                    }

                    if modifiers == ModifiersState::empty() {
                        if Some(VirtualKeyCode::Up) == virtual_keycode {
                            map_text_model(&|x| x.move_cursor((-1, 0)));
                        } else if Some(VirtualKeyCode::Down) == virtual_keycode {
                            map_text_model(&|x| x.move_cursor((1, 0)));
                        } else if Some(VirtualKeyCode::Left) == virtual_keycode {
                            map_text_model(&|x| x.move_cursor((0, -1)));
                        } else if Some(VirtualKeyCode::Right) == virtual_keycode {
                            map_text_model(&|x| x.move_cursor((0, 1)));
                        } else if Some(VirtualKeyCode::Tab) == virtual_keycode {
                            map_text_model(&|x| x.insert("    "));
                        }

                        match input.mode {
                            crate::input::Mode::Visual => {
                                let selection = state.borrow().open_file.selection;
                                let cursor = state.borrow().open_file.model.cursor;
                                state.borrow_mut().open_file.selection = selection.with_end(cursor);
                            },
                            crate::input::Mode::VisualLine => {
                                let selection = state.borrow().open_file.selection;
                                let cursor = state.borrow().open_file.model.cursor;
                                let lines = state.borrow().open_file.model.lines.clone();
                                let mut selection = selection.with_end(cursor);
                                selection.first.column = 0;
                                selection.last.column = lines.get(selection.last.row).map(|x| x.len()).unwrap_or(0);
                                state.borrow_mut().open_file.selection = selection;
                            },
                            _ => (),
                        }
                    } else if modifiers == ModifiersState::CTRL {
                        if Some(VirtualKeyCode::S) == virtual_keycode {
                            match state.borrow().open_file.save() {
                                Err(e) => println!("Error while saving: {}", e),
                                _ => println!("Saved successfully"),
                            }
                        }
                    }

                    windowed_context.window().request_redraw();
                }
                WindowEvent::ReceivedCharacter(c) => {
                    use crate::input::EditorAction::*;

                    for action in input.receive_char(c) {
                        match action {
                            InsertString(str) => map_text_model(&|x| x.insert(str.as_str())),
                            InsertNewline => map_text_model(&|x| x.insert_newline()),
                            DeleteLeft => map_text_model(&|x| x.backspace_key()),
                            Copy => {
                                let s = state.borrow().open_file.model.get_string(state.borrow().open_file.selection);
    
                                // TODO: make safe
                                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                                ctx.set_contents(s).unwrap();
                            },
                            Paste => {
                                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                                // TODO: make safe
                                let s = ctx.get_contents().unwrap_or(String::new());
                                map_text_model(&|x| x.insert(&*s));
                            },
                            Cut => {
                                let selection = state.borrow().open_file.selection;
                                state.borrow_mut().open_file.selection = Selection::empty();
                                map_text_model(&|x| x.delete(selection));
                            },
                            BeginSelection => {
                                let cursor = state.borrow().open_file.model.cursor;
                                state.borrow_mut().open_file.selection = crate::model::Selection::new(cursor, cursor);
                            },
                            EndSelection => { state.borrow_mut().open_file.selection = Selection::empty(); },
                            GoTo(loc) => {
                                match loc {
                                    Location::EndOfLine => map_text_model(&|x| x.move_cursor((0, 1000000000))),
                                    Location::StartOfLine => map_text_model(&|x| x.move_cursor((0, -1000000000))),
                                    Location::FirstLine => map_text_model(&|x| x.move_cursor((-1000000000, 0))),
                                    Location::LastLine => map_text_model(&|x| x.move_cursor((1000000000, 0))),
                                    Location::Offset(delta) => map_text_model(&|x| x.move_cursor(delta)),
                                }
                            },
                            //_ => (),
                        };
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                {
                    let mut canvas = surface.canvas();
                    canvas.clear(Color::DARK_GRAY);

                    let sf = windowed_context.window().scale_factor() as f32;
                    canvas.reset_matrix();
                    canvas.scale((sf, sf));

                    let mut editor = Editor {
                        model: &state.borrow().open_file.model,
                        selection: &state.borrow().open_file.selection,
                        font: &font,
                        input: &input,
                    };

                    let size = windowed_context.window().inner_size().to_logical(windowed_context.window().scale_factor());
                    let rect = skia::Rect::from_xywh(0., 0., size.width, size.height);
                    
                    editor.draw(canvas, &rect);
                }
                surface.canvas().flush();
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}

