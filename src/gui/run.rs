use super::container::Container;
use super::editor::Editor;
use super::{Widget, DrawInfo};
use crate::input::{Location, VimInput};
use crate::model::{EditorStateRef, Selection, TextModel};
use glutin::event::ModifiersState;
use skia_safe as skia;
use std::rc::Rc;

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
    let font = Rc::new(skia::Font::new(tf, Some(20.)));

    let mut root_widget = Container::make_row(crate::collection_items![
        Box::new(super::rect::Rect {}),
        Box::new(Editor { state, font })
    ]);

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

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
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == glutin::event::ElementState::Pressed {
                        root_widget.handle_event(&crate::gui::Event::KeyboardInput(input));
                    }

                    windowed_context.window().request_redraw();
                }
                WindowEvent::ReceivedCharacter(c) => {
                    root_widget.handle_event(&crate::gui::Event::Input(c));

                    windowed_context.window().request_redraw();
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

                    let size = windowed_context
                        .window()
                        .inner_size()
                        .to_logical(windowed_context.window().scale_factor());
                    let rect = skia::Rect::from_xywh(0., 0., size.width, size.height);

                    root_widget.draw(canvas, &rect, DrawInfo { is_focused: true });
                }
                surface.canvas().flush();
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
