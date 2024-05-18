use bevy::prelude::{Transform, Vec2, Window};

pub fn screen_pos_to_world_pos(window: &Window, camera_transform: &Transform) -> Option<Vec2> {
    // Get the cursor position in screen coordinates
    let screen_pos = window.cursor_position()?;

    // Calculate the window size as a Vec2
    let window_size = Vec2::new(window.width(), window.height());

    // Adjust the screen position to be relative to the center of the window
    let cursor_pos = screen_pos - window_size / 2.0;

    // Convert the adjusted screen position to world coordinates
    let cursor_world_pos = camera_transform.compute_matrix() * cursor_pos.extend(0.0).extend(1.0);

    // Return the world position as Vec2, truncated to remove the z-component
    Some(cursor_world_pos.truncate().truncate())
}
