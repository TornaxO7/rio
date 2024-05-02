extern crate tokio;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use sugarloaf::{layout::SugarloafLayout, Sugar};
use sugarloaf::{
    SugarGraphic, SugarGraphicData, SugarGraphicId, Sugarloaf, SugarloafWindow,
    SugarloafWindowSize,
};
use tokio::runtime::Runtime;
use winit::application::ApplicationHandler;
use winit::event_loop::ControlFlow;
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use winit::window::Window;
use winit::{
    dpi::LogicalSize, event::WindowEvent, event_loop::EventLoop, window::WindowAttributes,
};

struct AppData {
    pub window: Window,
    pub _sugarloaf_layout: SugarloafLayout,
    pub sugarloaf: Sugarloaf,

    pub sixel_sugar: Sugar,
}

impl AppData {
    pub fn new(event_loop: &winit::event_loop::ActiveEventLoop) -> Self {
        let width = 800.0;
        let height = 600.0;
        let window = {
            let window_attribute = WindowAttributes::default()
                .with_title("Term example")
                .with_inner_size(LogicalSize::new(width, height))
                .with_resizable(true);
            event_loop.create_window(window_attribute).unwrap()
        };

        let scale_factor = window.scale_factor();
        let font_size = 90.;
        // Unitless values: use this number multiplied
        // by the element's font size
        let line_height = 2.0;

        let sugarloaf_layout = SugarloafLayout::new(
            width as f32,
            height as f32,
            (10.0, 10.0, 0.0),
            scale_factor as f32,
            font_size,
            line_height,
        );

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

        let mut sugarloaf = {
            std::thread::spawn(move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    Sugarloaf::new(
                        sugarloaf_window,
                        sugarloaf::SugarloafRenderer::default(),
                        &sugarloaf::font::FontLibrary::default(),
                        sugarloaf_layout,
                    )
                    .await
                    .expect("Sugarloaf instance should be created")
                })
            })
            .join()
            .unwrap()
        };

        let sixel_sugar = {
            const TEST_IMAGE_ID: SugarGraphicId = SugarGraphicId(69);
            let test_image = image::io::Reader::open("./examples/sixel/peepoShy.png")
                .unwrap()
                .decode()
                .unwrap();

            let test_image_color_type = test_image.color();
            let test_image = test_image.to_rgba8();
            let test_image_sugar_graphic = SugarGraphic {
                id: TEST_IMAGE_ID,
                width: test_image.height() as u16,
                height: test_image.height() as u16,
            };

            sugarloaf.add_graphic(SugarGraphicData {
                id: TEST_IMAGE_ID,
                width: test_image.width() as usize,
                height: test_image.height() as usize,
                color_type: sugarloaf::ColorType::from(test_image_color_type),
                pixels: test_image.into_vec(),
                is_opaque: false,
            });

            Sugar {
                media: Some(test_image_sugar_graphic),
                ..Sugar::default()
            }
        };

        Self {
            window,
            _sugarloaf_layout: sugarloaf_layout,
            sugarloaf,

            sixel_sugar,
        }
    }
}

#[derive(Default)]
struct App {
    data: Option<AppData>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.data
            .get_or_insert(AppData::new(&event_loop))
            .window
            .request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if self.data.is_none() {
            self.resumed(event_loop);
            return;
        }

        let AppData {
            window,
            sugarloaf,
            sixel_sugar,
            ..
        } = self.data.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let scale_factor_f32 = scale_factor as f32;
                let new_inner_size = window.inner_size();
                sugarloaf.rescale(scale_factor_f32);
                sugarloaf.resize(new_inner_size.width, new_inner_size.height);
                window.request_redraw();
            }
            WindowEvent::Resized(new_size) => {
                sugarloaf.resize(new_size.width, new_size.height);
                window.request_redraw();
            }
            WindowEvent::RedrawRequested { .. } => {
                sugarloaf.start_line();
                sugarloaf.insert_on_current_line(&sixel_sugar);
                sugarloaf.finish_line();
                sugarloaf.render();
            }
            _ => event_loop.set_control_flow(ControlFlow::Wait),
        }
    }
}

// TODO: Wrap up
// TODO: Line height

#[tokio::main]
async fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let mut app = App::default();

    event_loop.run_app_on_demand(&mut app).unwrap();
}
