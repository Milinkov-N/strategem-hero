use crate::strategem::{Strategem, StrategemClass};

pub const fn reinforce() -> Strategem {
    Strategem::builder(StrategemClass::Mission)
        .up()
        .down()
        .right()
        .left()
        .up()
        .build("Reinforce")
}

pub const fn sos_beacon() -> Strategem {
    Strategem::builder(StrategemClass::Mission)
        .up()
        .down()
        .right()
        .up()
        .build("SOS Beacon")
}

pub const fn resupply() -> Strategem {
    Strategem::builder(StrategemClass::Mission)
        .down()
        .down()
        .up()
        .right()
        .build("Resupply")
}

pub const fn nux223_hellbomb() -> Strategem {
    Strategem::builder(StrategemClass::Mission)
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
    Strategem::builder(StrategemClass::Mission)
        .down()
        .down()
        .down()
        .up()
        .up()
        .build("SSSD Delivery")
}

pub const fn seismic_probe() -> Strategem {
    Strategem::builder(StrategemClass::Mission)
        .up()
        .up()
        .left()
        .right()
        .down()
        .down()
        .build("Seismic Probe")
}

pub const fn upload_data() -> Strategem {
    Strategem::builder(StrategemClass::Mission)
        .left()
        .right()
        .up()
        .up()
        .up()
        .build("Upload Data")
}

pub const fn eagle_rearm() -> Strategem {
    Strategem::builder(StrategemClass::Mission)
        .up()
        .up()
        .left()
        .up()
        .right()
        .build("Eagle Rearm")
}

pub const fn illumination_flare() -> Strategem {
    Strategem::builder(StrategemClass::Mission)
        .right()
        .right()
        .left()
        .left()
        .build("Illumination Flare")
}

pub const fn seaf_artillery() -> Strategem {
    Strategem::builder(StrategemClass::Mission)
        .right()
        .up()
        .up()
        .down()
        .build("SEAF Artillery")
}

pub const fn super_earth_flag() -> Strategem {
    Strategem::builder(StrategemClass::Mission)
        .down()
        .up()
        .down()
        .up()
        .build("Super Earth Flag")
}
