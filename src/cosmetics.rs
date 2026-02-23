pub const CLOAKS: &[&str] = &[
    "mercedes_flow",
    "glitch",
    "crimson_mark",
    "bmw",
    "amg",
    "amg_petronas",
    "ferrari",
    "redbull",
];

pub const HATS: &[&str] = &["horns_black", "horns_white", "halo", "halo_black"];

enum CosmeticKind {
    Hat,
    Cloak,
}

async fn buy(kind: CosmeticKind, id: &str) -> Result<String, String> {
    Ok(format!("Item bought successfully, congrats!"))
}

async fn equip(kind: CosmeticKind, id: &str) -> Result<String, String> {
    Ok(format!("Item equipped"))
}
