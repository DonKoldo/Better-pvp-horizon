// Better PVP for Horizon: pure logic (settings, formatting, timers). It is both
// this crate's Cargo library (its tests) and a module the bundle adds to the
// game as `game::better_pvp`. The parts that touch the game are in engine.rs.
//
// An independent implementation of the feature set of Xaero's Better PVP; it
// contains none of that mod's code or assets.

pub const ID: &str = "better_pvp";
pub const NAME: &str = "Better PVP";
pub const VERSION: &str = "1.0.1";
pub const DESCRIPTION: &str = "PVP and survival HUD: potion status, armor status, notifications, XP drops, entity info, toggle sneak, better sprint and quick use. Inspired by Xaero's Better PVP.";

/// The corners, as the settings files name them and the page labels them.
pub const CORNERS: [(&str, &str); 4] = [
    ("top_left", "Top left"),
    ("top_right", "Top right"),
    ("bottom_left", "Bottom left"),
    ("bottom_right", "Bottom right"),
];
pub const DURABILITY: [(&str, &str); 4] = [
    ("percent", "Percents"),
    ("number", "Numbers"),
    ("both", "Both"),
    ("none", "None"),
];

/// Keys: name, label, default key. Players rebind them in Options > Mods.
pub const KEYS: [(&str, &str, &str); 12] = [
    ("toggle_sneak", "Toggle Sneak", "key.keyboard.caps.lock"),
    ("better_sprint", "Better Sprint", "key.keyboard.v"),
    ("quick_eat", "Quick Eat", "key.keyboard.b"),
    ("quick_use_1", "Quick Use 1", "key.keyboard.keypad.1"),
    ("quick_use_2", "Quick Use 2", "key.keyboard.keypad.2"),
    ("quick_use_3", "Quick Use 3", "key.keyboard.keypad.3"),
    ("quick_use_4", "Quick Use 4", "key.keyboard.keypad.4"),
    ("quick_use_5", "Quick Use 5", "key.keyboard.keypad.5"),
    ("quick_use_6", "Quick Use 6", "key.keyboard.keypad.6"),
    ("quick_use_7", "Quick Use 7", "key.keyboard.keypad.7"),
    ("quick_use_8", "Quick Use 8", "key.keyboard.keypad.8"),
    ("quick_use_9", "Quick Use 9", "key.keyboard.keypad.9"),
];

/// How many of `KEYS` the settings page lists; the Quick Use slots stay on
/// the keypad (rebind them in horizon-mods.txt as `key.better_pvp.quick_use_N`).
pub const PAGE_KEYS: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub potion_status: bool,
    pub armor_status: bool,
    pub notifications: bool,
    pub xp_drops: bool,
    pub entity_info: bool,
    pub toggle_sneak: bool,
    pub better_sprint: bool,
    pub quick_use: bool,
    pub left_arrows: bool,
    /// Index in `DURABILITY`.
    pub durability: usize,
    /// Indices in `CORNERS`.
    pub potion_corner: usize,
    pub armor_corner: usize,
    pub info_corner: usize,
    pub notice_corner: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            potion_status: true,
            armor_status: true,
            notifications: true,
            xp_drops: true,
            entity_info: true,
            toggle_sneak: true,
            better_sprint: true,
            quick_use: true,
            left_arrows: true,
            durability: 0,
            potion_corner: 2,
            armor_corner: 3,
            info_corner: 1,
            notice_corner: 0,
        }
    }
}

