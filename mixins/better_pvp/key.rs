// Toggle sneak, better sprint and quick use.
fn better_pvp_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    crate::game::better_pvp_engine::key(game, key, pressed);
}
