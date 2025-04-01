use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::fmt;

pub struct CodeProfiler {
    measurements: HashMap<String, Vec<Duration>>,
}

impl CodeProfiler {
    pub fn new() -> Self {
        Profiler {
            measurements: HashMap::new();
        }
    }
}