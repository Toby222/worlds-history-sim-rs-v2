use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResourcePlugin,
        render_asset::RenderAssets,
        render_graph::RenderGraph,
        render_resource::{BindGroupDescriptor, BindGroupEntry, BindingResource},
        Render, RenderApp, RenderSet,
    },
    render::{
        render_graph::{Node, NodeRunError, RenderGraphContext},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
    },
};

use crate::{pipeline::ShaderTest, ShaderTestBuffer, ShaderTestCounter};
use crate::{ShaderTestImage, ShaderTestImageBindGroup, SIZE, WORKGROUP_SIZE};

#[derive(Default)]
struct ShaderTestNode {}

impl Node for ShaderTestNode {
    fn update(&mut self, world: &mut World) {
        let buffer = world.resource::<ShaderTestBuffer>();
        let counter = world.resource::<ShaderTestCounter>();

        let mut buffer_data = buffer.slice(..).get_mapped_range_mut();
        buffer_data[0..4].copy_from_slice(&counter.0.to_le_bytes());
        drop(buffer_data);

        buffer.unmap();
        world.resource_mut::<ShaderTestCounter>().inc();
    }

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let texture_bind_group = &world.resource::<ShaderTestImageBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<ShaderTest>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        let render_pipeline = pipeline_cache
            .get_compute_pipeline(pipeline.render_pipeline)
            .unwrap();
        pass.set_pipeline(render_pipeline);
        pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);

        Ok(())
    }
}

fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<ShaderTest>,
    gpu_images: Res<RenderAssets<Image>>,
    shader_test_image: Res<ShaderTestImage>,
    render_device: Res<RenderDevice>,
) {
    let buffer = render_device.create_buffer(&BufferDescriptor {
        label: None,
        usage: BufferUsages::UNIFORM,
        size: 4,
        mapped_at_creation: true,
    });

    let view = &gpu_images[&shader_test_image.0];

    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.texture_bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&view.texture_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: None,
                }),
            },
        ],
    });
    commands.insert_resource(ShaderTestImageBindGroup(bind_group));

    let shader_test_buffer = ShaderTestBuffer(buffer);
    commands.insert_resource(shader_test_buffer);
}

pub struct ShaderTestComputePlugin;

impl Plugin for ShaderTestComputePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<ShaderTestImage>::default());
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(Render, queue_bind_group.in_set(RenderSet::Queue));

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("shader_test", ShaderTestNode::default());
        render_graph.add_node_edge("shader_test", bevy::render::main_graph::node::CAMERA_DRIVER);
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<ShaderTest>();
        render_app.insert_resource(ShaderTestCounter(0));
    }
}
