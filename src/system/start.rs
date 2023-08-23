
use bevy::prelude::*;
use instant::Instant;
use crate::state::experiment::AppState;
use crate::state::experiment::ExperimentState;
use crate::state::experiment::Radius;
use crate::state::experiment::MinEllipse;
use crate::state::experiment::MaxEllipse;
use crate::state::experiment::TrialState;
use crate::state::experiment::EllipseColor;

use crate::setup::setup;



pub fn start_experiment_system(
    keys: Res<Input<KeyCode>>,
    mut app_state: ResMut<AppState>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    experiment_state: ResMut<ExperimentState>,
    text_query: Query<Entity, With<Text>>,
    mut trial_state: ResMut<TrialState>, 
    radius: Res<Radius>,
    min_ellipse: Res<MinEllipse>,
    max_ellipse: Res<MaxEllipse>,
    ellipse_color_resource: ResMut<EllipseColor>,
) {
    if *app_state == AppState::Instruction && keys.just_pressed(KeyCode::Return) {
        *app_state = AppState::Experiment;
        for entity in text_query.iter() {
            commands.entity(entity).despawn();
        }
        trial_state.start_time = Instant::now(); 
        setup(commands, meshes, materials, experiment_state,radius,min_ellipse,max_ellipse,ellipse_color_resource);    
    }
}

