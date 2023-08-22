use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Uniform};
use crate::state::experiment::ExperimentState;
use crate::state::experiment::Ellipse;
use crate::state::experiment::Radius;
use crate::state::experiment::MinEllipse;
use crate::state::experiment::MaxEllipse;


pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut experiment_state: ResMut<ExperimentState>,
    radius: Res<Radius>,
    min_ellipse: Res<MinEllipse>,
    max_ellipse: Res<MaxEllipse>,
) {
    let mut rng = thread_rng();
    let x = -450.0;
    let y_range = Uniform::new(-200.0, 200.0);
    let x_2= 450.0;
    let y_range_2 = Uniform::new(-200.0, 200.0);
    let num_ellipses_1 = rng.gen_range(min_ellipse.0..max_ellipse.0);
    let num_ellipses_2 = rng.gen_range(min_ellipse.0..max_ellipse.0);
    experiment_state.num_ellipses_left = num_ellipses_1;
    experiment_state.num_ellipses_right = num_ellipses_2;
    for i in 0..num_ellipses_1 {
        let y = y_range.sample(&mut rng);
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(x + i as f32 * 2., y, 0.)),
            ..default()
        }).insert(Ellipse);

    }
    for i in 0..num_ellipses_2{
        let y_2: f32 = y_range_2.sample(&mut rng);
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(x_2 + i as f32 * 2., y_2, 0.)),
            ..default()
        }).insert(Ellipse);
    }
    experiment_state.ellipses_drawn = true;

}



pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}