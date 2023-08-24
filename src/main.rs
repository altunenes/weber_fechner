use bevy::prelude::*;
use bevy_egui::EguiPlugin;



mod setup;
use setup::setup_camera;
mod state;
mod system;
use crate::state::experiment::AppState;
use state::experiment::ExperimentState;
use state::experiment::Radius;
use state::experiment::MinEllipse;
use state::experiment::MaxEllipse;
use state::experiment::TotalTrial;
use state::experiment::TrialState;
use state::experiment::FixationTimer;
use state::experiment::EllipseColor;
use crate::state::experiment::ExperimentBackgroundColor;
use crate::state::experiment::FixationBackgroundColor;
use crate::state::experiment::DrawingMethod;
use crate::state::experiment::CurrentDrawingMethod;






use system::instruction::display_instruction_system;
use system::fixation::transition_from_fixation_system;
use system::resultscreen::display_results_system;
use system::fixation::display_fixation_system;
use system::update::remove_text_system;
use system::update::refresh_ellipses;
use system::update::update_background_color_system;
use system::update::update_user_responses;
use system::start::start_experiment_system;


pub const PARTICIPANT_ID: &str = "1";




fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }), 
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .insert_resource(AppState::Instruction)
        .insert_resource(ExperimentState::default())
        .insert_resource(DrawingMethod::Uniform)
        .insert_resource(TotalTrial::default())
        .insert_resource(Radius::default())
        .insert_resource(MinEllipse::default())
        .insert_resource(MaxEllipse::default())
        .insert_resource(TrialState::default()) 
        .insert_resource(FixationTimer::default())
        .insert_resource(EllipseColor::default())
        .insert_resource(CurrentDrawingMethod::default())
        .insert_resource(ExperimentBackgroundColor::default())
        .insert_resource(FixationBackgroundColor::default())
        .add_systems(Startup, setup_camera)
        .add_systems(Update, remove_text_system.before(display_instruction_system))
        .add_systems(Update, display_instruction_system)
        .add_systems(Update, start_experiment_system.after(display_instruction_system))
        .add_systems(Update, display_fixation_system)
        .add_systems(Update, transition_from_fixation_system)
        .add_systems(Update, update_background_color_system)
        .add_systems(Update, update_user_responses) 
        .add_systems(Update, refresh_ellipses.after(update_user_responses)) 
        .add_systems(Update, display_results_system)
        

        .run();
}




