extern crate criterion;
extern crate sugarloaf;

use crate::layout::SugarloafLayout;
use criterion::{criterion_group, criterion_main, Criterion};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use sugarloaf::*;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn bench_sugar_pile(c: &mut Criterion) {
    const NUM: usize = 100_000;

    let event_loop = EventLoop::new();
    let width = 1200.0;
    let height = 800.0;

    let window = WindowBuilder::new()
        .with_title("Bench")
        .with_inner_size(LogicalSize::new(width, height))
        .with_resizable(true)
        .build(&event_loop.unwrap())
        .unwrap();

    let scale_factor = window.scale_factor();
    let font_size = 60.;
    let line_height = 1.0;

    let size = window.inner_size();
    let sugarloaf_window = SugarloafWindow {
        handle: window.window_handle().unwrap().into(),
        display: window.display_handle().unwrap().into(),
        scale: scale_factor as f32,
        size: SugarloafWindowSize {
            width: size.width as f32,
            height: size.height as f32,
        },
    };

    let sugarloaf_layout = SugarloafLayout::new(
        width as f32,
        height as f32,
        (0.0, 0.0, 0.0),
        scale_factor as f32,
        font_size,
        line_height,
    );

    let font_library = sugarloaf::font::FontLibrary::default();
    let mut sugarloaf = futures::executor::block_on(Sugarloaf::new(
        sugarloaf_window,
        sugarloaf::SugarloafRenderer::default(),
        &font_library,
        sugarloaf_layout,
    ))
    .expect("Sugarloaf instance should be created");

    sugarloaf.set_background_color(wgpu::Color::RED);

    c.bench_function("bench_sugar_pile", |b| {
        b.iter(|| {
            sugarloaf.start_line();
            for _i in 0..NUM {
                sugarloaf.insert_on_current_line(&Sugar {
                    content: ' ',
                    foreground_color: [0.0, 0.0, 0.0, 1.0],
                    background_color: [0.0, 1.0, 1.0, 1.0],
                    ..Sugar::default()
                });

                sugarloaf.insert_on_current_line(&Sugar {
                    content: '«',
                    foreground_color: [0.0, 0.0, 0.0, 1.0],
                    background_color: [0.0, 1.0, 1.0, 1.0],
                    ..Sugar::default()
                });

                sugarloaf.insert_on_current_line(&Sugar {
                    content: '≥',
                    foreground_color: [0.0, 0.0, 0.0, 1.0],
                    background_color: [0.0, 1.0, 1.0, 1.0],
                    ..Sugar::default()
                });
            }

            sugarloaf.finish_line();
            sugarloaf.render();
        })
    });
}

criterion_group!(benches, bench_sugar_pile);
criterion_main!(benches);
