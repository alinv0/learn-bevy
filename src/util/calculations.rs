use bevy::math::Vec3;
use bevy::prelude::*;
use rand::prelude::*;

pub fn get_boundaries(window: &Window, object_size: f32) -> (f32, f32, f32, f32) {
    let radius = object_size / 2.0;
    let x_min = 0.0 + radius;
    let x_max = window.width() - radius;
    let y_min = 0.0 + radius;
    let y_max = window.height() - radius;

    (x_min, x_max, y_min, y_max)
}

pub fn get_random_bounded_coordinates(window: &Window, object_size: f32) -> (f32, f32) {
    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();

    let (x, y) = get_bounded_position(window, object_size, random_x, random_y);
    (x, y)
}

pub fn get_bounded_position(window: &Window, object_size: f32, x: f32, y: f32) -> (f32, f32) {
    let (x_min, x_max, y_min, y_max) = get_boundaries(window, object_size);

    let x = if x < x_min {
        x_min
    } else if x > x_max {
        x_max
    } else {
        x
    };

    let y = if y < y_min {
        y_min
    } else if y > y_max {
        y_max
    } else {
        y
    };

    (x, y)
}

pub fn get_bounded_translation(translation: Vec3,
                               x_min: f32, x_max: f32,
                               y_min: f32, y_max: f32) -> Vec3 {
    let mut translation = translation;

    if translation.x < x_min {
        translation.x = x_min;
    } else if translation.x > x_max {
        translation.x = x_max;
    }

    if translation.y < y_min {
        translation.y = y_min;
    } else if translation.y > y_max {
        translation.y = y_max;
    }

    translation
}

pub fn detect_collision(t1: &Transform, size1: f32, t2: &Transform, size2: f32) -> bool {
    let distance = t1.translation.distance(t2.translation);
    let radius1 = size1 / 2.0;
    let radius2 = size2 / 2.0;

    distance < radius1 + radius2
}

pub fn get_random_direction() -> Vec2 {
    Vec2::new(random::<f32>(), random::<f32>()).normalize()
}

