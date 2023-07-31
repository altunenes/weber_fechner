use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{Rng, thread_rng, seq::SliceRandom};
use bevy::input::keyboard::KeyboardInput;
use rand::distributions::{Distribution, Uniform};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ExperimentState::default())
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup)
        .add_systems(Update, refresh_ellipses)
        .add_systems(Update, update_user_responses)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn refresh_ellipses(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut experiment_state: ResMut<ExperimentState>,
    ellipses: Query<Entity, With<Ellipse>>,
) {
    if keys.just_pressed(KeyCode::S) || keys.just_pressed(KeyCode::D) {
        // Despawn the existing ellipses
        for entity in ellipses.iter() {
            commands.entity(entity).despawn();
        }
        // Re-setup the ellipses
        setup(commands, meshes, materials, experiment_state);
    }
}
#[derive(Component)]
struct Ellipse;
struct UserResponse(Option<bool>);
#[derive(Default, Resource)]
struct ExperimentState {
    final_result: Vec<(usize, usize, bool)>, // true for correct, false for incorrect S is same, D is not same 
    num_ellipses_left: usize,
    num_ellipses_right: usize,
    num_trials: usize, 

}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut experiment_state: ResMut<ExperimentState>,
) {
    let mut rng = thread_rng();
    let x = -450.0;
    let y_range = Uniform::new(-200.0, 200.0);
    let x_2= 450.0;
    let y_range_2 = Uniform::new(-200.0, 200.0);
    let num_ellipses_1 = rng.gen_range(1..2);
    let num_ellipses_2 = rng.gen_range(1..2);
    experiment_state.num_ellipses_left = num_ellipses_1;
    experiment_state.num_ellipses_right = num_ellipses_2;
    for i in 0..num_ellipses_1 {
        let y = y_range.sample(&mut rng);
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(2.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(x + i as f32 * 2., y, 0.)),
            ..default()
        }).insert(Ellipse);
    }
    for i in 0..num_ellipses_2{
        let y_2: f32 = y_range_2.sample(&mut rng);
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(2.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(x_2 + i as f32 * 2., y_2, 0.)),
            ..default()
        }).insert(Ellipse);
    }
}
fn update_user_responses(
    keys: Res<Input<KeyCode>>,
    mut experiment_state: ResMut<ExperimentState>,
) {
    if keys.just_pressed(KeyCode::S) {
        let num_left = experiment_state.num_ellipses_left;
        let num_right = experiment_state.num_ellipses_right;

        if num_left == num_right {
            experiment_state.final_result.push((num_left, num_right, true));
        } else {
            experiment_state.final_result.push((num_left, num_right, false));
        }

        experiment_state.num_trials += 1;
        if experiment_state.num_trials == 20 {
            print_final_results(&experiment_state.final_result);
        }
    }

    if keys.just_pressed(KeyCode::D) {
        let num_left = experiment_state.num_ellipses_left;
        let num_right = experiment_state.num_ellipses_right;

        if num_left != num_right {
            experiment_state.final_result.push((num_left, num_right, true));
        } else {
            experiment_state.final_result.push((num_left, num_right, false));
        }

        experiment_state.num_trials += 1;
        if experiment_state.num_trials == 20 {
            print_final_results(&experiment_state.final_result);
        }
    }
}
fn print_final_results(final_results: &Vec<(usize, usize, bool)>) {
    println!("---Final Results---");
    for (trial, (num_left, num_right, is_correct)) in final_results.iter().enumerate() {
        let correctness = if *is_correct {"Correct"} else {"Incorrect"};
        println!("Trial {}: Left = {}, Right = {}, Result = {}", trial+1, num_left, num_right, correctness);
    }
}