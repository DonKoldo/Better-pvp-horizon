// The mod's row in Options > Mods (and so in the launcher's inventory).
fn better_pvp_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::better_pvp as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: m::KEYS.iter().take(m::PAGE_KEYS).map(|k| [k.0.to_owned(), k.1.to_owned(), k.2.to_owned()]).collect(),
    });
}
