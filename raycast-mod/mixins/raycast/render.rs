use crate::game::Game;
use horizon_client::hud::HudBatch;
use horizon_client::extras_hud::Corners;

// Эта функция вызывается Horizon вместо стандартного начала отрисовки HUD
pub fn raycast_hud_hook(
    game: &mut Game<'_>,
    batch: &mut HudBatch,
    corners: &mut Corners,
) {
    // Передаем управление в runtime.rs
    crate::game::raycast::on_frame(game, batch, corners);
}