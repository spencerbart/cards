fn main() {
    tonic_build::configure()
        .type_attribute("cards.Card.Suit", "#[derive(strum_macros::EnumIter)]")
        .type_attribute("cards.Card.Rank", "#[derive(strum_macros::EnumIter)]")
        .compile(&["./proto/cards.proto"], &["./proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    println!("cargo:rerun-if-changed=migrations");
}
