fn main() {
    // abort compilation if both features active, because both are mutually exclusive
    // https://doc.rust-lang.org/cargo/reference/features.html#mutually-exclusive-features
    #[cfg(all(feature = "atan2_deg3", feature = "atan2_deg5"))]
    compile_error!("Features `atan2_deg3` and `atan2_deg5` cannot be enabled at the same time.");
}
