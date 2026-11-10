// Better PVP's game-facing code: reads the player's state and draws the HUD.
// Added to the game as `game::better_pvp_engine`, a child of `game`, so it can
// read the private fields of `Game`. Pure logic lives in runtime.rs.
use super::Game;
use super::better_pvp as m;
use horizon_client::{extras::Corner, extras_hud::Corners, hud::HudBatch, keymapping::KeyAction};
use std::{cell::RefCell, time::Instant};
use winit::keyboard::KeyCode;

struct State {
    settings: Option<m::Settings>,
    notices: m::Notices,
    xp: m::XpDrops,
    quick: m::Quick,
    started: Instant,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        settings: None,
        notices: m::Notices::default(),
        xp: m::XpDrops::default(),
        quick: m::Quick::default(),
        started: Instant::now(),
    });
}

const BOX: u32 = 0x7800_0000;
const WHITE: u32 = 0xFF_FFFF;
const GRAY: u32 = 0xAA_AAAA;
const ROW: f32 = 11.0;
const ICON_ROW: f32 = 18.0;
const PAD: f32 = 4.0;

fn corner(i: usize) -> Corner {
    [Corner::TopLeft, Corner::TopRight, Corner::BottomLeft, Corner::BottomRight][i]
}

pub(super) fn settings(game: &Game<'_>) -> m::Settings {
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        *s.settings.get_or_insert_with(|| m::Settings::read(|k| game.native_setting(m::ID, k)))
    })
}

pub(super) fn set_settings(game: &Game<'_>, settings: m::Settings) {
    STATE.with(|s| s.borrow_mut().settings = Some(settings));
    for (key, value) in settings.stored() {
        game.native_set_setting(m::ID, key, &value);
    }
}

/// A box of `w` x `h` in corner `i`, when that corner is free.
fn place(game: &Game<'_>, corners: &mut Corners, i: usize, w: f32, h: f32) -> Option<(f32, f32)> {
    let c = corner(i);
    game.native_corner_shown(c).then(|| corners.place(c, w, h))
}

// ---- per tick --------------------------------------------------------------

pub(super) fn tick(game: &mut Game<'_>) {
    if !game.native_window() || !game.native_on(m::ID) {
        return;
    }
    let s = settings(game);
    let Some(p) = game.player.as_ref() else { return };
    let health = p.health.health();
    let food = p.health.food.food as i32;
    let air = p.air;
    let (px, py, pz) = (p.position.x, p.position.y, p.position.z);
    // A creeper that is swelling within 7 blocks.
    let danger = game.mobs.iter().any(|mob| {
        mob.kind.id.contains("creeper") && mob.swell > 0 && {
            let (dx, dy, dz) = (mob.position.x - px, mob.position.y - py, mob.position.z - pz);
            dx * dx + dy * dy + dz * dz < 49.0
        }
    });
    let total = game.experience.total;
    let playing = game.native_playing();
    STATE.with(|st| {
        let mut st = st.borrow_mut();
        if s.notifications {
            st.notices.tick(health, food, air, danger);
        } else {
            st.notices.list.clear();
        }
        if s.xp_drops {
            st.xp.tick(total);
        } else {
            st.xp.list.clear();
        }
    });
    // A quick use ends if a screen opens over it.
    if !playing {
        quick_end(game);
    }
}

// ---- keys ------------------------------------------------------------------

pub(super) fn key(game: &mut Game<'_>, key: &str, pressed: bool) {
    if !game.native_on(m::ID) || !game.native_window() {
        return;
    }
    let s = settings(game);
    let Some(name) = m::KEYS
        .iter()
        .find(|k| game.native_mod_key(m::ID, k.0, k.2) == key)
        .map(|k| k.0)
    else {
        return;
    };
    if !game.native_playing() && pressed {
        return;
    }
    match name {
        "toggle_sneak" if s.toggle_sneak && pressed => {
            game.toggles.key(0, KeyAction::Press, false, true);
            game.sync_toggle(0);
        }
        "better_sprint" if s.better_sprint => {
            if pressed {
                game.keys.insert(KeyCode::KeyW);
                game.keys.insert(KeyCode::ControlLeft);
            } else {
                if !game.physical_keys.contains(&KeyCode::KeyW) {
                    game.keys.remove(&KeyCode::KeyW);
                }
                if !game.physical_keys.contains(&KeyCode::ControlLeft) {
                    game.keys.remove(&KeyCode::ControlLeft);
                }
            }
        }
        "quick_eat" if s.quick_use => {
            if pressed {
                let hungry = game.player.as_ref().is_some_and(|p| p.health.food.food < 20);
                let slot = (0..9).find(|&i| {
                    game.inventory.get(i).and_then(|s| s.as_ref()).is_some_and(|st| {
                        game.items()
                            .get(&st.item)
                            .is_some_and(|info| info.food.is_some() && (hungry || info.can_always_eat))
                    })
                });
                if let Some(slot) = slot {
                    quick_start(game, slot);
                }
            } else {
                quick_end(game);
            }
        }
        n if s.quick_use => {
            if let Some(slot) = m::quick_slot(n) {
                if pressed {
                    quick_start(game, slot);
                } else {
                    quick_end(game);
                }
            }
        }
        _ => {}
    }
}

