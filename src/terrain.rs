use bevy::prelude::*;

mod basic_vertex_material;
use basic_vertex_material::BasicVertexMaterial;
mod setup_basic_vertex_material;
use setup_basic_vertex_material::setup_basic_vertex_material;
mod generate;
use generate::calculate;

/// Morphed from https://github.com/bevyengine/bevy/blob/v0.7.0/examples/shader/custom_vertex_attribute.rs starting point.
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<BasicVertexMaterial>::default())
            .add_startup_system(setup_basic_vertex_material)
            .add_system(calculate);
    }
}

// A "high" random id should be used for custom attributes to ensure consistent sorting and avoid collisions with other attributes.
// See the MeshVertexAttribute docs for more info.
// const ATTRIBUTE_BLEND_COLOR: MeshVertexAttribute =
//     MeshVertexAttribute::new("BlendColor", 988540917, VertexFormat::Float32x4);

// fn setup_custom_vertex_material(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<CustomMaterial>>,
// ) {
//     let mut mesh = Mesh::from(shape::Cube { size: 1. });
//     mesh.insert_attribute(
//         ATTRIBUTE_BLEND_COLOR,
//         // The cube mesh has 24 vertices (6 faces, 4 vertices per face), so we insert one BlendColor for each
//         vec![[0.1, 0.6, 0.1, 1.0]; 24],
//     );

//     // cube
//     commands
//         .spawn()
//         .insert_bundle(MaterialMeshBundle {
//             mesh: meshes.add(mesh),
//             transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(10., 0.1, 10.)),
//             material: materials.add(CustomMaterial {
//                 color: Color::WHITE,
//             }),
//             ..default()
//         })
//         .insert(Name::new("terrain"));
// }

// #[derive(Debug, Clone, TypeUuid)]
// #[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
// struct CustomMaterial {
//     color: Color,
// }

// #[derive(Clone)]
// struct GpuCustomMaterial {
//     _buffer: Buffer,
//     bind_group: BindGroup,
// }

// // The implementation of [`Material`] needs this impl to work properly.
// impl RenderAsset for CustomMaterial {
//     type ExtractedAsset = CustomMaterial;
//     type PreparedAsset = GpuCustomMaterial;
//     type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);
//     fn extract_asset(&self) -> Self::ExtractedAsset {
//         self.clone()
//     }

//     fn prepare_asset(
//         extracted_asset: Self::ExtractedAsset,
//         (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
//     ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
//         let color = Vec4::from_slice(&extracted_asset.color.as_linear_rgba_f32());
//         let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
//             contents: color.as_std140().as_bytes(),
//             label: None,
//             usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
//         });
//         let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
//             entries: &[BindGroupEntry {
//                 binding: 0,
//                 resource: buffer.as_entire_binding(),
//             }],
//             label: None,
//             layout: &material_pipeline.material_layout,
//         });

//         Ok(GpuCustomMaterial {
//             _buffer: buffer,
//             bind_group,
//         })
//     }
// }

// impl Material for CustomMaterial {
//     // When creating a custom material, you need to define either a vertex shader, a fragment shader or both.
//     // If you don't define one of them it will use the default mesh shader which can be found at
//     // <https://github.com/bevyengine/bevy/blob/latest/crates/bevy_pbr/src/render/mesh.wgsl>

//     fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
//         Some(asset_server.load("shaders/custom_vertex_attribute.wgsl"))
//     }
//     fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
//         Some(asset_server.load("shaders/custom_vertex_attribute.wgsl"))
//     }

//     fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
//         &render_asset.bind_group
//     }

//     fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
//         render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
//             entries: &[BindGroupLayoutEntry {
//                 binding: 0,
//                 visibility: ShaderStages::FRAGMENT,
//                 ty: BindingType::Buffer {
//                     ty: BufferBindingType::Uniform,
//                     has_dynamic_offset: false,
//                     min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64),
//                 },
//                 count: None,
//             }],
//             label: None,
//         })
//     }

//     fn specialize(
//         _pipeline: &MaterialPipeline<Self>,
//         descriptor: &mut RenderPipelineDescriptor,
//         layout: &MeshVertexBufferLayout,
//     ) -> Result<(), SpecializedMeshPipelineError> {
//         let vertex_layout = layout.get_layout(&[
//             Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
//             ATTRIBUTE_BLEND_COLOR.at_shader_location(1),
//         ])?;
//         descriptor.vertex.buffers = vec![vertex_layout];
//         Ok(())
//     }
// }
