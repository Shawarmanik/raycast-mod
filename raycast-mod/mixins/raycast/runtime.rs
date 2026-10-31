use bytemuck::{Pod, Zeroable};
use horizon_client::hud::HudBatch;
use horizon_client::extras_hud::Corners;
use crate::game::Game;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct RaycastUniforms {
    camera_pos: [f32; 3],
    aspect_ratio: f32,
    camera_dir: [f32; 3],
    fov: f32,
    max_steps: f32,
    _padding: [f32; 3],
}

pub fn on_frame(game: &mut Game<'_>, _batch: &mut HudBatch, _corners: &mut Corners) {
    // Пока просто заглушка — убеждаемся что хук вызывается
    // Позже здесь будет логика рейкастинга
    
    let _uniforms = RaycastUniforms {
        camera_pos: [0.0, 0.0, 0.0],
        aspect_ratio: 1.0,
        camera_dir: [0.0, 0.0, -1.0],
        fov: 70.0,
        max_steps: 128.0,
        _padding: [0.0; 3],
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniforms_size() {
        assert_eq!(std::mem::size_of::<RaycastUniforms>(), 64);
    }
}
