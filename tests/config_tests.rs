use changelog_gen::config::loader::load_config;

#[test]
fn default_config_loads() {
    let cfg = load_config().unwrap();
    // Config successfully loads from default
    assert_eq!(cfg.notifications, None);
    assert_eq!(cfg.telemetry, None);
}
