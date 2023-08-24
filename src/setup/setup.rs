use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Uniform};
use crate::state::experiment::ExperimentState;
use crate::state::experiment::Ellipse;
use crate::state::experiment::Radius;
use crate::state::experiment::MinEllipse;
use crate::state::experiment::MaxEllipse;
use crate::state::experiment::EllipseColor;
use crate::state::experiment::DrawingMethod;
use crate::state::experiment::CurrentDrawingMethod;
use rand_distr::Normal;


pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut experiment_state: ResMut<ExperimentState>,
    radius: Res<Radius>,
    min_ellipse: Res<MinEllipse>,
    max_ellipse: Res<MaxEllipse>,
    ellipse_color_resource: ResMut<EllipseColor>,
    current_drawing_method: Res<CurrentDrawingMethod> 
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

    match current_drawing_method.0 {
        DrawingMethod::Uniform => {
            for i in 0..num_ellipses_1 {
                let y = y_range.sample(&mut rng);
                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
                    material: materials.add(ColorMaterial::from(ellipse_color_resource.0)),
                    transform: Transform::from_translation(Vec3::new(x + i as f32 * 2., y, 0.)),
                    ..default()
                }).insert(Ellipse);
            }
            for i in 0..num_ellipses_2 {
                let y_2: f32 = y_range_2.sample(&mut rng);
                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
                    material: materials.add(ColorMaterial::from(ellipse_color_resource.0)),
                    transform: Transform::from_translation(Vec3::new(x_2 + i as f32 * 2., y_2, 0.)),
                    ..default()
                }).insert(Ellipse);
            }
        },
        DrawingMethod::Grid => {
    let horizontal_spacing = 2.0 * radius.0 + 10.0; 
    let vertical_spacing = 2.0 * radius.0 + 10.0; 

    let max_ellipses_horizontal = (900.0 / horizontal_spacing).floor() as usize; 
    let max_ellipses_vertical = (400.0 / vertical_spacing).floor() as usize; 

    let mut left_positions = Vec::new();
    'left_loop: for i in 0..max_ellipses_horizontal {
        for j in 0..max_ellipses_vertical {
            if left_positions.len() >= num_ellipses_1 {
                break 'left_loop;
            }
            let pos_x = x + i as f32 * horizontal_spacing;
            let pos_y = -200.0 + j as f32 * vertical_spacing;
            left_positions.push((pos_x, pos_y));
        }
    }

    let mut right_positions = Vec::new();
    'right_loop: for i in 0..max_ellipses_horizontal {
        for j in 0..max_ellipses_vertical {
            if right_positions.len() >= num_ellipses_2 {
                break 'right_loop;
            }
            let pos_x = x_2 + i as f32 * horizontal_spacing;
            let pos_y = -200.0 + j as f32 * vertical_spacing;
            right_positions.push((pos_x, pos_y));
        }
    }

    for (pos_x, pos_y) in left_positions {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
            material: materials.add(ColorMaterial::from(ellipse_color_resource.0)),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.)),
            ..default()
        }).insert(Ellipse);
    }

    for (pos_x, pos_y) in right_positions {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
            material: materials.add(ColorMaterial::from(ellipse_color_resource.0)),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.)),
            ..default()
        }).insert(Ellipse);
    }
},
DrawingMethod::Circular => {
    let circle_spacing = 2.0 * radius.0 + 10.0;
    let generate_positions = |num_ellipses, x_offset| {
        let mut positions = Vec::new();
        let mut current_radius = circle_spacing;
        while positions.len() < num_ellipses {
            let circumference = 2.0 * std::f32::consts::PI * current_radius;
            let num_ellipses_at_this_radius = (circumference / circle_spacing).floor() as usize;
            for i in 0..num_ellipses_at_this_radius {
                if positions.len() >= num_ellipses {
                    break;
                }
                let angle = i as f32 * (2.0 * std::f32::consts::PI / num_ellipses_at_this_radius as f32);
                let pos_x = x_offset + current_radius * angle.cos();
                let pos_y = current_radius * angle.sin();
                positions.push((pos_x, pos_y));
            }

            current_radius += circle_spacing;
        }
        positions
    };
    let left_positions = generate_positions(num_ellipses_1, x);
    let right_positions = generate_positions(num_ellipses_2, x_2);
    for (pos_x, pos_y) in left_positions {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
            material: materials.add(ColorMaterial::from(ellipse_color_resource.0)),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.)),
            ..default()
        }).insert(Ellipse);
    }
    for (pos_x, pos_y) in right_positions {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
            material: materials.add(ColorMaterial::from(ellipse_color_resource.0)),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.)),
            ..default()
        }).insert(Ellipse);
    }
},

