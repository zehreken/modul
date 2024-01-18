use wgpu::{
    util::BufferInitDescriptor, util::DeviceExt, BindGroup, BindGroupDescriptor, BindGroupEntry,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, Buffer, BufferBindingType, BufferUsages,
    Device, Queue, RenderPipeline, ShaderStages, SurfaceConfiguration, TextureView,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Uniforms {
    pub time: f32,
}

impl Uniforms {
    fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

pub struct Renderer {
    uniforms: Uniforms,
    uniforms_buffer: Buffer,
    uniforms_bind_group: BindGroup,
    render_pipeline: RenderPipeline,
}

impl Renderer {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        let uniforms = Uniforms::default();
        let uniforms_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: uniforms.as_bytes(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let uniforms_buffer_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                count: None,
                ty: wgpu::BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            }],
        });

        let uniforms_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &uniforms_buffer_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniforms_buffer.as_entire_binding(),
            }],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render pipeline layout"),
                bind_group_layouts: &[&uniforms_buffer_layout],
                push_constant_ranges: &[],
            });

        let vertex_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("vertex shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/six_vertex.wgsl").into()),
        });
        let fragment_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("fragment shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/rt2.wgsl").into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader_module,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &fragment_shader_module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        Self {
            uniforms,
            uniforms_buffer,
            uniforms_bind_group,
            render_pipeline,
        }
    }

    pub fn render(&mut self, device: &Device, queue: &Queue, view: &TextureView, time: f32) {
        self.uniforms.time = time;

        queue.write_buffer(&self.uniforms_buffer, 0, self.uniforms.as_bytes());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("renderer encoder"),
        });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.03,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniforms_bind_group, &[]);
        render_pass.draw(0..6, 0..1);
        drop(render_pass);
        queue.submit(Some(encoder.finish()));
    } // pass a list of objects
}
