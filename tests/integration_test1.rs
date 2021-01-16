use rust_boilerplate;


#[test]
fn test_config_and_utils() {
    let cfg = rust_boilerplate::config::Config::default();

    let sum = rust_boilerplate::utils::add(cfg.number, cfg.number);

    assert_eq!(10, sum);
}