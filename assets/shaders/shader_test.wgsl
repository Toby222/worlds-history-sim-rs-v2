@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

@group(0) @binding(1)
var<uniform> counter: u32;

@compute @workgroup_size(8, 8, 1)
fn render(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let textureSize = vec2<f32>(textureDimensions(texture));

    let color = vec4<f32>(sin(f32(location.x) / textureSize.x), sin(f32(location.y) / textureSize.y), sin(f32(counter) / 256.0), 1.0);

    textureStore(texture, location, color);
}