use zerocopy::AsBytes;

pub trait CreateTextureViewWithData {
    fn create_texture_view_with_data(
        &self,
        data: &[u8],
        desc: &wgpu::TextureDescriptor,
    ) -> (wgpu::TextureView, wgpu::CommandBuffer);
}

impl CreateTextureViewWithData for wgpu::Device {
    fn create_texture_view_with_data(
        &self,
        data: &[u8],
        desc: &wgpu::TextureDescriptor,
    ) -> (wgpu::TextureView, wgpu::CommandBuffer) {
        let texture = self.create_texture(desc);
        let texture_view = texture.create_default_view();
        let copy_data = self.create_buffer_with_data(data, wgpu::BufferUsage::COPY_SRC);
        let mut copy_encoder =
            self.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        copy_encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &copy_data,
                offset: 0,
                row_pitch: 4 * desc.size.width,
                image_height: desc.size.height,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: 0,
                },
            },
            desc.size,
        );
        (texture_view, copy_encoder.finish())
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_title("Graphics 2020 January 01");
    window.set_resizable(false);
    window.set_inner_size(winit::dpi::LogicalSize::<u32>::from((512u32, 512u32)));
    let size = window.inner_size();

    let adapter = wgpu::Adapter::request(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
        },
        wgpu::BackendBit::PRIMARY,
    )
    .unwrap();

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    let context = device.create_buffer_with_data(
        [0.0].to_vec().as_bytes(),
        wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::MAP_WRITE,
    );

    let context_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare_function: wgpu::CompareFunction::Always,
    });

    let plan = device
        .create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: 512,
                height: 512,
                depth: 1,
            },
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsage::STORAGE,
        })
        .create_default_view();

    let text = {
        let (texture_view, copy_buffer) = device.create_texture_view_with_data(
            image::load_from_memory_with_format(include_bytes!("./img/text.png"), image::PNG)
                .unwrap()
                .to_rgba()
                .to_vec()
                .as_slice(),
            &wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: 2048,
                    height: 2048,
                    depth: 1,
                },
                array_layer_count: 1,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            },
        );
        queue.submit(&[copy_buffer]);
        texture_view
    };

    let raid = device
        .create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: 512,
                height: 512,
                depth: 64,
            },
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::R16Float,
            usage: wgpu::TextureUsage::STORAGE,
        })
        .create_default_view();

    let trade = device
        .create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: 512,
                height: 512,
                depth: 1,
            },
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R16Float,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::STORAGE,
        })
        .create_default_view();

    let vertices = device.create_buffer_with_data(
        [-3.0f32, -1.0f32, 1.0f32, -1.0f32, 1.0f32, 3.0f32]
            .to_vec()
            .as_bytes(),
        wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
    );

    let context_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::COMPUTE | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 1,
                    visibility: wgpu::ShaderStage::COMPUTE | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler,
                },
            ],
        });

    let context_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &context_bind_group_layout,
        bindings: &[
            wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &context,
                    range: 0u64..4u64,
                },
            },
            wgpu::Binding {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&context_sampler),
            },
        ],
    });

    let plan_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::COMPUTE | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::StorageTexture {
                    dimension: wgpu::TextureViewDimension::D2,
                },
            }],
        });

    let plan_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &plan_bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&plan),
        }],
    });

    let text_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::COMPUTE | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::SampledTexture {
                    multisampled: false,
                    dimension: wgpu::TextureViewDimension::D2,
                },
            }],
        });

    let text_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &text_bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&text),
        }],
    });

    let raid_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::COMPUTE | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::StorageTexture {
                    dimension: wgpu::TextureViewDimension::D3,
                },
            }],
        });

    let raid_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &raid_bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&raid),
        }],
    });

    let trade_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::COMPUTE | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::StorageTexture {
                    dimension: wgpu::TextureViewDimension::D2,
                },
            }],
        });

    let trade_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &trade_bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&trade),
        }],
    });

    let render_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::SampledTexture {
                    multisampled: false,
                    dimension: wgpu::TextureViewDimension::D2,
                },
            }],
        });

    let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &render_bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&trade),
        }],
    });

    let plan_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        layout: &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&plan_bind_group_layout],
        }),
        compute_stage: wgpu::ProgrammableStageDescriptor {
            module: &device.create_shader_module(
                &wgpu::read_spirv(std::io::Cursor::new(
                    &include_bytes!("./gpu/compute/plan.spv")[..],
                ))
                .unwrap(),
            ),
            entry_point: "main",
        },
    });

    let raid_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        layout: &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[
                &context_bind_group_layout,
                &plan_bind_group_layout,
                &text_bind_group_layout,
                &raid_bind_group_layout,
            ],
        }),
        compute_stage: wgpu::ProgrammableStageDescriptor {
            module: &device.create_shader_module(
                &wgpu::read_spirv(std::io::Cursor::new(
                    &include_bytes!("./gpu/compute/raid.spv")[..],
                ))
                .unwrap(),
            ),
            entry_point: "main",
        },
    });

    let trade_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        layout: &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&raid_bind_group_layout, &trade_bind_group_layout],
        }),
        compute_stage: wgpu::ProgrammableStageDescriptor {
            module: &device.create_shader_module(
                &wgpu::read_spirv(std::io::Cursor::new(
                    &include_bytes!("./gpu/compute/trade.spv")[..],
                ))
                .unwrap(),
            ),
            entry_point: "main",
        },
    });

    let surface_format: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

    let mut swap_chain = device.create_swap_chain(
        &wgpu::Surface::create(&window),
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Vsync,
        },
    );

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&context_bind_group_layout, &render_bind_group_layout],
        }),
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &device.create_shader_module(
                &wgpu::read_spirv(std::io::Cursor::new(
                    &include_bytes!("./gpu/render/vertex.spv")[..],
                ))
                .unwrap(),
            ),
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &device.create_shader_module(
                &wgpu::read_spirv(std::io::Cursor::new(
                    &include_bytes!("./gpu/render/fragment.spv")[..],
                ))
                .unwrap(),
            ),
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
            format: surface_format,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[wgpu::VertexBufferDescriptor {
            stride: 2 * 4,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[wgpu::VertexAttributeDescriptor {
                offset: 0,
                format: wgpu::VertexFormat::Float2,
                shader_location: 0,
            }],
        }],
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    let instant = time::Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                        state: winit::event::ElementState::Pressed,
                        ..
                    },
                ..
            }
            | winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            _ => (),
        },
        winit::event::Event::MainEventsCleared => {
            if let Ok(mut content) = futures::executor::block_on(context.map_write(0u64, 4u64)) {
                content
                    .as_slice()
                    .copy_from_slice([instant.elapsed().as_seconds_f32()].to_vec().as_bytes())
            }
            let frame = swap_chain.get_next_texture().unwrap();

            let mut command_encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

            {
                let mut plan_pass = command_encoder.begin_compute_pass();
                plan_pass.set_pipeline(&plan_pipeline);
                plan_pass.set_bind_group(0, &plan_bind_group, &[]);
                plan_pass.dispatch(32, 32, 1);
            }

            {
                let mut raid_pass = command_encoder.begin_compute_pass();
                raid_pass.set_pipeline(&raid_pipeline);
                raid_pass.set_bind_group(0, &context_bind_group, &[]);
                raid_pass.set_bind_group(1, &plan_bind_group, &[]);
                raid_pass.set_bind_group(2, &text_bind_group, &[]);
                raid_pass.set_bind_group(3, &raid_bind_group, &[]);
                raid_pass.dispatch(64, 64, 16);
            }

            {
                let mut trade_pass = command_encoder.begin_compute_pass();
                trade_pass.set_pipeline(&trade_pipeline);
                trade_pass.set_bind_group(0, &raid_bind_group, &[]);
                trade_pass.set_bind_group(1, &trade_bind_group, &[]);
                trade_pass.dispatch(32, 32, 1);
            }

            {
                let mut rpass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::BLACK,
                    }],
                    depth_stencil_attachment: None,
                });
                rpass.set_pipeline(&render_pipeline);
                rpass.set_bind_group(0, &context_bind_group, &[]);
                rpass.set_bind_group(1, &render_bind_group, &[]);
                rpass.set_vertex_buffers(0, &[(&vertices, 0)]);
                rpass.draw(0..3, 0..1);
            }

            queue.submit(&[command_encoder.finish()]);
        }
        _ => (),
    });
}
