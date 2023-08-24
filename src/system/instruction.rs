
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::Widget;
use bevy::prelude::*;
use crate::state::experiment::Radius;
use crate::state::experiment::MinEllipse;
use crate::state::experiment::MaxEllipse;
use crate::state::experiment::AppState;
use crate::state::experiment::TotalTrial;
use crate::state::experiment::EllipseColor;
use crate::state::experiment::ExperimentBackgroundColor;
use crate::state::experiment::FixationBackgroundColor;
use crate::state::experiment::DrawingMethod;
use crate::state::experiment::CurrentDrawingMethod;


pub fn display_instruction_system(
    app_state: Res<AppState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut contexts: EguiContexts,
    mut total_trial: ResMut<TotalTrial>, 
    mut radius: ResMut<Radius>,
    mut min_ellipse: ResMut<MinEllipse>,
    mut max_ellipse: ResMut<MaxEllipse>,
    mut ellipse_color_resource: ResMut<EllipseColor>,
    mut experiment_background_color: ResMut<ExperimentBackgroundColor>,
    mut fixation_background_color: ResMut<FixationBackgroundColor>,
    mut current_drawing_method: ResMut<CurrentDrawingMethod>,


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

            ui.horizontal(|ui| {
                ui.label("ELLIPSE COLOR:");
                color_picker_widget(ui, &mut ellipse_color_resource.0);
            });
            ui.horizontal(|ui| {
                ui.label("EXPERIMENT BG COLOR:");
                color_picker_widget(ui, &mut experiment_background_color.0);
            });
            
            ui.horizontal(|ui| {
                ui.label("FIXATION BG COLOR:");
                color_picker_widget(ui, &mut fixation_background_color.0);
            });
            ui.label(format!("Current drawing method: {:?}", current_drawing_method.0));
            if ui.button("Next drawing method").clicked() {
                current_drawing_method.0 = match current_drawing_method.0 {
                    DrawingMethod::Uniform => DrawingMethod::Grid,
                    DrawingMethod::Grid => DrawingMethod::Circular,
                    DrawingMethod::Circular => DrawingMethod::Spiral,
                    DrawingMethod::Spiral => DrawingMethod::Gaussian,
                    DrawingMethod::Gaussian => DrawingMethod::Uniform,
                };
            }

    

        });
    }
}


fn color_picker_widget(ui: &mut egui::Ui, color: &mut Color) -> egui::Response {
    let [r, g, b, a] = color.as_rgba_f32();
    let mut egui_color: egui::Rgba = egui::Rgba::from_srgba_unmultiplied(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    );
    let res = egui::widgets::color_picker::color_edit_button_rgba(
        ui,
        &mut egui_color,
        egui::color_picker::Alpha::Opaque,
    );
    let [r, g, b, a] = egui_color.to_srgba_unmultiplied();
    *color = [
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    ]
    .into();
    res
}