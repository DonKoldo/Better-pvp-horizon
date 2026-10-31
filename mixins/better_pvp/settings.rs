// The mod's settings page.
fn better_pvp_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::{better_pvp as m, better_pvp_engine as e};
    if mod_id != m::ID {
        return;
    }
    buttons.extend(e::settings(&*game).labels());
}
