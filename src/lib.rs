pub mod prelude {
    pub use crate::{benchmark, init_log};

    // To reexport procedural macros, the root of the crate (`self`) must be exported.
    pub use anyhow::{anyhow, Context, Result};
    pub use clap::{self, Parser};
    pub use colog;
    pub use log::{self, error, warn, info, debug, trace};
    pub use rand::{self, prelude::*};
    pub use rayon::{self, prelude::*, iter::{IntoParallelIterator, ParallelIterator}};
    pub use tokio;
    pub use walkdir::{self, WalkDir};
}

#[macro_export]
macro_rules! init_log {
    ($filter_module:expr) => {{
        if cfg!(debug_assertions) {
            colog::basic_builder().filter_level(log::LevelFilter::Info).filter_module($filter_module, log::LevelFilter::Trace).init();
        } else {
            colog::init();
        };
    }};
    () => {{
        if cfg!(debug_assertions) {
            colog::basic_builder().filter_level(log::LevelFilter::Trace).init();
        } else {
            colog::init();
        };
    }}
}

#[macro_export]
macro_rules! benchmark {
    ($msg:expr, $code:block) => {{
        if cfg!(debug_assertions) {
            let start = std::time::Instant::now();
            let result = $code;
            let duration = start.elapsed();

            let msg = (|| $msg)();
            info!("{} {:?}", msg, duration);

            result
        } else {
            $code
        }
    }};
}