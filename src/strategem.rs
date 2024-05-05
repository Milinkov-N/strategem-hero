use std::fmt::Display;

use crossterm::style::Stylize;
use rand::Rng;

use crate::event::Key;

pub type StrategemCode = [Option<StrategemKey>; 16];

#[derive(Clone, PartialEq, Eq)]
pub enum StrategemKey {
    Up,
    Down,
    Left,
    Right,
}

impl From<Key> for StrategemKey {
    fn from(value: Key) -> Self {
        match value {
            Key::ArrowUp => StrategemKey::Up,
            Key::ArrowDown => StrategemKey::Down,
            Key::ArrowLeft => StrategemKey::Left,
            Key::ArrowRight => StrategemKey::Right,
            unhandled => panic!("Cannot convert {unhandled:#?} to StrategemKey"),
        }
    }
}

impl Display for StrategemKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "🡅"),
            Self::Down => write!(f, "🡇"),
            Self::Left => write!(f, "🡄"),
            Self::Right => write!(f, "🡆"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrategemDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Clone)]
pub struct Strategem {
    name: &'static str,
    difficulty: StrategemDifficulty,
    idx: usize,
    valid: bool,
    completed: bool,
    code: StrategemCode,
}

impl Strategem {
    const fn builder() -> StrategemBuilder {
        StrategemBuilder::new()
    }

    pub const fn name(&self) -> &str {
        self.name
    }

    pub const fn difficulty(&self) -> &StrategemDifficulty {
        &self.difficulty
    }

    pub fn assert_key(&mut self, key: StrategemKey) {
        if self.is_completed() || !self.is_valid() {
            return;
        }

        if let Some(code_key) = &self.code[self.idx] {
            self.idx += 1;
            self.valid = code_key.eq(&key);
        }
    }

    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn is_completed(&self) -> bool {
        self.valid && self.code[self.idx] == None
    }

    pub fn reset(&mut self) {
        self.idx = 0;
        self.valid = true;
        self.completed = false;
    }
}

impl Display for Strategem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code.iter().enumerate().for_each(|(i, code)| {
            if let Some(key) = code {
                if !self.is_valid() {
                    write!(f, "{} ", key.to_string().dark_red()).unwrap();
                } else if i < self.idx {
                    write!(f, "{} ", key.to_string().yellow()).unwrap();
                } else {
                    write!(f, "{key} ").unwrap();
                }
            } else {
                write!(f, " ").unwrap();
            }
        });

        Ok(())
    }
}

struct StrategemBuilder {
    idx: usize,
    code: StrategemCode,
}

impl StrategemBuilder {
    const fn new() -> Self {
        Self {
            idx: 0,
            code: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ],
        }
    }

    const fn up(self) -> Self {
        self.insert(StrategemKey::Up)
    }

    const fn down(self) -> Self {
        self.insert(StrategemKey::Down)
    }

    const fn left(self) -> Self {
        self.insert(StrategemKey::Left)
    }

    const fn right(self) -> Self {
        self.insert(StrategemKey::Right)
    }

    const fn build(self, name: &'static str) -> Strategem {
        Strategem {
            name,
            difficulty: match self.idx {
                0..=3 => StrategemDifficulty::Easy,
                4..=6 => StrategemDifficulty::Medium,
                _ => StrategemDifficulty::Hard,
            },
            idx: 0,
            valid: true,
            completed: false,
            code: self.code,
        }
    }

    const fn insert(mut self, value: StrategemKey) -> Self {
        if self.idx < 16 {
            self.code[self.idx] = Some(value);
            self.idx += 1;
        }

        self
    }
}

