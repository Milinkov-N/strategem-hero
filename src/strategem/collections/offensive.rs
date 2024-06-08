use crate::strategem::{Strategem, StrategemClass};

pub const fn orbital_gatling_barrage() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .right()
        .down()
        .left()
        .up()
        .up()
        .build("Orbital Gatling Barrage")
}

pub const fn orbital_airburst_strike() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .right()
        .right()
        .right()
        .build("Orbital Airburst Strike")
}

pub const fn orbital_120mm_he_barrage() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .right()
        .right()
        .down()
        .left()
        .right()
        .down()
        .build("Orbital 120MM HE Barrage")
}

pub const fn orbital_380mm_he_barrage() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
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
    Strategem::builder(StrategemClass::Offensive)
        .right()
        .down()
        .right()
        .down()
        .right()
        .down()
        .build("Orbital Walking Barrage")
}

pub const fn orbital_laser() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .right()
        .down()
        .up()
        .right()
        .down()
        .build("Orbital Laser")
}

pub const fn orbital_railcannon_strike() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .right()
        .up()
        .down()
        .down()
        .right()
        .build("Orbital Railcannon Strike")
}

pub const fn orbital_precision_strike() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .right()
        .right()
        .up()
        .build("Orbital Precision Strike")
}

pub const fn orbital_gas_strike() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .right()
        .right()
        .down()
        .right()
        .build("Orbital Gas Strike")
}

pub const fn orbital_ems_strike() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .right()
        .right()
        .left()
        .down()
        .build("Orbital EMS Strike")
}

pub const fn orbital_smoke_strike() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .right()
        .right()
        .down()
        .up()
        .build("Orbital Smoke Strike")
}

pub const fn eagle_strafing_run() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .up()
        .right()
        .right()
        .build("Eagle Strafing Run")
}

pub const fn eagle_air_strike() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .up()
        .right()
        .down()
        .right()
        .build("Eagle Air Strike")
}

pub const fn eagle_cluster_bomb() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .up()
        .right()
        .down()
        .down()
        .right()
        .build("Eagle Cluster Bomb")
}

pub const fn eagle_napalm_airstrike() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .up()
        .right()
        .down()
        .up()
        .build("Eagle Napalm Airstrike")
}

pub const fn eagle_smoke_strike() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .up()
        .right()
        .up()
        .down()
        .build("Eagle Smoke Strike")
}

pub const fn eagle_110mm_rocket_pods() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .up()
        .right()
        .up()
        .left()
        .build("Eagle 110MM Rocket Pods")
}

pub const fn eagle_500kg_bomb() -> Strategem {
    Strategem::builder(StrategemClass::Offensive)
        .up()
        .right()
        .down()
        .down()
        .down()
        .build("Eagle 500KG Bomb")
}
