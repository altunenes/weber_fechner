use bevy::prelude::*;
use crate::state::experiment::AppState;
use crate::state::experiment::ExperimentState;
use crate::state::experiment::Radius;
use crate::state::experiment::MinEllipse;
use crate::state::experiment::MaxEllipse;
use crate::state::experiment::FixationTimer;
use crate::state::experiment::EllipseColor;

use crate::setup::setup;



pub fn transition_from_fixation_system(
    mut app_state: ResMut<AppState>,
    mut fixation_timer: ResMut<FixationTimer>,
    time: Res<Time>,
    mut commands: Commands,
    text_query: Query<Entity, With<Text>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    experiment_state: ResMut<ExperimentState>,
    radius: Res<Radius>,
    min_ellipse: Res<MinEllipse>,
    max_ellipse: Res<MaxEllipse>,
    ellipse_color_resource: ResMut<EllipseColor>,
) {
    if *app_state == AppState::Fixation {
        if fixation_timer.timer.tick(time.delta()).just_finished() {
            for entity in text_query.iter() {
                commands.entity(entity).despawn();
            }
            fixation_timer.timer.reset();
            if !experiment_state.ellipses_drawn {
                setup(commands, meshes, materials, experiment_state,radius,min_ellipse,max_ellipse,ellipse_color_resource);    
            }
            *app_state = AppState::Experiment;
        }
    }
}


pub fn display_fixation_system(
    app_state: Res<AppState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if *app_state == AppState::Fixation {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf"); 
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 150.0,
            color: Color::WHITE,
        };
        let text_alignment = TextAlignment::Center;
        commands.spawn(Text2dBundle {
            text: Text::from_section("+", text_style)
                .with_alignment(text_alignment), 
            ..Default::default()
        });
    }
}
