#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var material_color_texture: texture_2d<f32>;
@group(2) @binding(1) var material_color_sampler: sampler;
//@group(2) @binding(2) var<uniform> levels: u32; // causes error on wasm

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let lvl = f32(4);
    let color = textureSample(material_color_texture, material_color_sampler, mesh.uv);

    let r = round(color.r * lvl) / (lvl);
    let g = round(color.g * lvl) / (lvl);
    let b = round(color.b * lvl) / (lvl);
    return vec4<f32>(r, g, b, color.a);
}