fn flag(get: &impl Fn(&str) -> Option<String>, key: &str, default: bool) -> bool {
    get(key).map_or(default, |v| v.trim() != "false")
}
fn index(get: &impl Fn(&str) -> Option<String>, key: &str, names: &[(&str, &str)], default: usize) -> usize {
    get(key)
        .and_then(|c| names.iter().position(|k| k.0 == c.trim()))
        .unwrap_or(default)
}
/// Off -> top left -> top right -> bottom left -> bottom right -> off.
fn cycle(on: &mut bool, corner: &mut usize) {
    if !*on {
        *on = true;
        *corner = 0;
    } else if *corner + 1 < CORNERS.len() {
        *corner += 1;
    } else {
        *on = false;
    }
}
fn word(b: bool) -> &'static str {
    if b { "ON" } else { "OFF" }
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        Self {
            potion_status: flag(&get, "potion_status", d.potion_status),
            armor_status: flag(&get, "armor_status", d.armor_status),
            notifications: flag(&get, "notifications", d.notifications),
            xp_drops: flag(&get, "xp_drops", d.xp_drops),
            entity_info: flag(&get, "entity_info", d.entity_info),
            toggle_sneak: flag(&get, "toggle_sneak", d.toggle_sneak),
            better_sprint: flag(&get, "better_sprint", d.better_sprint),
            quick_use: flag(&get, "quick_use", d.quick_use),
            left_arrows: flag(&get, "left_arrows", d.left_arrows),
            durability: index(&get, "durability", &DURABILITY, d.durability),
            potion_corner: index(&get, "potion_corner", &CORNERS, d.potion_corner),
            armor_corner: index(&get, "armor_corner", &CORNERS, d.armor_corner),
            info_corner: index(&get, "info_corner", &CORNERS, d.info_corner),
            notice_corner: index(&get, "notice_corner", &CORNERS, d.notice_corner),
        }
    }
    /// The page's buttons, ten of them: each HUD box is one button that
    /// cycles Off -> the four corners -> Off.
    pub fn labels(&self) -> Vec<String> {
        let spot = |on: bool, c: usize| if on { CORNERS[c].1 } else { "OFF" };
        vec![
            format!("Potion Status: {}", spot(self.potion_status, self.potion_corner)),
            format!("Armor Status: {}", spot(self.armor_status, self.armor_corner)),
            format!("Durability In: {}", DURABILITY[self.durability].1),
            format!("Display Left Arrows: {}", word(self.left_arrows)),
            format!("Notifications: {}", spot(self.notifications, self.notice_corner)),
            format!("XP Drops: {}", word(self.xp_drops)),
            format!("Entity Info: {}", spot(self.entity_info, self.info_corner)),
            format!("Toggle Sneak: {}", word(self.toggle_sneak)),
            format!("Better Sprint: {}", word(self.better_sprint)),
            format!("Quick Item Use: {}", word(self.quick_use)),
        ]
    }
    pub fn click(&mut self, i: usize) {
        match i {
            0 => cycle(&mut self.potion_status, &mut self.potion_corner),
            1 => cycle(&mut self.armor_status, &mut self.armor_corner),
            2 => self.durability = (self.durability + 1) % DURABILITY.len(),
            3 => self.left_arrows = !self.left_arrows,
            4 => cycle(&mut self.notifications, &mut self.notice_corner),
            5 => self.xp_drops = !self.xp_drops,
            6 => cycle(&mut self.entity_info, &mut self.info_corner),
            7 => self.toggle_sneak = !self.toggle_sneak,
            8 => self.better_sprint = !self.better_sprint,
            9 => self.quick_use = !self.quick_use,
            _ => {}
        }
    }
    pub fn stored(&self) -> Vec<(&'static str, String)> {
        let b = |v: bool| if v { "true" } else { "false" }.to_owned();
        vec![
            ("potion_status", b(self.potion_status)),
            ("armor_status", b(self.armor_status)),
            ("notifications", b(self.notifications)),
            ("xp_drops", b(self.xp_drops)),
            ("entity_info", b(self.entity_info)),
            ("toggle_sneak", b(self.toggle_sneak)),
            ("better_sprint", b(self.better_sprint)),
            ("quick_use", b(self.quick_use)),
            ("left_arrows", b(self.left_arrows)),
            ("durability", DURABILITY[self.durability].0.to_owned()),
            ("potion_corner", CORNERS[self.potion_corner].0.to_owned()),
            ("armor_corner", CORNERS[self.armor_corner].0.to_owned()),
            ("info_corner", CORNERS[self.info_corner].0.to_owned()),
            ("notice_corner", CORNERS[self.notice_corner].0.to_owned()),
        ]
    }
}

// ---- formatting -----------------------------------------------------------

