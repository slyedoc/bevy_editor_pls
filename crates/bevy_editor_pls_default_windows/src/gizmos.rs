use bevy::{
    ecs::query::QueryFilter,
    prelude::*, render::view::RenderLayers,
    //render::{camera::CameraProjection, view::RenderLayers},
};

use bevy_editor_pls_core::editor_window::{EditorWindow, EditorWindowContext};
use bevy_inspector_egui::{bevy_inspector::hierarchy::SelectedEntities, egui};
//use transform_gizmo_bevy::GizmoMode;

use crate::{
    cameras::{CameraWindow, EditorCamera, EDITOR_RENDER_LAYER},
    hierarchy::HierarchyWindow,
};

pub struct GizmoState {
    pub camera_gizmo_active: bool,
    //pub gizmo_mode: GizmoMode,
}

impl Default for GizmoState {
    fn default() -> Self {
        Self {
            camera_gizmo_active: true,
            //gizmo_mode: GizmoMode::Translate,
        }
    }
}

pub struct GizmoWindow;

impl EditorWindow for GizmoWindow {
    type State = GizmoState;

    const NAME: &'static str = "Gizmos";

    fn ui(_world: &mut World, _cx: EditorWindowContext, ui: &mut egui::Ui) {
        ui.label("Gizmos can currently not be configured");
    }

    fn viewport_toolbar_ui(_world: &mut World, cx: EditorWindowContext, _ui: &mut egui::Ui) {
        let gizmo_state = cx.state::<GizmoWindow>().unwrap();

        if gizmo_state.camera_gizmo_active {
            if let (Some(_hierarchy_state), Some(_camera_state)) =
                (cx.state::<HierarchyWindow>(), cx.state::<CameraWindow>())
            {
                //draw_gizmo(ui, world, &hierarchy_state.selected, ); //gizmo_state.gizmo_mode
            }
        }
    }

    fn app_setup(app: &mut App) {
        let mut materials = app.world_mut().resource_mut::<Assets<StandardMaterial>>();
        let material_light = materials.add(StandardMaterial {
            base_color: Color::srgba_u8(222, 208, 103, 255),
            unlit: true,
            fog_enabled: false,
            alpha_mode: AlphaMode::Add,
            ..default()
        });
        let material_camera = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 1.0),
            unlit: true,
            fog_enabled: false,
            alpha_mode: AlphaMode::Multiply,
            ..default()
        });

        let mut meshes = app.world_mut().resource_mut::<Assets<Mesh>>();
        let sphere = meshes.add(Sphere { radius: 0.3 });

        app.world_mut().insert_resource(GizmoMarkerConfig {
            point_light_mesh: sphere.clone(),
            point_light_material: material_light.clone(),
            directional_light_mesh: sphere.clone(),
            directional_light_material: material_light,
            camera_mesh: sphere,
            camera_material: material_camera,
        });

        app.add_systems(PostUpdate, add_gizmo_markers);
    }
}

#[derive(Resource)]
struct GizmoMarkerConfig {
    point_light_mesh: Handle<Mesh>,
    point_light_material: Handle<StandardMaterial>,
    directional_light_mesh: Handle<Mesh>,
    directional_light_material: Handle<StandardMaterial>,
    camera_mesh: Handle<Mesh>,
    camera_material: Handle<StandardMaterial>,
}

#[derive(Component)]
struct HasGizmoMarker;

type GizmoMarkerQuery<'w, 's, T, F = ()> =
    Query<'w, 's, Entity, (With<T>, Without<HasGizmoMarker>, F)>;

