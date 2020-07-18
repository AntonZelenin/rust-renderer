mod scene;

use iced_wgpu::{wgpu, Backend, Renderer, Settings, Viewport};
use iced_winit::winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use iced_winit::{winit, Size};
use scene::Scene;

pub fn main() {
    // Initialize winit
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    let physical_size = window.inner_size();
    // let mut viewport = Viewport::with_physical_size(
    //     Size::new(physical_size.width, physical_size.height),
    //     window.scale_factor(),
    // );

    // Initialize WGPU
    // contains properties of gpu like name, extensions etc. It's like a graphics driver
    let surface = wgpu::Surface::create(&window);
    let (mut device, queue) = futures::executor::block_on(async {
        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        )
        .await
        .expect("Request adapter");

        adapter
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: wgpu::Limits::default(),
            })
            .await
    });
    let format = wgpu::TextureFormat::Bgra8UnormSrgb;

    let mut swap_chain = {
        let size = window.inner_size();

        device.create_swap_chain(
            &surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: format,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Mailbox,
            },
        )
    };
    let mut resized = false;

    // Initialize iced
    // let mut renderer =
    //     Renderer::new(Backend::new(&mut device, Settings::default()));

    let scene = Scene::new(&device);

    // Run event loop
    event_loop.run(move |event, _, control_flow| {
        // You should change this if you want to render continuosly
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(new_size) => {
                    window.set_inner_size(new_size.to_logical::<f32>(window.scale_factor()));
                    resized = true;
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                if resized {
                    let size = window.inner_size();

                    swap_chain = device.create_swap_chain(
                        &surface,
                        &wgpu::SwapChainDescriptor {
                            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                            format,
                            width: size.width,
                            height: size.height,
                            present_mode: wgpu::PresentMode::Mailbox,
                        },
                    );

                    resized = false;
                }

                let frame = swap_chain.get_next_texture().expect("Next frame");

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                // We draw the scene first
                scene.draw(&mut encoder, &frame.view);

                // And then iced on top
                // let mouse_interaction = renderer.backend_mut().draw(
                //     &mut device,
                //     &mut encoder,
                //     &frame.view,
                //     &viewport,
                //     state.primitive(),
                //     &debug.overlay(),
                // );

                // Then we submit the work
                queue.submit(&[encoder.finish()]);

                // And update the mouse cursor
                // window.set_cursor_icon(
                //     iced_winit::conversion::mouse_interaction(
                //         mouse_interaction,
                //     ),
                // );
            }
            _ => {}
        }
    })
}
