use iced_wgpu::wgpu;
use iced_winit::Color;
use shaderc;
use shaderc::CompilationArtifact;
use std::fs;
use std::fs::File;
use std::io::Write;

pub struct Scene {
    pub background_color: Color,
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
}

impl Scene {
    pub fn new(device: &wgpu::Device) -> Scene {
        let (pipeline, bind_group) = build_pipeline(device);

        Scene {
            background_color: Color::BLACK,
            pipeline,
            bind_group,
        }
    }

    pub fn draw(&self, encoder: &mut wgpu::CommandEncoder, target: &wgpu::TextureView) {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: target,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: {
                    let [r, g, b, a] = self.background_color.into_linear();

                    wgpu::Color {
                        r: r as f64,
                        g: g as f64,
                        b: b as f64,
                        a: a as f64,
                    }
                },
            }],
            depth_stencil_attachment: None,
        });

        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw(0..3, 0..1);
    }
}

fn build_pipeline(device: &wgpu::Device) -> (wgpu::RenderPipeline, wgpu::BindGroup) {
    compile_my_shader("examples/triangle/shader/my.frag", "examples/triangle/shader/my_frag.spv", shaderc::ShaderKind::Fragment);
    compile_my_shader("examples/triangle/shader/my.vert", "examples/triangle/shader/my_vert.spv", shaderc::ShaderKind::Vertex);
    // let vs = include_bytes!("shader/vert.spv");
    // let fs = include_bytes!("shader/frag.spv");
    let fs = include_bytes!("shader/my_frag.spv");
    let vs = include_bytes!("shader/my_vert.spv");

    let vs_module =
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&vs[..])).unwrap());

    let fs_module =
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&fs[..])).unwrap());

    let bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[],
            label: None,
        });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        bindings: &[],
        label: None,
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[],
        },
    });

    (pipeline, bind_group)
}

fn compile_my_shader(path: &str, out: &str, shader_type: shaderc::ShaderKind) {
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.add_macro_definition("EP", Some("main"));
    let source = fs::read_to_string(path).expect("file doesn't exist");
    let frag= compiler.compile_into_spirv(
        &source,
        shader_type,
        "my.frag",
        "main",
        Some(&options)
    ).unwrap();
    let mut file = File::create(out).unwrap();
    file.write_all(frag.as_binary_u8());
}
