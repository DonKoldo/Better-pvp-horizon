// The HUD boxes: potion status, armor status, entity info, notifications, XP drops.
fn better_pvp_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    crate::game::better_pvp_engine::hud(game, batch, corners);
}
