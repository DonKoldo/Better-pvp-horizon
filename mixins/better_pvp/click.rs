// A click on the settings page: change the setting and keep it.
fn better_pvp_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::{better_pvp as m, better_pvp_engine as e};
    if mod_id != m::ID {
        return;
    }
    let mut settings = e::settings(&*game);
    settings.click(button);
    e::set_settings(&*game, settings);
}