/// `minecraft:diamond_sword` -> `Diamond Sword`.
pub fn pretty(id: &str) -> String {
    let id = id.rsplit(':').next().unwrap_or(id);
    id.split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next()
                .map(|f| f.to_uppercase().collect::<String>() + c.as_str())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn roman(n: i32) -> String {
    const T: [(i32, &str); 9] = [
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"), (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
    ];
    if !(1..=399).contains(&n) {
        return n.to_string();
    }
    let mut n = n;
    let mut out = String::new();
    for (v, s) in T {
        while n >= v {
            out.push_str(s);
            n -= v;
        }
    }
    out
}

/// An effect's label: `Strength II` (amplifier 0 is level I, which is left off).
pub fn effect_label(id: &str, amplifier: i32) -> String {
    let name = pretty(id);
    if amplifier <= 0 { name } else { format!("{name} {}", roman(amplifier + 1)) }
}

/// Remaining ticks as `1:05`; -1 is infinite.
pub fn clock(ticks: i32) -> String {
    if ticks < 0 {
        return "**:**".to_owned();
    }
    let s = (ticks + 19) / 20;
    format!("{}:{:02}", s / 60, s % 60)
}

/// Whether an effect with `ticks` left should blink (the last 10 seconds).
pub fn blinks(ticks: i32, frame_ms: u128) -> bool {
    (0..200).contains(&ticks) && (frame_ms / 250) % 2 == 0
}

/// Durability text; `max` 0 means the item has none.
pub fn durability_text(mode: usize, damage: u32, max: u32) -> Option<String> {
    if max == 0 || DURABILITY[mode].0 == "none" {
        return None;
    }
    let left = max.saturating_sub(damage);
    let pct = (left as f64 * 100.0 / max as f64).round() as u32;
    Some(match DURABILITY[mode].0 {
        "percent" => format!("{pct}%"),
        "number" => format!("{left}"),
        _ => format!("{left} ({pct}%)"),
    })
}

/// Colour of a durability fraction left (green -> red), as rgb.
pub fn durability_color(left: u32, max: u32) -> u32 {
    if max == 0 {
        return 0xFFFFFF;
    }
    let f = (left as f32 / max as f32).clamp(0.0, 1.0);
    let r = ((1.0 - f) * 2.0).min(1.0);
    let g = (f * 2.0).min(1.0);
    ((r * 255.0) as u32) << 16 | ((g * 255.0) as u32) << 8 | 0x20
}

// ---- timers ----------------------------------------------------------------

/// A message on screen for a while.
#[derive(Clone, Debug, PartialEq)]
pub struct Notice {
    pub text: String,
    pub rgb: u32,
    pub ticks: i32,
}

/// Low-value notifications: each fires when a value falls to its limit and
/// again only after it has recovered above it (so it never spams).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Notices {
    pub list: Vec<Notice>,
    low_health: bool,
    low_food: bool,
    low_air: bool,
    danger: bool,
}

pub const NOTICE_TICKS: i32 = 80;
pub const LOW_HEALTH: f32 = 6.0;
pub const LOW_FOOD: i32 = 6;
pub const LOW_AIR: i32 = 100;

impl Notices {
    fn push(&mut self, text: &str, rgb: u32) {
        self.list.retain(|n| n.text != text);
        self.list.push(Notice { text: text.to_owned(), rgb, ticks: NOTICE_TICKS });
    }
    /// One tick: the player's values and whether a creeper is about to blow.
    pub fn tick(&mut self, health: f32, food: i32, air: i32, danger: bool) {
        for n in &mut self.list {
            n.ticks -= 1;
        }
        self.list.retain(|n| n.ticks > 0);
        let low = health <= LOW_HEALTH && health > 0.0;
        if low && !self.low_health {
            self.push("Low health!", 0xFF5555);
        }
        self.low_health = low;
        let hungry = food <= LOW_FOOD;
        if hungry && !self.low_food {
            self.push("Low hunger!", 0xFFAA00);
        }
        self.low_food = hungry;
        let breathless = air < LOW_AIR;
        if breathless && !self.low_air {
            self.push("Low oxygen!", 0x55FFFF);
        }
        self.low_air = breathless;
        if danger && !self.danger {
            self.push("Danger: explosion!", 0xFF5555);
        }
        self.danger = danger;
    }
}

/// Floating experience gains and losses.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct XpDrops {
    pub list: Vec<(i32, i32)>,
    last: Option<i32>,
}

