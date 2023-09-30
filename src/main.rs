mod pipeline;
mod plugin;
use bevy::{
    prelude::*,
    render::{extract_resource::ExtractResource, render_resource::*},
};
use plugin::ShaderTestComputePlugin;

const SIZE: (u32, u32) = (1920, 1080);
const WORKGROUP_SIZE: u32 = 8;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((DefaultPlugins, ShaderTestComputePlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let image = images.add(image);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(ShaderTestImage(image));
}

#[derive(Resource, Clone, Deref, ExtractResource)]
struct ShaderTestImage(Handle<Image>);

#[derive(Resource, Clone, Deref, ExtractResource)]
struct ShaderTestBuffer(Buffer);

#[derive(Resource, Clone, Deref, ExtractResource)]
struct ShaderTestCounter(u32);
impl ShaderTestCounter {
    fn inc(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}

#[derive(Resource)]
struct ShaderTestImageBindGroup(BindGroup);
