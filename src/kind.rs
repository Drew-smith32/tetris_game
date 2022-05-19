use bevy::{prelude::*, math::vec3};

#[derive(Component, Debug, Clone, Copy)]
pub enum Kind {
    Square,
    Line,
    T,
    L,
    J,
    S,
    Z,
}

pub const KINDS: [Kind; 7] = [
    Kind::J,
    Kind::L,
    Kind::Line,
    Kind::S,
    Kind::Square,
    Kind::T,
    Kind::Z,
];

impl Kind {
    pub fn color(&self) -> Color {
        match self {
            Kind::Square => Color::LIME_GREEN,
            Kind::Line => Color::BLUE,
            Kind::T => Color::PURPLE,
            Kind::L => Color::ORANGE,
            Kind::J => Color::CYAN,
            Kind::S => Color::RED,
            Kind::Z => Color::YELLOW,
        }
    }

    pub fn locations(&self) -> [Vec3; 4] {
        match self {
            Kind::Square => [
                Vec3::ZERO,
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                vec3(1.0, 1.0, 0.0),
            ],
            Kind::Line => [
                Vec3::ZERO,
                vec3(1.0, 0.0, 0.0),
                vec3(2.0, 0.0, 0.0),
                vec3(-1.0, 0.0, 0.0),
            ],
            Kind::T => [
                Vec3::ZERO,
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                vec3(-1.0, 0.0, 0.0),
            ],
            Kind::L => [
                Vec3::ZERO,
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 2.0, 0.0),
            ],
            Kind::J => [
                Vec3::ZERO,
                vec3(-1.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 2.0, 0.0),
            ],
            Kind::S => [
                Vec3::ZERO,
                vec3(1.0, 0.0, 0.0),
                vec3(1.0, 1.0, 0.0),
                vec3(2.0, 1.0, 0.0),
            ],
            Kind::Z => [
                Vec3::ZERO,
                vec3(-1.0, 0.0, 0.0),
                vec3(-1.0, 1.0, 0.0),
                vec3(-2.0, 1.0, 0.0),
            ],
        }
    }
}