pub const DROP_TICKS: i32 = 50;

impl XpDrops {
    pub fn tick(&mut self, total: i32) {
        for d in &mut self.list {
            d.1 -= 1;
        }
        self.list.retain(|d| d.1 > 0);
        if let Some(last) = self.last {
            let change = total - last;
            // A big negative jump is a death or level command, not a spend.
            if change != 0 && self.list.len() < 6 {
                self.list.push((change, DROP_TICKS));
            }
        }
        self.last = Some(total);
    }
    pub fn text(change: i32) -> String {
        if change > 0 { format!("+{change} XP") } else { format!("{change} XP") }
    }
}

/// Toggle sneak and quick use bookkeeping.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Quick {
    /// The hotbar slot that was held before a quick use, and the one in use.
    pub restore: Option<usize>,
    pub active: Option<usize>,
}

/// The hotbar slot index from a quick use key name (`quick_use_3` -> 2).
pub fn quick_slot(name: &str) -> Option<usize> {
    name.strip_prefix("quick_use_")?.parse::<usize>().ok().filter(|n| (1..=9).contains(n)).map(|n| n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_round_trip() {
        let mut s = Settings::read(|_| None);
        assert_eq!(s, Settings::default());
        s.click(0);
        s.click(2);
        s.click(1);
        let stored = s.stored();
        let again = Settings::read(|k| stored.iter().find(|e| e.0 == k).map(|e| e.1.clone()));
        assert_eq!(s, again);
        assert_eq!(s.labels().len(), 10);
        for i in 0..12 {
            s.click(i);
        }
    }

    #[test]
    fn a_box_button_cycles_through_the_corners() {
        let mut s = Settings::default();
        assert_eq!(s.labels()[0], "Potion Status: Bottom left");
        s.click(0);
        assert_eq!(s.labels()[0], "Potion Status: Bottom right");
        s.click(0);
        assert_eq!(s.labels()[0], "Potion Status: OFF");
        s.click(0);
        assert_eq!(s.labels()[0], "Potion Status: Top left");
        for _ in 0..3 {
            s.click(0);
        }
        assert_eq!(s.labels()[0], "Potion Status: Bottom right");
    }

    #[test]
    fn formats() {
        assert_eq!(pretty("minecraft:diamond_sword"), "Diamond Sword");
        assert_eq!(effect_label("strength", 1), "Strength II");
        assert_eq!(effect_label("speed", 0), "Speed");
        assert_eq!(clock(1300), "1:05");
        assert_eq!(clock(-1), "**:**");
        assert_eq!(roman(4), "IV");
        assert_eq!(durability_text(0, 250, 1000).as_deref(), Some("75%"));
        assert_eq!(durability_text(1, 250, 1000).as_deref(), Some("750"));
        assert_eq!(durability_text(2, 250, 1000).as_deref(), Some("750 (75%)"));
        assert_eq!(durability_text(3, 250, 1000), None);
        assert_eq!(durability_text(0, 0, 0), None);
        assert_eq!(quick_slot("quick_use_9"), Some(8));
        assert_eq!(quick_slot("quick_eat"), None);
    }

    #[test]
    fn notices_fire_once_per_dip() {
        let mut n = Notices::default();
        n.tick(20.0, 20, 300, false);
        assert!(n.list.is_empty());
        n.tick(5.0, 20, 300, false);
        assert_eq!(n.list.len(), 1);
        n.tick(4.0, 20, 300, false);
        assert_eq!(n.list.len(), 1);
        n.tick(15.0, 20, 300, false);
        n.tick(5.0, 20, 300, false);
        assert_eq!(n.list.len(), 1);
        for _ in 0..100 {
            n.tick(20.0, 20, 300, false);
        }
        assert!(n.list.is_empty());
    }

    #[test]
    fn xp_drops_track_changes() {
        let mut x = XpDrops::default();
        x.tick(10);
        assert!(x.list.is_empty());
        x.tick(17);
        assert_eq!(x.list[0].0, 7);
        assert_eq!(XpDrops::text(7), "+7 XP");
        assert_eq!(XpDrops::text(-3), "-3 XP");
        for _ in 0..60 {
            x.tick(17);
        }
        assert!(x.list.is_empty());
    }
}
