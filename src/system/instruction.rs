
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Widget;
use bevy::prelude::*;
use crate::state::experiment::Radius;
use crate::state::experiment::MinEllipse;
use crate::state::experiment::MaxEllipse;
use crate::state::experiment::AppState;
use crate::state::experiment::TotalTrial;

pub fn display_instruction_system(
    app_state: Res<AppState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut contexts: EguiContexts,
    mut total_trial: ResMut<TotalTrial>, 
    mut radius: ResMut<Radius>,
    mut min_ellipse: ResMut<MinEllipse>,
    mut max_ellipse: ResMut<MaxEllipse>,


) {
    if *app_state == AppState::Instruction {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf"); 
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: Color::WHITE,
        };
        let text_alignment = TextAlignment::Center;
        commands.spawn(Text2dBundle {
            text: Text::from_section("Wellcome =) \n\n\
            this project is currently under development this is just a short demo to check if everything is working fine. 

            Instructions:
            If you see more ellipses on the left click to '1', 
            if you see more ellipses on the right click to '0', 
            if you see the same number of ellipses on both sides click to 
            'Space'.

            Set number of trials from the slider.

            Press 'Enter' to start the experiment.

            Please make sure you are in full screen mode.

            questions/comments: enesaltun2@gmail.com            
            ", text_style)
                .with_alignment(text_alignment), 
            ..Default::default()
        });
    }
    if *app_state == AppState::Instruction {
        egui::Window::new("Experiment Settings").show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("TOTAL_TRIAL:");
                let mut trial_value = total_trial.0 as f32;
                egui::Slider::new(&mut trial_value, 1.0..=100.0).ui(ui);
                total_trial.0 = trial_value as usize;
            });
            ui.horizontal(|ui| {
                ui.label("RADIUS:");
                let mut radius_value = radius.0;
                egui::Slider::new(&mut radius_value, 1.0..=5.0).ui(ui);
                radius.0 = radius_value;
            });
            ui.horizontal(|ui| {
                ui.label("MIN_ELLIPSE:");
                let mut min_ellipse_value = min_ellipse.0 as f32;
                egui::Slider::new(&mut min_ellipse_value, 1.0..=10.0).ui(ui);
                min_ellipse.0 = min_ellipse_value as usize;
            });

            ui.horizontal(|ui| {
                ui.label("MAX_ELLIPSE:");
                let mut max_ellipse_value = max_ellipse.0 as f32;
                egui::Slider::new(&mut max_ellipse_value, 1.0..=100.0).ui(ui);
                max_ellipse.0 = max_ellipse_value as usize;
            });
            
        });
    }
}