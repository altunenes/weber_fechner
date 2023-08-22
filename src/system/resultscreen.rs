use bevy::prelude::*;
use crate::state::experiment::AppState;
use crate::state::experiment::ExperimentState;



pub fn display_results_system(
    app_state: Res<AppState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    experiment_state: Res<ExperimentState>,
) {
    if *app_state == AppState::Results {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf"); 
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: Color::WHITE,
        };
        let text_alignment = TextAlignment::Center;

        let mut results_text = String::from("---Final Results---\n");
        let mut correct_count = 0;
        let mut correct_rt_sum = 0.0;
        let mut correct_rt_count = 0;

        for (_, (_, _, result, response_time)) in experiment_state.final_result.iter().enumerate() {
            if result == "Correct" {
                correct_count += 1;
                correct_rt_sum += *response_time;
                correct_rt_count += 1;
            }
        }

        let mean_accuracy: f32 = correct_count as f32 / experiment_state.final_result.len() as f32;
        let mean_correct_rt = if correct_rt_count > 0 {
            correct_rt_sum / correct_rt_count as f32
        } else {
            0.0
        }; 
        results_text += &format!("\nDone! Contact me for any suggestions/questions https://github.com/altunenes/weber_fechner");
    
        results_text += &format!("\n\nMean Accuracy: {}", mean_accuracy);
        results_text += &format!("\nMean Correct Response Time: {:.2}", mean_correct_rt);
        if experiment_state.final_result.len() > 10 { 
            results_text += "\n Detailed results saved as csv file.";
        } else {
            for (trial, (num_left, num_right, result, response_time)) in experiment_state.final_result.iter().enumerate() {
                results_text += &format!("\nTrial {}: Left = {}, Right = {}, Result = {}, Response Time = {}", trial+1, num_left, num_right, result, response_time);
            }
        }
        commands.spawn(Text2dBundle {
            text: Text::from_section(&results_text, text_style)
                .with_alignment(text_alignment), 
            ..Default::default()
        });
    }
}