/// Use the item in hotbar `slot` like holding the use button, without
/// leaving the item in hand: the hand returns when the key is released.
fn quick_start(game: &mut Game<'_>, slot: usize) {
    let busy = STATE.with(|s| s.borrow().quick.active.is_some());
    if busy || slot >= 9 || game.inventory.get(slot).is_none_or(|s| s.is_none()) {
        return;
    }
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        s.quick.restore = Some(game.selected);
        s.quick.active = Some(slot);
    });
    game.selected = slot;
    game.right = true;
}

fn quick_end(game: &mut Game<'_>) {
    let restore = STATE.with(|s| {
        let mut s = s.borrow_mut();
        s.quick.active = None;
        s.quick.restore.take()
    });
    if let Some(prev) = restore {
        game.right = false;
        game.selected = prev;
    }
}

// ---- HUD -------------------------------------------------------------------

fn panel(game: &Game<'_>, batch: &mut HudBatch, at: (f32, f32), w: f32, h: f32) {
    game.native_fill(batch, [at.0, at.1, w, h], BOX);
}

pub(super) fn hud(game: &mut Game<'_>, batch: &mut HudBatch, corners: &mut Corners) {
    if !game.native_on(m::ID) || !game.native_window() || game.player.is_none() {
        return;
    }
    let s = settings(game);
    let game = &*game;
    if s.potion_status {
        potions(game, batch, corners, &s);
    }
    if s.armor_status {
        armor(game, batch, corners, &s);
    }
    if s.entity_info {
        entity_info(game, batch, corners, &s);
    }
    messages(game, batch, corners, &s);
}

fn potions(game: &Game<'_>, batch: &mut HudBatch, corners: &mut Corners, s: &m::Settings) {
    let ms = STATE.with(|st| st.borrow().started.elapsed().as_millis());
    let rows: Vec<(String, String, i32)> = game
        .potions
        .effects
        .list
        .iter()
        .map(|e| (m::effect_label(e.id, e.amplifier), m::clock(e.duration), e.duration))
        .collect();
    if rows.is_empty() {
        return;
    }
    let col = rows.iter().map(|r| game.native_text_width(&r.0)).fold(0.0, f32::max);
    let tw = rows.iter().map(|r| game.native_text_width(&r.1)).fold(0.0, f32::max);
    let w = col + tw + PAD * 3.0;
    let h = rows.len() as f32 * ROW + PAD;
    let Some(at) = place(game, corners, s.potion_corner, w, h) else { return };
    panel(game, batch, at, w, h);
    for (i, (label, time, ticks)) in rows.iter().enumerate() {
        let y = at.1 + PAD / 2.0 + i as f32 * ROW + 1.0;
        let color = if m::blinks(*ticks, ms) { 0xFF5555 } else { WHITE };
        game.native_text(batch, label, [at.0 + PAD, y], color, true);
        game.native_text(batch, time, [at.0 + w - PAD - game.native_text_width(time), y], color, true);
    }
}

