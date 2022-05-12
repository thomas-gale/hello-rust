use bevy::{
    prelude::*,
    render::{mesh::MeshVertexAttribute, render_resource::VertexFormat},
};

use super::BasicVertexMaterial;

// A "high" random id should be used for custom attributes to ensure consistent sorting and avoid collisions with other attributes.
// See the MeshVertexAttribute docs for more info.
const ATTRIBUTE_BLEND_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("BlendColor", 988540917, VertexFormat::Float32x4);

pub fn setup_basic_vertex_material(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BasicVertexMaterial>>,
) {
    let mut mesh = Mesh::from(shape::Cube { size: 1. });
    mesh.insert_attribute(
        ATTRIBUTE_BLEND_COLOR,
        // The cube mesh has 24 vertices (6 faces, 4 vertices per face), so we insert one BlendColor for each
        vec![[0.1, 0.6, 0.1, 1.0]; 24],
    );

    // cube
    commands
        .spawn()
        .insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(mesh),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(10., 0.1, 10.)),
            material: materials.add(BasicVertexMaterial {
                color: Color::WHITE,
            }),
            ..default()
        })
        .insert(Name::new("terrain"));
}
