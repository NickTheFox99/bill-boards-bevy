#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var material_color_texture: texture_2d<f32>;
@group(2) @binding(1) var material_color_sampler: sampler;
@group(2) @binding(2) var<uniform> material_color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return (textureSample(material_color_texture, material_color_sampler, mesh.uv) * material_color);
}
