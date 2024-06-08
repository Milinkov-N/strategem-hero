use crate::strategem::{Strategem, StrategemClass};

pub const fn lift850_jump_pack() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .up()
        .up()
        .down()
        .up()
        .build("LIFT-850 Jump Pack")
}

pub const fn b1_supply_pack() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .left()
        .down()
        .up()
        .up()
        .down()
        .build("B-1 Supply Pack")
}

pub const fn axlas5_guard_dog_rover() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .up()
        .left()
        .up()
        .right()
        .right()
        .build("AX/LAS-5 \"Guard Dog\" Rover")
}

pub const fn sh20_ballistic_shield_backpack() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .left()
        .down()
        .down()
        .up()
        .left()
        .build("SH-20 Ballistic Shield Backpack")
}

pub const fn sh32_shield_generator_pack() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .up()
        .left()
        .right()
        .left()
        .right()
        .build("SH-32 Shield Generator Pack")
}

pub const fn axar23_guard_dog() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .up()
        .left()
        .up()
        .right()
        .down()
        .build("AX/AR-23 \"Guard Dog\"")
}

pub const fn mg43_machine_gun() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .left()
        .down()
        .up()
        .right()
        .build("MG-43 Machine Gun")
}

pub const fn apw1_antimateriel_rifle() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .left()
        .right()
        .up()
        .down()
        .build("APW-1 Anti-Materiel Rifle")
}

pub const fn m105_stalwart() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .left()
        .down()
        .up()
        .up()
        .left()
        .build("M-105 Stalwart")
}

pub const fn eat17_expendable_antitank() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .down()
        .left()
        .up()
        .right()
        .build("EAT-17 Expendable Anti-tank")
}

pub const fn gr8_recoilless_rifle() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .left()
        .right()
        .right()
        .left()
        .build("GR-8 Recoilless Rifle")
}

pub const fn flam40_flamethrower() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .left()
        .up()
        .down()
        .up()
        .build("FLAM-40 Flamethrower")
}

pub const fn ac8_autocannon() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .left()
        .down()
        .up()
        .up()
        .right()
        .build("AC-8 Autocannon")
}

pub const fn mg206_heavy_machine_gun() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .left()
        .up()
        .down()
        .down()
        .build("MG-206 Heavy Machine Gun")
}

pub const fn rs422_railgun() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .right()
        .down()
        .up()
        .left()
        .right()
        .build("RS-422 Railgun")
}

pub const fn faf14_spear_launcher() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .down()
        .up()
        .down()
        .down()
        .build("FAF-14 SPEAR Launcher")
}

pub const fn gl21_grenade_launcher() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .left()
        .up()
        .left()
        .down()
        .build("GL-21 Grenade Launcher")
}

pub const fn las98_laser_cannon() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .left()
        .down()
        .up()
        .left()
        .build("LAS-98 Laser Cannon")
}

pub const fn arc3_arc_thrower() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .right()
        .down()
        .up()
        .left()
        .left()
        .build("ARC-3 Arc Thrower")
}

pub const fn las99_quasar_cannon() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .down()
        .up()
        .left()
        .right()
        .build("LAS-99 Quasar Cannon")
}

pub const fn rl77_airburst_rocket_launcher() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .down()
        .up()
        .up()
        .left()
        .right()
        .build("RL-77 Airburst Rocket Launcher")
}

pub const fn exo45_patriot_exosuit() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .left()
        .down()
        .right()
        .up()
        .left()
        .down()
        .down()
        .build("EXO-45 Patriot Exosuit")
}

pub const fn exo45_emancipator_exosuit() -> Strategem {
    Strategem::builder(StrategemClass::Supply)
        .left()
        .down()
        .right()
        .up()
        .left()
        .down()
        .up()
        .build("EXO-49 Emancipator Exosuit")
}
