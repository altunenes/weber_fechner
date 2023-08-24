
use instant::Instant;
use bevy::{prelude::*, time::Timer};

#[derive(Component)]
pub struct Ellipse;
#[derive(Default, Resource)]
pub struct ExperimentState {
    pub final_result: Vec<(usize, usize, String, f32)>,
    pub num_ellipses_left: usize,
    pub num_ellipses_right: usize,
    pub num_trials: usize,
    pub complete: bool,
    pub ellipses_drawn: bool, 
}



#[derive(Debug,Resource)]
pub struct TotalTrial(pub usize);

impl Default for TotalTrial {
    fn default() -> Self {
        TotalTrial(5)
    }
}

#[derive(Debug,Resource)]
pub struct Radius(pub f32);
impl Default for Radius {
    fn default() -> Self {
        Radius(2.0)
    }
}

#[derive(Debug,Resource)]
pub struct MinEllipse(pub usize);
impl Default for MinEllipse {
    fn default() -> Self {
        MinEllipse(5)
    }
}

#[derive(Debug,Resource)]
pub struct MaxEllipse(pub usize);
impl Default for MaxEllipse {
    fn default() -> Self {
        MaxEllipse(100)
    }
}

#[derive(Debug,Resource)]
pub struct EllipseColor(pub Color);
impl Default for EllipseColor {
    fn default() -> Self {
        EllipseColor(Color::PURPLE)
    }
}


#[derive(Resource)]
pub struct TrialState {
    pub start_time: Instant,
}
impl Default for TrialState {
    fn default() -> Self {
        TrialState {
            start_time: Instant::now(),
        }
    }
}
#[derive(Debug,PartialEq,Resource)]
pub enum AppState {
    Instruction,
    Experiment,
    Fixation,
    Results,
}
#[derive(Resource)]
pub struct FixationTimer {
    pub timer: Timer,
}

impl Default for FixationTimer {
    fn default() -> Self {
        FixationTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
}

#[derive(Debug,Resource)]
pub struct ExperimentBackgroundColor(pub Color);
impl Default for ExperimentBackgroundColor {
    fn default() -> Self {
        ExperimentBackgroundColor(Color::GRAY)
    }
}

#[derive(Debug,Resource)]
pub struct FixationBackgroundColor(pub Color);
impl Default for FixationBackgroundColor {
    fn default() -> Self {
        FixationBackgroundColor(Color::GRAY)
    }
}

#[derive(Debug,Clone,Copy,Resource)]
#[allow(dead_code)]
pub enum DrawingMethod{
    Uniform,
    Grid,
}

#[derive(Debug,Resource)]
pub struct CurrentDrawingMethod(pub DrawingMethod);
impl Default for CurrentDrawingMethod {
    fn default() -> Self {
        CurrentDrawingMethod(DrawingMethod::Uniform)
    }
}
