// Once a tick: notifications and XP drops read the player.
fn better_pvp_tick(game: &mut crate::game::Game<'_>) {
    crate::game::better_pvp_engine::tick(game);
}
