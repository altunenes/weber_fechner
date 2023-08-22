use bevy::prelude::*;

use crate::state::experiment::AppState;
use crate::state::experiment::ExperimentState;
use crate::state::experiment::Ellipse;
use crate::state::experiment::TrialState;
use instant::Instant;
use crate::state::experiment::TotalTrial;
use crate::system::output::print_final_results;


pub fn remove_text_system(
    app_state: Res<AppState>,
    mut commands: Commands,
    text_query: Query<Entity, With<Text>>,
) {
    if *app_state == AppState::Experiment {
        for entity in text_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
pub fn refresh_ellipses(
    app_state: ResMut<AppState>, 
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut experiment_state: ResMut<ExperimentState>,
    mut trial_state: ResMut<TrialState>,
    ellipses: Query<Entity, With<Ellipse>>,
) {
    if experiment_state.ellipses_drawn && (keys.just_pressed(KeyCode::Key1) || keys.just_pressed(KeyCode::Key0) || keys.just_pressed(KeyCode::Space)) {
        for entity in ellipses.iter() {
            commands.entity(entity).despawn();
        }
        experiment_state.ellipses_drawn = false;
        trial_state.start_time = Instant::now();
    }
    if *app_state != AppState::Experiment {
        return;
    }
}


pub fn update_background_color_system(app_state: Res<AppState>, mut clear_color: ResMut<ClearColor>) {
    match *app_state {
        AppState::Instruction => {
            clear_color.0 = Color::rgb(0.1, 0.5, 0.5);
        }
        AppState::Experiment => {
            clear_color.0 = Color::GRAY;
        }
        AppState::Fixation => {
            clear_color.0 = Color::GRAY;
        }
        AppState::Results => {
            clear_color.0 = Color::rgb(0.1, 0.5, 0.5);
        }
    }
}



pub fn update_user_responses(
    keys: Res<Input<KeyCode>>,
    mut experiment_state: ResMut<ExperimentState>,
    trial_state: Res<TrialState>,
    mut app_state: ResMut<AppState>,
    total_trial: Res<TotalTrial>,
) {
    if *app_state != AppState::Experiment {
        return;
    }

    if keys.just_pressed(KeyCode::Key1) {
        let num_left = experiment_state.num_ellipses_left;
        let num_right = experiment_state.num_ellipses_right;
        let elapsed = trial_state.start_time.elapsed().as_secs_f32();
        if num_left > num_right {
            experiment_state.final_result.push((num_left, num_right, "Correct".to_string(), elapsed));
        } else {
            experiment_state.final_result.push((num_left, num_right, "Incorrect".to_string(), elapsed));
        }
        experiment_state.num_trials += 1;
        *app_state = AppState::Fixation;
    }
    if keys.just_pressed(KeyCode::Key0) {
        let num_left = experiment_state.num_ellipses_left;
        let num_right = experiment_state.num_ellipses_right;
        let elapsed = trial_state.start_time.elapsed().as_secs_f32();
        if num_left < num_right {
            experiment_state.final_result.push((num_left, num_right, "Correct".to_string(), elapsed));
        } else {
            experiment_state.final_result.push((num_left, num_right, "Incorrect".to_string(), elapsed));
        }
        experiment_state.num_trials += 1;
        if experiment_state.num_trials == total_trial.0 {
            print_final_results(&experiment_state.final_result);
        }
        *app_state = AppState::Fixation;
    }
    if keys.just_pressed(KeyCode::Space) {
        let num_left = experiment_state.num_ellipses_left;
        let num_right = experiment_state.num_ellipses_right;
        let elapsed = trial_state.start_time.elapsed().as_secs_f32();
        if num_left == num_right {
            experiment_state.final_result.push((num_left, num_right, "Correct".to_string(), elapsed));
        } else {
            experiment_state.final_result.push((num_left, num_right, "Incorrect".to_string(), elapsed));
        }
        experiment_state.num_trials += 1;
        if experiment_state.num_trials == total_trial.0 {
            print_final_results(&experiment_state.final_result);
        }
        *app_state = AppState::Fixation;

        
    }
    if experiment_state.num_trials == total_trial.0 && !experiment_state.complete {
        print_final_results(&experiment_state.final_result);
        experiment_state.complete = true; 
        *app_state = AppState::Results;
    }

    if *app_state == AppState::Results{
        return;
    }

}
