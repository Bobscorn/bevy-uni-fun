use crate::consts::*;
use bevy::input::{keyboard::KeyCode, Input};
use bevy::prelude::*;
use core::f32::consts::PI;


#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum ArrowDirection {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl ArrowDirection {
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            ArrowDirection::Up => [KeyCode::Up, KeyCode::W],
            ArrowDirection::Left => [KeyCode::Left, KeyCode::A],
            ArrowDirection::Down => [KeyCode::Down, KeyCode::S],
            ArrowDirection::Right => [KeyCode::Right, KeyCode::D]
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }

    pub fn rotation(&self) -> f32 {
        match self {
            ArrowDirection::Up => PI * 0.5,
            ArrowDirection::Down => -PI * 0.5,
            ArrowDirection::Left => PI,
            ArrowDirection::Right => 0.,
        }
    }

    pub fn y(&self) -> f32 {
        match self {
            ArrowDirection::Up => 150.,
            ArrowDirection::Down => 50.,
            ArrowDirection::Left => -50.,
            ArrowDirection::Right => -150.,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub enum Speed {
    #[default]
    Slow,
    Medium,
    Fast,
}

impl Speed {
    pub fn value(&self) -> f32 {
        ARROW_JOURNEY_SPEED * self.multiplier()
    }

    pub fn multiplier(&self) -> f32 {
        match self {
            Speed::Slow => 1.,
            Speed::Medium => 1.2,
            Speed::Fast => 1.5,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ArrowTime {
    pub spawn_time: f64,
    pub speed: Speed,
    pub direction: ArrowDirection,
}

#[derive(Debug, Resource)]
pub struct SongConfig {
    pub arrows: Vec<ArrowTime>,
}

impl ArrowTime {
    fn new(click_time: f64, speed: Speed, direction: ArrowDirection) -> Self {
        let speed_value = speed.value();
        Self {
            spawn_time: click_time - (ARROW_JOURNEY_DISTANCE / speed_value) as f64,
            speed,
            direction
        }
    }
}

pub fn load_config() -> SongConfig {
    SongConfig { 
        arrows:  vec![
            ArrowTime::new(1., Speed::Slow, ArrowDirection::Up),
            ArrowTime::new(2., Speed::Slow, ArrowDirection::Down),
            ArrowTime::new(3., Speed::Slow, ArrowDirection::Left),
            ArrowTime::new(4., Speed::Medium, ArrowDirection::Up),
            ArrowTime::new(5., Speed::Fast, ArrowDirection::Down),
        ]
    }
}

