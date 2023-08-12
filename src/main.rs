use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Uniform};
use std::fs::OpenOptions;
use std::io::Write;
use std::process;
use instant::Instant;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }), 
            ..default()
        }))
        .insert_resource(AppState::Instruction)
        .insert_resource(ExperimentState::default())
        .insert_resource(TrialState::default()) 
        .add_systems(Startup, setup_camera)
        .add_systems(Update, remove_instruction_system.before(display_instruction_system))
        .add_systems(Update, display_instruction_system)
        .add_systems(Update, start_experiment_system.after(display_instruction_system))
        .add_systems(Update, update_background_color_system)
        .add_systems(Update, refresh_ellipses)
        .add_systems(Update, update_user_responses)
        .run();
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn remove_instruction_system(
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
fn refresh_ellipses(
    app_state: ResMut<AppState>, 
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    experiment_state: ResMut<ExperimentState>,
    mut trial_state: ResMut<TrialState>,
    ellipses: Query<Entity, With<Ellipse>>,
) {
    if !experiment_state.complete && (keys.just_pressed(KeyCode::Key1) || keys.just_pressed(KeyCode::Key0) || keys.just_pressed(KeyCode::Space)) {
        for entity in ellipses.iter() {
            commands.entity(entity).despawn();
        }
        trial_state.start_time = Instant::now();

        setup(commands, meshes, materials, experiment_state);
    }
    if *app_state != AppState::Experiment {
        return;
    }

}
#[derive(Resource)]
struct TrialState {
    start_time: Instant,
}
impl Default for TrialState {
    fn default() -> Self {
        TrialState {
            start_time: Instant::now(),
        }
    }
}
#[derive(Debug,PartialEq,Resource)]
enum AppState {
    Instruction,
    Experiment,
}

fn start_experiment_system(
    keys: Res<Input<KeyCode>>,
    mut app_state: ResMut<AppState>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut experiment_state: ResMut<ExperimentState>,
    text_query: Query<Entity, With<Text>>,
) {
    if *app_state == AppState::Instruction && keys.just_pressed(KeyCode::Return) {
        *app_state = AppState::Experiment;
        for entity in text_query.iter() {
            commands.entity(entity).despawn();
        }
        setup(commands, meshes, materials, experiment_state);    }
}
fn update_background_color_system(app_state: Res<AppState>, mut clear_color: ResMut<ClearColor>) {
    match *app_state {
        AppState::Instruction => {
            clear_color.0 = Color::rgb(0.1, 0.5, 0.5);
        }
        AppState::Experiment => {
            clear_color.0 = Color::GRAY;
        }
    }
}
#[derive(Component)]
struct Ellipse;
#[derive(Default, Resource)]
struct ExperimentState {
    final_result: Vec<(usize, usize, String, f32)>, 
    num_ellipses_left: usize,
    num_ellipses_right: usize,
    num_trials: usize, 
    complete: bool,
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
    let num_ellipses_1 = rng.gen_range(1..4);
    let num_ellipses_2 = rng.gen_range(1..4);
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

fn display_instruction_system(
    app_state: Res<AppState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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

            Press 'Enter' to start the experiment.
            Note: You have 5 trials to complete the experiment.
            questions/comments: enesaltun2@gmail.com
            
            ", text_style)
                .with_alignment(text_alignment), 
            ..Default::default()
        });
    }
}
fn update_user_responses(
    keys: Res<Input<KeyCode>>,
    mut experiment_state: ResMut<ExperimentState>,
    trial_state: Res<TrialState>,
) {
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
        if experiment_state.num_trials == 5 {
            print_final_results(&experiment_state.final_result);
        }
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
        if experiment_state.num_trials == 5 {
            print_final_results(&experiment_state.final_result);
        }
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
        if experiment_state.num_trials == 5 {
            print_final_results(&experiment_state.final_result);
        }
    }
    if experiment_state.num_trials == 5 {
        print_final_results(&experiment_state.final_result);
        experiment_state.complete = true;
    }
}
fn print_final_results(final_results: &Vec<(usize, usize, String, f32)>) {
    println!("---Final Results---");
    let mut csv_data = String::from("Trial,Num_Left,Num_Right,Result,Response_Time\n");
    let mut correct_count = 0;
    for (trial, (num_left, num_right, result, response_time)) in final_results.iter().enumerate() {
        if result == "Correct" {
            correct_count += 1;
        }
        println!("Trial {}: Left = {}, Right = {}, Result = {}, Response Time = {}", trial+1, num_left, num_right, result, response_time);
        csv_data += &format!("{},{},{},{},{}\n", trial+1, num_left, num_right, result, response_time);
    }
    let mean_accuracy: f32 = correct_count as f32 / final_results.len() as f32;
    let mean_correct_rt: f32 = final_results
    .iter()
    .filter(|(_, _, is_correct, _)| is_correct == "Correct") 
    .map(|(_, _, _, response_time)| response_time)
    .sum::<f32>() / final_results.len() as f32;
    println!("Mean Accuracy: {}", mean_accuracy);
    csv_data += &format!("\nMean Accuracy: {}\n", mean_accuracy);
    println!("Mean Correct Response Time: {}", mean_correct_rt);
    csv_data += &format!("Mean Correct Response Time: {}\n", mean_correct_rt);
    let file_name = format!("participant_{}.csv", 2); // change 1 with participant number

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsValue;
        use wasm_bindgen::JsCast;
        use web_sys::{Blob, Url, HtmlAnchorElement};
        let csv_array = js_sys::Array::new();
        csv_array.push(&JsValue::from_str(&csv_data));
        let blob = Blob::new_with_str_sequence(&csv_array).unwrap();
        
        let url = Url::create_object_url_with_blob(&blob).unwrap();
        let document = web_sys::window().unwrap().document().unwrap();
        let a: HtmlAnchorElement = document.create_element("a").unwrap().dyn_into().unwrap();
        a.set_href(&url);
        a.set_download(&file_name);
        a.style().set_property("display", "none").unwrap();
        document.body().unwrap().append_child(&a).unwrap();
        a.click();
        document.body().unwrap().remove_child(&a).unwrap();
        Url::revoke_object_url(&url).unwrap();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_name)
            .unwrap();
        file.write_all(csv_data.as_bytes()).unwrap();
        println!("Data saved to {}", &file_name);
    }

    process::exit(0);
}
