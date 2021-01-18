use rust_boilerplate;


#[test]
fn test_config_and_utils() {
    rust_boilerplate::utils::logging::init_logger();

    let cfg = rust_boilerplate::config::Config::default();
    cfg.print();

    let mut line = String::new();

    let mut bytes_read = 0;
    while bytes_read == 0 {
        log::warn!("Did the logger display the config? (y/n):");
        bytes_read = std::io::stdin().read_line(&mut line).unwrap();
    }

    assert_eq!("y\n", line);
}