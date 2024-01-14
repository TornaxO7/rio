extern crate tokio;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use sugarloaf::{
    core::{Sugar, SugarDecoration},
    layout::SugarloafLayout,
};
use sugarloaf::{Sugarloaf, SugarloafWindow, SugarloafWindowSize};
use winit::event_loop::ControlFlow;
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

#[tokio::main]
async fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let width = 1200.0;
    let height = 800.0;

    let window = WindowBuilder::new()
        .with_title("Text example")
        .with_inner_size(LogicalSize::new(width, height))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    let scale_factor = window.scale_factor();
    let font_size = 90.;

    let sugarloaf_layout = SugarloafLayout::new(
        width as f32,
        height as f32,
        (10.0, 10.0, 0.0),
        scale_factor as f32,
        font_size,
        1.0,
        (2, 1),
    );

    let size = window.inner_size();
    let sugarloaf_window = SugarloafWindow {
        handle: window.raw_window_handle(),
        display: window.raw_display_handle(),
        scale: scale_factor as f32,
        size: SugarloafWindowSize {
            width: size.width,
            height: size.height,
        },
    };

    let mut sugarloaf = Sugarloaf::new(
        &sugarloaf_window,
        sugarloaf::SugarloafRenderer::default(),
        sugarloaf::font::fonts::SugarloafFonts::default(),
        sugarloaf_layout,
        None,
    )
    .await
    .expect("Sugarloaf instance should be created");

    let _ = event_loop.run_on_demand(move |event, event_loop_window_target| {
        event_loop_window_target.set_control_flow(ControlFlow::Wait);

        let sugar = vec![
            Sugar {
                content: 'S',
                fg_color: [1.0, 1.0, 1.0, 1.0],
                bg_color: [0.0, 0.0, 0.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: 'u',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: 'g',
                fg_color: [1.0, 1.0, 1.0, 1.0],
                bg_color: [0.0, 0.0, 0.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: 'a',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: 'r',
                fg_color: [1.0, 1.0, 1.0, 1.0],
                bg_color: [0.0, 0.0, 0.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: 'g',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [0.0, 0.0, 1.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: '|',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
        ];

        let loaf = vec![
            Sugar {
                content: 'l',
                fg_color: [1.0, 1.0, 1.0, 1.0],
                bg_color: [0.0, 0.0, 0.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: 'o',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: 'a',
                fg_color: [1.0, 1.0, 1.0, 1.0],
                bg_color: [0.0, 0.0, 0.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: 'f',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [0.0, 0.0, 1.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: 'g',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [0.0, 0.0, 1.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: '|',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
        ];

        let underline = SugarDecoration {
            relative_position: (0.0, 0.94),
            size: (1.0, 0.03),
            color: [1.0, 0.4, 1.0, 1.0],
        };

        let rio = vec![
            Sugar {
                content: ' ',
                fg_color: [1.0, 1.0, 1.0, 1.0],
                bg_color: [0.0, 0.0, 0.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: 'r',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [0.0, 0.0, 1.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: 'i',
                fg_color: [1.0, 1.0, 1.0, 1.0],
                bg_color: [0.0, 0.0, 0.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: 'o',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [1.0, 1.0, 1.0, 1.0],
                decoration: Some(underline),
                ..Default::default()
            },
            Sugar {
                content: 'g',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [0.0, 1.0, 0.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: '¼',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [1.0, 1.0, 0.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: '¬',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [0.0, 1.0, 0.0, 1.0],
                ..Default::default()
            },
        ];

        let special = vec![
            // Font Unicode (unicode font)
            Sugar {
                content: '㏑',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [0.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
            // Font Symbol (apple symbols font)
            Sugar {
                content: '⫹',
                fg_color: [1.0, 1.0, 1.0, 1.0],
                bg_color: [0.0, 0.0, 0.0, 1.0],
                ..Default::default()
            },
            // Font Regular (firamono)
            Sugar {
                content: 'λ',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [0.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
            // Font Emojis
            Sugar {
                content: '🥇',
                fg_color: [1.0, 1.0, 1.0, 1.0],
                bg_color: [0.0, 0.0, 0.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: '👷',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [0.0, 0.0, 1.0, 1.0],
                ..Default::default()
            },
        ];

        let special_2 = vec![
            // Font Symbol (char width 2)
            Sugar {
                content: 'a',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: '％',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [0.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: '',
                fg_color: [1.0, 1.0, 1.0, 1.0],
                bg_color: [0.5, 0.5, 0.5, 1.0],
                ..Default::default()
            },
            Sugar {
                content: 'a',
                fg_color: [0.0, 0.0, 0.0, 1.0],
                bg_color: [1.0, 1.0, 1.0, 1.0],
                ..Default::default()
            },
            Sugar {
                content: '',
                fg_color: [1.0, 1.0, 1.0, 1.0],
                bg_color: [0.0, 0.0, 0.0, 1.0],
                ..Default::default()
            },
        ];

        match event {
            Event::Resumed => {
                sugarloaf.set_background_color(wgpu::Color::RED);
                sugarloaf.calculate_bounds();
                window.request_redraw();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => event_loop_window_target.exit(),
                WindowEvent::ScaleFactorChanged {
                    // mut inner_size_writer,
                    scale_factor,
                    ..
                } => {
                    let scale_factor_f32 = scale_factor as f32;
                    let new_inner_size = window.inner_size();
                    sugarloaf.rescale(scale_factor_f32);
                    sugarloaf.resize(new_inner_size.width, new_inner_size.height);
                    sugarloaf.calculate_bounds();
                    window.request_redraw();
                }
                winit::event::WindowEvent::Resized(new_size) => {
                    sugarloaf.resize(new_size.width, new_size.height);
                    sugarloaf.calculate_bounds();
                    window.request_redraw();
                }
                winit::event::WindowEvent::RedrawRequested { .. } => {
                    sugarloaf.stack(sugar);
                    sugarloaf.stack(loaf);
                    sugarloaf.stack(special_2);
                    sugarloaf.stack(rio);
                    sugarloaf.stack(special);
                    sugarloaf.render();
                }
                _ => (),
            },
            _ => {
                event_loop_window_target.set_control_flow(ControlFlow::Wait);
            }
        }
    });
}
