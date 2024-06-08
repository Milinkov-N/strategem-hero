use crate::strategem::{Strategem, StrategemClass};

pub const fn emg101_hmg_emplacement() -> Strategem {
    Strategem::builder(StrategemClass::Defensive)
        .down()
        .up()
        .left()
        .right()
        .right()
        .left()
        .build("E/MG-101 HMG Emplacement")
}

pub const fn fx12_shield_generator_relay() -> Strategem {
    Strategem::builder(StrategemClass::Defensive)
        .down()
        .down()
        .left()
        .right()
        .left()
        .right()
        .build("FX-12 Shield Generator Relay")
}

pub const fn aarc3_tesla_tower() -> Strategem {
    Strategem::builder(StrategemClass::Defensive)
        .down()
        .up()
        .right()
        .up()
        .left()
        .right()
        .build("A/ARC-3 Tesla Tower")
}

pub const fn md6_anti_personnel_minefield() -> Strategem {
    Strategem::builder(StrategemClass::Defensive)
        .down()
        .left()
        .up()
        .right()
        .build("MD-6 Anti-Personnel Minefield")
}

pub const fn mdi4_incendiary_mines() -> Strategem {
    Strategem::builder(StrategemClass::Defensive)
        .down()
        .left()
        .left()
        .down()
        .build("MD-I4 Incendiary Mines")
}

pub const fn amg43_machine_gun_sentry() -> Strategem {
    Strategem::builder(StrategemClass::Defensive)
        .down()
        .up()
        .right()
        .right()
        .up()
        .build("A/MG-43 Machine Gun Sentry")
}

pub const fn ag16_galting_sentry() -> Strategem {
    Strategem::builder(StrategemClass::Defensive)
        .down()
        .up()
        .right()
        .left()
        .build("A/G-16 Gatling Sentry")
}

pub const fn am12_mortar_sentry() -> Strategem {
    Strategem::builder(StrategemClass::Defensive)
        .down()
        .up()
        .right()
        .right()
        .down()
        .build("A/M-12 Mortar Sentry")
}

pub const fn aac8_autocannon_sentry() -> Strategem {
    Strategem::builder(StrategemClass::Defensive)
        .down()
        .up()
        .right()
        .up()
        .left()
        .up()
        .build("A/AC-8 Autocannon Sentry")
}

pub const fn amls4x_rocket_sentry() -> Strategem {
    Strategem::builder(StrategemClass::Defensive)
        .down()
        .up()
        .right()
        .right()
        .left()
        .build("A/MLS-4X Rocket Sentry")
}

pub const fn am23_ems_mortar_sentry() -> Strategem {
    Strategem::builder(StrategemClass::Defensive)
        .down()
        .up()
        .right()
        .down()
        .right()
        .build("A/M-23 EMS Mortar Sentry")
}
