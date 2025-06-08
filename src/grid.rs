use bevy::asset::RenderAssetUsages;
use bevy::prelude::Mesh;
use bevy::render::mesh::PrimitiveTopology;

fn push_color_check(i: i32, colors: &mut Vec<[f32; 4]>){
    if i == 0 {
        colors.append(&mut vec![[0.5, 0.5, 0.5, 1.0], [0.5, 0.5, 0.5, 1.0]])
    } else {
        colors.append(&mut vec![[0.75, 0.75, 0.75, 1.0], [0.75, 0.75, 0.75, 1.0]])
    }
}

pub fn gen_mesh(cells: u32) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();
    let hcells = cells as i32 / 2;

    for i in -hcells..=hcells {
        push_color_check(i, &mut colors);
        let z = i as f32 / cells as f32;
        positions.push([-hcells as f32 / cells as f32, 0.0, z]);
        positions.push([hcells as f32 / cells as f32, 0.0, z]);
    }

    for i in -hcells..=hcells {
        push_color_check(i, &mut colors);
        let x = i as f32 / cells as f32;
        positions.push([x, 0.0, -hcells as f32 / cells as f32]);
        positions.push([x, 0.0, hcells as f32 / cells as f32]);
    }

    Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
}
