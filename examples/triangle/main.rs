mod scene;

use scene::Scene;

use iced_wgpu::{wgpu, window::SwapChain, Primitive, Renderer, Settings, Target};
use iced_winit::{winit, MouseCursor};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    dpi::LogicalSize
};

pub fn main() {
    // Initialize winit
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    // Initialize WGPU
    // contains properties of gpu like name, extensions etc. It's like a graphics driver
    let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::Default,
        backends: wgpu::BackendBit::PRIMARY,
    })
    .expect("Request adapter");

    // device is an instance of a graphics driver
    let (mut device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    let surface = wgpu::Surface::create(&window);
    let format = wgpu::TextureFormat::Bgra8UnormSrgb;

    let mut swap_chain = {
        let size = window.inner_size();
        SwapChain::new(&device, &surface, format, size.width, size.height)
    };
    let mut resized = false;

    // Initialize iced
    let mut renderer = Renderer::new(&mut device, Settings::default());
    let output = (Primitive::None, MouseCursor::OutOfBounds);

    let scene = Scene::new(&device);

    // Run event loop
    event_loop.run(move |event, _, control_flow| {
        // You should change this if you want to render continuosly
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(new_size) => {
                        window.set_inner_size(new_size.to_logical::<f32>(window.scale_factor()));
                        resized = true;
                    }
                    _ => {}
                }

            }
            Event::RedrawRequested(_) => {
                if resized {
                    let size = window.inner_size();

                    swap_chain = SwapChain::new(
                        &device,
                        &surface,
                        format,
                        size.width,
                        size.height,
                    );
                }

                let (frame, viewport) = swap_chain.next_frame();

                let mut encoder = device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor { todo: 0 },
                );

                // We draw the scene first
                scene.draw(&mut encoder, &frame.view);

                // And then iced on top
                let mouse_cursor = renderer.draw(
                    &mut device,
                    &mut encoder,
                    Target {
                        texture: &frame.view,
                        viewport,
                    },
                    &output,
                    window.scale_factor(),
                    &["Some debug information!"],
                );

                // Then we submit the work
                queue.submit(&[encoder.finish()]);

                // And update the mouse cursor
                window.set_cursor_icon(iced_winit::conversion::mouse_cursor(
                    mouse_cursor,
                ));
            }
            _ => {}
        }
    })
}
