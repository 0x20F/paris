extern crate paris;


use paris::Logger;




#[test]
fn test_info() {
    let logger = Logger::new();
    logger.info("This is some info, it should work");
}


#[test]
fn test_error() {
    let logger = Logger::new();
    logger.error("This is an error, but test should still work");
}


#[test]
fn test_warning() {
    let logger = Logger::new();
    logger.warning("This is a warning, watch it");
}


#[test]
fn test_attention() {
    let logger = Logger::new();
    logger.attention("Pay attention to this");
}


#[test]
fn test_panic() {
    let logger = Logger::new();
    logger.panic("You should die now", |c| assert_eq!(c, 0x0100));
}