use std::f32::consts::TAU;
use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, tick);
}

impl SinPhase {
    pub fn new(secs: f32) -> Self {
        SinPhase(Timer::from_seconds(secs, TimerMode::Repeating))
    }

    #[inline]
    pub fn get_phase(&self) -> f32 {
        (TAU * self.0.elapsed_secs() / self.0.duration().as_secs_f32()).sin()
    }
}

impl Default for SinPhase {
    fn default() -> Self {
        Self::new(1.0)
    }
}

#[derive(Component)]
pub struct SinPhase(pub(crate) Timer);

fn tick(time: Res<Time>, mut phases: Query<&mut SinPhase>) {
    for mut phase in phases.iter_mut() {
        phase.0.tick(time.delta());
    }
}