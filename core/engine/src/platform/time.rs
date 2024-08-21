#[cfg(not(target_family = "wasm"))]
mod imp {
    use std::time::Instant;
    use std::time::Duration;

    pub struct Performance {
        std: Instant,
    }
    
    impl Performance {
        pub fn now() -> Self {
            Self {
                std: Instant::now(),
            }
        }
    
        pub fn elapsed(&self) -> Duration {
            self.std.elapsed()
        }
    }
}

#[cfg(target_family = "wasm")]
mod imp {
    use std::time::Duration;
    use js_sys::Date;

    pub struct Performance {
        millis: f64,
    }
    
    impl Performance {
        pub fn now() -> Self {
            Self {
                millis: Date::now(),
            }
        }
    
        pub fn elapsed(&self) -> Duration {
            Duration::from_millis((Date::now() - self.millis) as u64)
        }
    }
}

pub use imp::*;