const ALL_STRATEGEMS: [Strategem; 62] = [
    lift850_jump_pack(),
    b1_supply_pack(),
    axlas5_guard_dog_rover(),
    sh20_ballistic_shield_backpack(),
    sh32_shield_generator_pack(),
    axar23_guard_dog(),
    mg43_machine_gun(),
    apw1_antimateriel_rifle(),
    m105_stalwart(),
    eat17_expendable_antitank(),
    gr8_recoilless_rifle(),
    flam40_flamethrower(),
    ac8_autocannon(),
    mg206_heavy_machine_gun(),
    rs422_railgun(),
    faf14_spear_launcher(),
    gl21_grenade_launcher(),
    las98_laser_cannon(),
    arc3_arc_thrower(),
    las99_quasar_cannon(),
    rl77_airburst_rocket_launcher(),
    exo45_patriot_exosuit(),
    reinforce(),
    sos_beacon(),
    resupply(),
    nux223_hellbomb(),
    sssd_delivery(),
    seismic_probe(),
    upload_data(),
    eagle_rearm(),
    illumination_flare(),
    seaf_artillery(),
    super_earth_flag(),
    emg101_hmg_emplacement(),
    fx12_shield_generator_relay(),
    aarc3_tesla_tower(),
    md6_anti_personnel_minefield(),
    mdi4_incendiary_mines(),
    amg43_machine_gun_sentry(),
    ag16_galting_sentry(),
    am12_mortar_sentry(),
    aac8_autocannon_sentry(),
    amls4x_rocket_sentry(),
    am23_ems_mortar_sentry(),
    orbital_gatling_barrage(),
    orbital_airburst_strike(),
    orbital_120mm_he_barrage(),
    orbital_380mm_he_barrage(),
    orbital_walking_barrage(),
    orbital_laser(),
    orbital_railcannon_strike(),
    orbital_precision_strike(),
    orbital_gas_strike(),
    orbital_ems_strike(),
    orbital_smoke_strike(),
    eagle_strafing_run(),
    eagle_air_strike(),
    eagle_cluster_bomb(),
    eagle_napalm_airstrike(),
    eagle_smoke_strike(),
    eagle_110mm_rocket_pods(),
    eagle_500kg_bomb(),
];

pub fn random() -> Strategem {
    ALL_STRATEGEMS[rand::thread_rng().gen::<usize>() % ALL_STRATEGEMS.len()].clone()
}

pub const fn lift850_jump_pack() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .up()
        .down()
        .up()
        .build("LIFT-850 Jump Pack")
}

pub const fn b1_supply_pack() -> Strategem {
    Strategem::builder()
        .down()
        .left()
        .down()
        .up()
        .up()
        .down()
        .build("B-1 Supply Pack")
}

pub const fn axlas5_guard_dog_rover() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .left()
        .up()
        .right()
        .right()
        .build("AX/LAS-5 \"Guard Dog\" Rover")
}

pub const fn sh20_ballistic_shield_backpack() -> Strategem {
    Strategem::builder()
        .down()
        .left()
        .down()
        .down()
        .up()
        .left()
        .build("SH-20 Ballistic Shield Backpack")
}

pub const fn sh32_shield_generator_pack() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .left()
        .right()
        .left()
        .right()
        .build("SH-32 Shield Generator Pack")
}

pub const fn axar23_guard_dog() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .left()
        .up()
        .right()
        .down()
        .build("AX/AR-23 \"Guard Dog\"")
}

pub const fn mg43_machine_gun() -> Strategem {
    Strategem::builder()
        .down()
        .left()
        .down()
        .up()
        .right()
        .build("MG-43 Machine Gun")
}

pub const fn apw1_antimateriel_rifle() -> Strategem {
    Strategem::builder()
        .down()
        .left()
        .right()
        .up()
        .down()
        .build("APW-1 Anti-Materiel Rifle")
}

pub const fn m105_stalwart() -> Strategem {
    Strategem::builder()
        .down()
        .left()
        .down()
        .up()
        .up()
        .left()
        .build("M-105 Stalwart")
}

pub const fn eat17_expendable_antitank() -> Strategem {
    Strategem::builder()
        .down()
        .down()
        .left()
        .up()
        .right()
        .build("EAT-17 Expendable Anti-tank")
}

pub const fn gr8_recoilless_rifle() -> Strategem {
    Strategem::builder()
        .down()
        .left()
        .right()
        .right()
        .left()
        .build("GR-8 Recoilless Rifle")
}

pub const fn flam40_flamethrower() -> Strategem {
    Strategem::builder()
        .down()
        .left()
        .up()
        .down()
        .up()
        .build("FLAM-40 Flamethrower")
}

pub const fn ac8_autocannon() -> Strategem {
    Strategem::builder()
        .down()
        .left()
        .down()
        .up()
        .up()
        .right()
        .build("AC-8 Autocannon")
}

