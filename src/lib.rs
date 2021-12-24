use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub struct OrthoCameraPlugin;

impl Plugin for OrthoCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system(camera_movement)
            .add_system(camera_zoom);
    }
}

fn camera_zoom(
    mut query: Query<(&PanCam, &mut OrthographicProjection)>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    let pixels_per_line = 100.; // Maybe make configurable?
    let scroll = scroll_events
        .iter()
        .map(|ev| match ev.unit {
            MouseScrollUnit::Pixel => ev.y,
            MouseScrollUnit::Line => ev.y * pixels_per_line,
        })
        .sum::<f32>();

    if scroll == 0. {
        return;
    }

    for (_cam, mut projection) in query.iter_mut() {
        projection.scale = (projection.scale * (1. + -scroll * 0.001)).max(0.00001);
    }
}

fn camera_movement(
    mut windows: ResMut<Windows>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut query: Query<(&PanCam, &mut Transform, &OrthographicProjection)>,
    mut last_pos: Local<Option<Vec2>>,
) {
    let window = windows.get_primary_mut().unwrap();

    // Use position instead of MouseMotion, otherwise we don't get acceleration movement
    let current_pos = match window.cursor_position() {
        Some(current_pos) => current_pos,
        None => return,
    };
    let delta = current_pos - last_pos.unwrap_or(current_pos);

    for (cam, mut transform, projection) in query.iter_mut() {
        if cam
            .grab_buttons
            .iter()
            .any(|btn| mouse_buttons.pressed(*btn))
        {
            let scaling = Vec2::new(
                window.width() / (projection.right - projection.left),
                window.height() / (projection.top - projection.bottom),
            ) * projection.scale;

            transform.translation -= (delta * scaling).extend(0.);
        }
    }
    *last_pos = Some(current_pos);
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PanCam {
            ..Default::default()
        });
}

#[derive(Component, Debug)]
pub struct PanCam {
    grab_buttons: Vec<MouseButton>,
}

impl Default for PanCam {
    fn default() -> Self {
        Self {
            grab_buttons: vec![MouseButton::Left, MouseButton::Right, MouseButton::Middle],
        }
    }
}
