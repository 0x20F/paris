use paris::Logger;
use paris::LogIcon;



#[test]
fn test_log() {
    let logger = Logger::new(false);
    logger.log("This is the most basic of texts.");
}


#[test]
fn test_info() {
    let logger = Logger::new(false);
    logger.info("This is some info, it should work");
}


#[test]
fn test_success() {
    let logger = Logger::new(true);
    logger.success("You did it and nothing broke!!");
}


#[test]
fn test_error() {
    let logger = Logger::new(false);
    logger.error("This is an error, but test should still work");
}


#[test]
fn test_warning() {
    let logger = Logger::new(true);
    logger.warning("This is a warning, watch it");
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