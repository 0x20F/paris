extern crate paris;


use paris::Logger;
use paris::LogIcon;



#[test]
fn test_info() {
    let logger = Logger::new();
    logger.info("This is some info, it should work");
}


#[test]
fn test_error() {
    let logger = Logger::new();
    logger
        .error("This is an error, but test should still work");
}


#[test]
fn test_warning() {
    let logger = Logger::new();
    logger.warning("This is a warning, watch it");
}


#[test]
fn test_panic() {
    let logger = Logger::new();
    logger.panic("You should die now", |c| assert_eq!(c, 0x0100));
}


#[test]
fn test_icons() {
    println!(
        "{} All {} Of {} Them {} At {} Once", 
        LogIcon::Info, 
        LogIcon::Tick,
        LogIcon::Cross,
        LogIcon::Warning,
        LogIcon::Heart
    );
}