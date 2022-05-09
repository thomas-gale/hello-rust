use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::MaterialPipeline,
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer,
            BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages,
            RenderPipelineDescriptor, ShaderStages, SpecializedMeshPipelineError, VertexFormat,
        },
        renderer::RenderDevice,
    },
};

mod pan_orbit_camera;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(set_window)
        .add_startup_system(pan_orbit_camera::spawn_camera)
        .add_startup_system(setup_model_lights)
        .add_startup_system(setup_custom_vertex_material)
        .add_system(animate_light_direction)
        .add_system(pan_orbit_camera::pan_orbit_camera)
        .run();
}

fn set_window(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    println!("Window size was: {},{}", window.width(), window.height());
    window.set_resolution(500.0, 500.0);
}

fn setup_model_lights(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_scene(asset_server.load("models/digger_v3.glb#Scene0"));
    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

// A "high" random id should be used for custom attributes to ensure consistent sorting and avoid collisions with other attributes.
// See the MeshVertexAttribute docs for more info.
const ATTRIBUTE_BLEND_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("BlendColor", 988540917, VertexFormat::Float32x4);

fn setup_custom_vertex_material(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let mut mesh = Mesh::from(shape::Cube { size: 2.0 });
    mesh.insert_attribute(
        ATTRIBUTE_BLEND_COLOR,
        // The cube mesh has 24 vertices (6 faces, 4 vertices per face), so we insert one BlendColor for each
        vec![[1.0, 1.0, 0.0, 1.0]; 24],
    );

    // cube
    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_xyz(0., 0., 0.),
        material: materials.add(CustomMaterial {
            color: Color::WHITE,
        }),
        ..default()
    });
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}

// This is the struct that will be passed to your shader
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    color: Color,
}

#[derive(Clone)]
pub struct GpuCustomMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

// The implementation of [`Material`] needs this impl to work properly.
impl RenderAsset for CustomMaterial {
    type ExtractedAsset = CustomMaterial;
    type PreparedAsset = GpuCustomMaterial;
    type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let color = Vec4::from_slice(&extracted_asset.color.as_linear_rgba_f32());
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: color.as_std140().as_bytes(),
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuCustomMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}

impl Material for CustomMaterial {
    // When creating a custom material, you need to define either a vertex shader, a fragment shader or both.
    // If you don't define one of them it will use the default mesh shader which can be found at
    // <https://github.com/bevyengine/bevy/blob/latest/crates/bevy_pbr/src/render/mesh.wgsl>

    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/custom_vertex_attribute.wgsl"))
    }
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/custom_vertex_attribute.wgsl"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64),
                },
                count: None,
            }],
            label: None,
        })
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_BLEND_COLOR.at_shader_location(1),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