fn add_gizmo_markers(
    mut commands: Commands,
    gizmo_marker_meshes: Res<GizmoMarkerConfig>,

    point_lights: GizmoMarkerQuery<PointLight>,
    directional_lights: GizmoMarkerQuery<DirectionalLight>,
    cameras: GizmoMarkerQuery<Camera, Without<EditorCamera>>,
) {
    fn add<T: Component, F: QueryFilter, B: Bundle>(
        commands: &mut Commands,
        query: GizmoMarkerQuery<T, F>,
        name: &'static str,
        f: impl Fn() -> B,
    ) {
        let render_layers = RenderLayers::layer(EDITOR_RENDER_LAYER);
        for entity in &query {
            commands
                .entity(entity)
                .insert(HasGizmoMarker)
                .with_children(|commands| {
                    commands.spawn((f(), render_layers.clone(), Name::new(name)));
                });
        }
    }

    add(&mut commands, point_lights, "PointLight Gizmo", || {
        PbrBundle {
            mesh: gizmo_marker_meshes.point_light_mesh.clone_weak(),
            material: gizmo_marker_meshes.point_light_material.clone_weak(),
            ..default()
        }
    });
    add(
        &mut commands,
        directional_lights,
        "DirectionalLight Gizmo",
        || PbrBundle {
            mesh: gizmo_marker_meshes.directional_light_mesh.clone_weak(),
            material: gizmo_marker_meshes.directional_light_material.clone_weak(),
            ..default()
        },
    );

    let render_layers = RenderLayers::layer(EDITOR_RENDER_LAYER);
    for entity in &cameras {
        commands
            .entity(entity)
            .insert((
                HasGizmoMarker,
                Visibility::Visible,
                InheritedVisibility::VISIBLE,
                ViewVisibility::default(),
            ))
            .with_children(|commands| {
                commands.spawn((
                    PbrBundle {
                        mesh: gizmo_marker_meshes.camera_mesh.clone_weak(),
                        material: gizmo_marker_meshes.camera_material.clone_weak(),
                        ..default()
                    },
                    render_layers.clone(),
                    Name::new("Camera Gizmo"),
                ));
            });
    }
}

#[allow(dead_code)]
fn draw_gizmo(
    _ui: &mut egui::Ui,
    _world: &mut World,
    _selected_entities: &SelectedEntities,
    //_gizmo_mode: GizmoMode,

) {
    // let Ok((cam_transform, projection)) = world
    //     .query_filtered::<(&GlobalTransform, &Projection), With<ActiveEditorCamera>>()
    //     .get_single(world)
    // else {
    //     return;
    // };

    // let view_matrix = Mat4::from(cam_transform.affine().inverse());
    // let projection_matrix = projection.get_projection_matrix();

    // if selected_entities.len() != 1 {
    //     return;
    // }

    // for selected in selected_entities.iter() {
        // let Some(global_transform) = world.get::<GlobalTransform>(selected) else {
        //     continue;
        // };
        // let model_matrix = global_transform.compute_matrix();
        
        //new
        // let mut gizmo_options = world.resource_mut::<GizmoOptions>();
        // gizmo_options.
        // *gizmo_options.update_config(GizmoConfig {
        //     view_matrix: view_matrix.into(),
        //     projection_matrix: projection_matrix.into(),
        //     viewport,
        //     modes: self.gizmo_modes,
        //     orientation: self.gizmo_orientation,
        //     snapping,
        //     ..Default::default()
        // });

        // old
        // // let Some(result) = Gizmo::default()
        // //     .model_matrix(model_matrix.into())
        // //     .view_matrix(view_matrix.into())
        // //     .projection_matrix(projection_matrix.into())
        // //     .orientation(egui_gizmo::GizmoOrientation::Local)
        // //     .mode(gizmo_mode)
        // //     .interact(ui)
        // // else {
        // //     continue;
        // // };

        // let global_affine = global_transform.affine();

        // let mut transform = world.get_mut::<Transform>(selected).unwrap();

        // let parent_affine = global_affine * transform.compute_affine().inverse();
        // let inverse_parent_transform = GlobalTransform::from(parent_affine.inverse());

        // let global_transform = Transform {
        //     translation: result.translation.into(),
        //     rotation: result.rotation.into(),
        //     scale: result.scale.into(),
        // };

        // *transform = (inverse_parent_transform * global_transform).into();
    //}
}