fn armor(game: &Game<'_>, batch: &mut HudBatch, corners: &mut Corners, s: &m::Settings) {
    let mut slots: Vec<&horizon_client::items::ItemStack> = Vec::new();
    for a in game.armor.iter().flatten() {
        slots.push(a);
    }
    let held = game.inventory.get(game.selected).and_then(|s| s.as_ref());
    if let Some(h) = held {
        slots.push(h);
    }
    if let Some(o) = game.offhand.as_ref() {
        slots.push(o);
    }
    if slots.is_empty() {
        return;
    }
    let arrows = |stack: &horizon_client::items::ItemStack| -> Option<u32> {
        if !s.left_arrows || !(stack.item.ends_with("bow") || stack.item.ends_with("crossbow")) {
            return None;
        }
        let n = game
            .inventory
            .iter()
            .flatten()
            .chain(game.offhand.iter())
            .filter(|i| i.item.ends_with("arrow"))
            .map(|i| i.count)
            .sum::<u32>();
        Some(n)
    };
    let lines: Vec<(&horizon_client::items::ItemStack, String, u32)> = slots
        .iter()
        .map(|st| {
            let max = game.items().get(&st.item).map_or(0, |i| i.max_damage);
            let parts: Vec<String> = [
                m::durability_text(s.durability, st.damage, max),
                arrows(st).map(|n| format!("{n} arrows")),
                (max == 0 && st.count > 1).then(|| format!("x{}", st.count)),
            ]
            .into_iter()
            .flatten()
            .collect();
            let text = parts.join("  ");
            let color = if max > 0 { m::durability_color(max.saturating_sub(st.damage), max) } else { WHITE };
            (*st, text, color)
        })
        .collect();
    let tw = lines.iter().map(|l| game.native_text_width(&l.1)).fold(0.0, f32::max);
    let w = 16.0 + tw + PAD * 3.0;
    let h = lines.len() as f32 * ICON_ROW + PAD;
    let Some(at) = place(game, corners, s.armor_corner, w, h) else { return };
    panel(game, batch, at, w, h);
    for (i, (stack, text, color)) in lines.iter().enumerate() {
        let y = at.1 + PAD / 2.0 + i as f32 * ICON_ROW;
        game.native_item(batch, &stack.item, [at.0 + PAD, y], 16.0);
        game.native_text(batch, text, [at.0 + PAD * 2.0 + 16.0, y + 4.0], *color, true);
    }
}

fn entity_info(game: &Game<'_>, batch: &mut HudBatch, corners: &mut Corners, s: &m::Settings) {
    let Some((i, ..)) = game.mob_hit() else { return };
    let mob = &game.mobs[i];
    let name = m::pretty(&mob.kind.id);
    let hp = format!("Health: {:.1} / {:.1}", mob.health.max(0.0), mob.kind.max_health);
    let armor = (mob.kind.armor > 0.0).then(|| format!("Armor: {:.0}", mob.kind.armor));
    let mut lines = vec![(name, WHITE), (hp, 0xFF5555)];
    if let Some(a) = armor {
        lines.push((a, GRAY));
    }
    // The mob's spawn egg as its icon, when it has one.
    let egg = format!("{}_spawn_egg", mob.kind.id);
    let icon = game.items().get(&egg).is_some();
    let side = if icon { 16.0 + PAD } else { 0.0 };
    let w = lines.iter().map(|l| game.native_text_width(&l.0)).fold(0.0, f32::max) + PAD * 2.0 + side;
    let h = (lines.len() as f32 * ROW + PAD).max(if icon { 16.0 + PAD } else { 0.0 });
    let Some(at) = place(game, corners, s.info_corner, w, h) else { return };
    panel(game, batch, at, w, h);
    if icon {
        game.native_item(batch, &egg, [at.0 + PAD, at.1 + (h - 16.0) / 2.0], 16.0);
    }
    for (n, (text, color)) in lines.iter().enumerate() {
        game.native_text(batch, text, [at.0 + PAD + side, at.1 + PAD / 2.0 + n as f32 * ROW + 1.0], *color, true);
    }
}

fn messages(game: &Game<'_>, batch: &mut HudBatch, corners: &mut Corners, s: &m::Settings) {
    let lines: Vec<(String, u32)> = STATE.with(|st| {
        let st = st.borrow();
        let mut v: Vec<(String, u32)> = st.notices.list.iter().map(|n| (n.text.clone(), n.rgb)).collect();
        v.extend(st.xp.list.iter().map(|d| (m::XpDrops::text(d.0), if d.0 > 0 { 0x55FF55 } else { 0xFF5555 })));
        v
    });
    if lines.is_empty() {
        return;
    }
    let w = lines.iter().map(|l| game.native_text_width(&l.0)).fold(0.0, f32::max) + PAD * 2.0;
    let h = lines.len() as f32 * ROW + PAD;
    let Some(at) = place(game, corners, s.notice_corner, w, h) else { return };
    panel(game, batch, at, w, h);
    for (n, (text, color)) in lines.iter().enumerate() {
        game.native_text(batch, text, [at.0 + PAD, at.1 + PAD / 2.0 + n as f32 * ROW + 1.0], *color, true);
    }
}