pub const fn mg206_heavy_machine_gun() -> Strategem {
    Strategem::builder()
        .down()
        .left()
        .up()
        .down()
        .down()
        .build("MG-206 Heavy Machine Gun")
}

pub const fn rs422_railgun() -> Strategem {
    Strategem::builder()
        .down()
        .right()
        .down()
        .up()
        .left()
        .right()
        .build("RS-422 Railgun")
}

pub const fn faf14_spear_launcher() -> Strategem {
    Strategem::builder()
        .down()
        .down()
        .up()
        .down()
        .down()
        .build("FAF-14 SPEAR Launcher")
}

pub const fn gl21_grenade_launcher() -> Strategem {
    Strategem::builder()
        .down()
        .left()
        .up()
        .left()
        .down()
        .build("GL-21 Grenade Launcher")
}

pub const fn las98_laser_cannon() -> Strategem {
    Strategem::builder()
        .down()
        .left()
        .down()
        .up()
        .left()
        .build("LAS-98 Laser Cannon")
}

pub const fn arc3_arc_thrower() -> Strategem {
    Strategem::builder()
        .down()
        .right()
        .down()
        .up()
        .left()
        .left()
        .build("ARC-3 Arc Thrower")
}

pub const fn las99_quasar_cannon() -> Strategem {
    Strategem::builder()
        .down()
        .down()
        .up()
        .left()
        .right()
        .build("LAS-99 Quasar Cannon")
}

pub const fn rl77_airburst_rocket_launcher() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .up()
        .left()
        .right()
        .build("RL-77 Airburst Rocket Launcher")
}

pub const fn exo45_patriot_exosuit() -> Strategem {
    Strategem::builder()
        .left()
        .down()
        .right()
        .up()
        .left()
        .down()
        .down()
        .build("EXO-45 Patriot Exosuit")
}

pub const fn reinforce() -> Strategem {
    Strategem::builder()
        .up()
        .down()
        .right()
        .left()
        .up()
        .build("Reinforce")
}

pub const fn sos_beacon() -> Strategem {
    Strategem::builder()
        .up()
        .down()
        .right()
        .up()
        .build("SOS Beacon")
}

pub const fn resupply() -> Strategem {
    Strategem::builder()
        .down()
        .down()
        .up()
        .right()
        .build("Resupply")
}

pub const fn nux223_hellbomb() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .left()
        .down()
        .up()
        .right()
        .down()
        .up()
        .build("NUX-223 Hellbomb")
}

pub const fn sssd_delivery() -> Strategem {
    Strategem::builder()
        .down()
        .down()
        .down()
        .up()
        .up()
        .build("SSSD Delivery")
}

pub const fn seismic_probe() -> Strategem {
    Strategem::builder()
        .up()
        .up()
        .left()
        .right()
        .down()
        .down()
        .build("Seismic Probe")
}

pub const fn upload_data() -> Strategem {
    Strategem::builder()
        .left()
        .right()
        .up()
        .up()
        .up()
        .build("Upload Data")
}

pub const fn eagle_rearm() -> Strategem {
    Strategem::builder()
        .up()
        .up()
        .left()
        .up()
        .right()
        .build("Eagle Rearm")
}

pub const fn illumination_flare() -> Strategem {
    Strategem::builder()
        .right()
        .right()
        .left()
        .left()
        .build("Illumination Flare")
}

pub const fn seaf_artillery() -> Strategem {
    Strategem::builder()
        .right()
        .up()
        .up()
        .down()
        .build("SEAF Artillery")
}

pub const fn super_earth_flag() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .down()
        .up()
        .build("Super Earth Flag")
}

pub const fn emg101_hmg_emplacement() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .left()
        .right()
        .right()
        .left()
        .build("E/MG-101 HMG Emplacement")
}

pub const fn fx12_shield_generator_relay() -> Strategem {
    Strategem::builder()
        .down()
        .down()
        .left()
        .right()
        .left()
        .right()
        .build("FX-12 Shield Generator Relay")
}

pub const fn aarc3_tesla_tower() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .right()
        .up()
        .left()
        .right()
        .build("A/ARC-3 Tesla Tower")
}

pub const fn md6_anti_personnel_minefield() -> Strategem {
    Strategem::builder()
        .down()
        .left()
        .up()
        .right()
        .build("MD-6 Anti-Personnel Minefield")
}