DrawingMethod::Spiral => {
    let a = 0.0;
    let b = 2.0;
    let generate_positions = |num_ellipses, x_offset| {
        let mut positions = Vec::new();
        let mut theta: f32 = 0.0;
        let step = 0.5;
        while positions.len() < num_ellipses {
            let r = a + b * theta;
            let pos_x = x_offset + r * theta.cos();
            let pos_y = r * theta.sin();
            positions.push((pos_x, pos_y));
            theta += step;
        }
        positions
    };

    let left_positions = generate_positions(num_ellipses_1, x);
    let right_positions = generate_positions(num_ellipses_2, x_2);
    for (pos_x, pos_y) in left_positions {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
            material: materials.add(ColorMaterial::from(ellipse_color_resource.0)),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.)),
            ..default()
        }).insert(Ellipse);
    }

    for (pos_x, pos_y) in right_positions {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
            material: materials.add(ColorMaterial::from(ellipse_color_resource.0)),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.)),
            ..default()
        }).insert(Ellipse);
    }
},

DrawingMethod::Gaussian => {
    let mean = 0.0;
    let standard_deviation = 100.0;
    let normal = Normal::new(mean, standard_deviation).unwrap();

    let mut generate_positions = |num_ellipses, x_offset| {
        let mut positions = Vec::new();
        for _ in 0..num_ellipses {
            let pos_x = x_offset + normal.sample(&mut rng);
            let pos_y = normal.sample(&mut rng);
            positions.push((pos_x, pos_y));
        }
        positions
    };

    let left_positions = generate_positions(num_ellipses_1, x);
    let right_positions = generate_positions(num_ellipses_2, x_2);
    for (pos_x, pos_y) in left_positions {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
            material: materials.add(ColorMaterial::from(ellipse_color_resource.0)),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.)),
            ..default()
        }).insert(Ellipse);
    }

    for (pos_x, pos_y) in right_positions {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
            material: materials.add(ColorMaterial::from(ellipse_color_resource.0)),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.)),
            ..default()
        }).insert(Ellipse);
    }
},

DrawingMethod::Phyllotaxis => {
    let golden_angle = 137.5_f32.to_radians();
    let distance = 15.0;

    let generate_positions = |num_ellipses, x_offset| {
        let mut positions = Vec::new();
        for i in 0..num_ellipses {
            let theta = i as f32 * golden_angle;
            let r = distance * (i as f32).sqrt();
            let pos_x = x_offset + r * theta.cos();
            let pos_y = r * theta.sin();
            positions.push((pos_x, pos_y));
        }
        positions
    };

    let left_positions = generate_positions(num_ellipses_1, x);
    let right_positions = generate_positions(num_ellipses_2, x_2);
    for (pos_x, pos_y) in left_positions {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
            material: materials.add(ColorMaterial::from(ellipse_color_resource.0)),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.)),
            ..default()
        }).insert(Ellipse);
    }

    for (pos_x, pos_y) in right_positions {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius.0).into()).into(),
            material: materials.add(ColorMaterial::from(ellipse_color_resource.0)),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.)),
            ..default()
        }).insert(Ellipse);
    }
},
    }
    
    experiment_state.ellipses_drawn = true;
}



pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}