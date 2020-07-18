use iced_wgpu::wgpu;
use iced_winit::Color;
use shaderc;
use shaderc::CompilationArtifact;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor};

pub struct Scene {
    pub background_color: Color,
    pipeline: wgpu::RenderPipeline,
    color_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    use_color: bool,
}

impl Scene {
    pub fn new(device: &wgpu::Device) -> Scene {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[],
            label: None,
        });
        let bind_group = build_bind_group(device, &bind_group_layout);
        compile_my_shader(
            "examples/iced_triangle/shader/challenge.frag",
            "examples/iced_triangle/shader/challenge_frag.spv",
            shaderc::ShaderKind::Fragment,
        );
        compile_my_shader(
            "examples/iced_triangle/shader/challenge.vert",
            "examples/iced_triangle/shader/challenge_vert.spv",
            shaderc::ShaderKind::Vertex,
        );
        // compile_my_shader("examples/iced_triangle/shader/my.frag", "examples/iced_triangle/shader/my_frag.spv", shaderc::ShaderKind::Fragment);
        // compile_my_shader("examples/iced_triangle/shader/my.vert", "examples/iced_triangle/shader/my_vert.spv", shaderc::ShaderKind::Vertex);
        let pipeline = build_pipeline(
            device,
            BuildPipelineDescriptor {
                farg_path: "examples/iced_triangle/shader/my_frag.spv",
                vert_path: "examples/iced_triangle/shader/my_vert.spv",
                bind_group_layout: &bind_group_layout,
            },
        );
        let color_pipeline = build_pipeline(
            device,
            BuildPipelineDescriptor {
                farg_path: "examples/iced_triangle/shader/challenge_frag.spv",
                vert_path: "examples/iced_triangle/shader/challenge_vert.spv",
                bind_group_layout: &bind_group_layout,
            },
        );
        Scene {
            background_color: Color::WHITE,
            pipeline,
            color_pipeline,
            bind_group,
            use_color: false,
        }
    }

    pub fn toggle_use_color(&mut self) {
        self.use_color = !self.use_color;
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
        rpass.set_pipeline(if !self.use_color {
            &self.pipeline
        } else {
            &self.color_pipeline
        });
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw(0..3, 0..1);
    }
}

struct BuildPipelineDescriptor<'a> {
    farg_path: &'a str,
    vert_path: &'a str,
    bind_group_layout: &'a BindGroupLayout,
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn build_pipeline(
    device: &wgpu::Device,
    build_pipeline_descriptor: BuildPipelineDescriptor,
) -> wgpu::RenderPipeline {
    // let vs = include_bytes!("shader/vert.spv");
    // let fs = include_bytes!("shader/frag.spv");
    // let fs = include_bytes!("shader/my_frag.spv");
    // let vs = include_bytes!("shader/my_vert.spv");

    let fs = get_file_as_byte_vec(&build_pipeline_descriptor.farg_path.to_string());
    let vs = get_file_as_byte_vec(&build_pipeline_descriptor.vert_path.to_string());

    let vs_module =
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&vs[..])).unwrap());
    let fs_module =
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&fs[..])).unwrap());

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&build_pipeline_descriptor.bind_group_layout],
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
    pipeline
}

fn build_bind_group(device: &wgpu::Device, bind_group_layout: &BindGroupLayout) -> BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: bind_group_layout,
        bindings: &[],
        label: None,
    })
}

fn compile_my_shader(path: &str, out: &str, shader_type: shaderc::ShaderKind) {
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.add_macro_definition("EP", Some("main"));
    let source = fs::read_to_string(path).expect("file doesn't exist");
    let frag = compiler
        .compile_into_spirv(
            &source,
            shader_type,
            // "my.frag",
            path,
            "main",
            Some(&options),
        )
        .unwrap();
    let mut file = File::create(out).unwrap();
    file.write_all(frag.as_binary_u8());
}
