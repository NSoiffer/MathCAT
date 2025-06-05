#[cfg(feature = "enable-logs")]
use std::sync::Once;
#[cfg(feature = "enable-logs")]
static INIT: Once = Once::new();

pub(crate) fn enable_logs() {
    #[cfg(feature = "enable-logs")]
    INIT.call_once(||{
        #[cfg(target_os = "android")]
        {
            extern crate log;
            extern crate android_logger;
            
            use log::*;
            use android_logger::*;
        
            android_logger::init_once(
                Config::default()
                .with_max_level(LevelFilter::Trace)
                .with_tag("MathCat")
            );    
            trace!("Activated Android logger!");  
        }    
    });
}