pub const fn mdi4_incendiary_mines() -> Strategem {
    Strategem::builder()
        .down()
        .left()
        .left()
        .down()
        .build("MD-I4 Incendiary Mines")
}

pub const fn amg43_machine_gun_sentry() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .right()
        .right()
        .up()
        .build("A/MG-43 Machine Gun Sentry")
}

pub const fn ag16_galting_sentry() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .right()
        .left()
        .build("A/G-16 Gatling Sentry")
}

pub const fn am12_mortar_sentry() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .right()
        .right()
        .down()
        .build("A/M-12 Mortar Sentry")
}

pub const fn aac8_autocannon_sentry() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .right()
        .up()
        .left()
        .up()
        .build("A/AC-8 Autocannon Sentry")
}

pub const fn amls4x_rocket_sentry() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .right()
        .right()
        .left()
        .build("A/MLS-4X Rocket Sentry")
}

pub const fn am23_ems_mortar_sentry() -> Strategem {
    Strategem::builder()
        .down()
        .up()
        .right()
        .down()
        .right()
        .build("A/M-23 EMS Mortar Sentry")
}

pub const fn orbital_gatling_barrage() -> Strategem {
    Strategem::builder()
        .right()
        .down()
        .left()
        .up()
        .up()
        .build("Orbital Gatling Barrage")
}

pub const fn orbital_airburst_strike() -> Strategem {
    Strategem::builder()
        .right()
        .right()
        .right()
        .build("Orbital Airburst Strike")
}

pub const fn orbital_120mm_he_barrage() -> Strategem {
    Strategem::builder()
        .right()
        .right()
        .down()
        .left()
        .right()
        .down()
        .build("Orbital 120MM HE Barrage")
}

pub const fn orbital_380mm_he_barrage() -> Strategem {
    Strategem::builder()
        .right()
        .down()
        .up()
        .up()
        .left()
        .down()
        .down()
        .build("Orbital 380MM HE Barrage")
}

pub const fn orbital_walking_barrage() -> Strategem {
    Strategem::builder()
        .right()
        .down()
        .right()
        .down()
        .right()
        .down()
        .build("Orbital Walking Barrage")
}

pub const fn orbital_laser() -> Strategem {
    Strategem::builder()
        .right()
        .down()
        .up()
        .right()
        .down()
        .build("Orbital Laser")
}

pub const fn orbital_railcannon_strike() -> Strategem {
    Strategem::builder()
        .right()
        .up()
        .down()
        .down()
        .right()
        .build("Orbital Railcannon Strike")
}

pub const fn orbital_precision_strike() -> Strategem {
    Strategem::builder()
        .right()
        .right()
        .up()
        .build("Orbital Precision Strike")
}

pub const fn orbital_gas_strike() -> Strategem {
    Strategem::builder()
        .right()
        .right()
        .down()
        .right()
        .build("Orbital Gas Strike")
}

pub const fn orbital_ems_strike() -> Strategem {
    Strategem::builder()
        .right()
        .right()
        .left()
        .down()
        .build("Orbital EMS Strike")
}

pub const fn orbital_smoke_strike() -> Strategem {
    Strategem::builder()
        .right()
        .right()
        .down()
        .up()
        .build("Orbital Smoke Strike")
}

pub const fn eagle_strafing_run() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .right()
        .build("Eagle Strafing Run")
}

pub const fn eagle_air_strike() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .down()
        .right()
        .build("Eagle Air Strike")
}

pub const fn eagle_cluster_bomb() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .down()
        .down()
        .right()
        .build("Eagle Cluster Bomb")
}

pub const fn eagle_napalm_airstrike() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .down()
        .up()
        .build("Eagle Napalm Airstrike")
}

pub const fn eagle_smoke_strike() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .up()
        .down()
        .build("Eagle Smoke Strike")
}

pub const fn eagle_110mm_rocket_pods() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .up()
        .left()
        .build("Eagle 110MM Rocket Pods")
}

pub const fn eagle_500kg_bomb() -> Strategem {
    Strategem::builder()
        .up()
        .right()
        .down()
        .down()
        .down()
        .build("Eagle 500KG Bomb")
}
