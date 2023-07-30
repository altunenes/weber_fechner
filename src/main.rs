use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{Rng, thread_rng, seq::SliceRandom};
use bevy::input::keyboard::KeyboardInput;
use rand::distributions::{Distribution, Uniform};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ExperimentState::default())
        .add_systems(Startup, setup)

        .run();
}
// Components
struct Dot;
struct UserResponse(Option<bool>);

// Resources
#[derive(Default, Resource)]
struct ExperimentState {
    final_result: Vec<(usize, usize)>,
    num_ellipses_left: usize,
    num_ellipses_right: usize,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut experiment_state: ResMut<ExperimentState>,
) {
    commands.spawn(Camera2dBundle::default());

    let mut rng = thread_rng();
    let x = -450.0;
    let y_range = Uniform::new(-200.0, 200.0);
    let x_2= 450.0;
    let y_range_2 = Uniform::new(-200.0, 200.0);
    let num_ellipses_1 = rng.gen_range(10..40);
    let num_ellipses_2 = rng.gen_range(40..80);

    experiment_state.num_ellipses_left = num_ellipses_1;
    experiment_state.num_ellipses_right = num_ellipses_2;

    for i in 0..num_ellipses_1 {
        let y = y_range.sample(&mut rng);
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(2.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(x + i as f32 * 2., y, 0.)),
            ..default()
        });
    }
    for i in 0..num_ellipses_2{
        let y_2: f32 = y_range_2.sample(&mut rng);
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(2.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(x_2 + i as f32 * 2., y_2, 0.)),
            ..default()
        });
    }
}