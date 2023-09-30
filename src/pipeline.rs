use bevy::{
    prelude::*,
    render::{render_resource::*, renderer::RenderDevice},
};
use std::borrow::Cow;
#[derive(Resource)]
pub(crate) struct ShaderTest {
    pub(crate) texture_bind_group_layout: BindGroupLayout,
    // pub(crate) init_pipeline: CachedComputePipelineId,
    // pub(crate) update_pipeline: CachedComputePipelineId,
    pub(crate) render_pipeline: CachedComputePipelineId,
}

impl FromWorld for ShaderTest {
    fn from_world(world: &mut World) -> Self {
        let texture_bind_group_layout =
            world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::StorageTexture {
                                access: StorageTextureAccess::ReadWrite,
                                format: TextureFormat::Rgba8Unorm,
                                view_dimension: TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });

        let shader = world
            .resource::<AssetServer>()
            .load("shaders/shader_test.wgsl");
        let pipeline_cache = world.resource::<PipelineCache>();
        let render_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: vec![],
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("render"),
        });

        ShaderTest {
            texture_bind_group_layout,
            render_pipeline,
        }
    }
}
