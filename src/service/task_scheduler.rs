use crate::config::scheduler::SchedulerConfig;

#[derive(Clone)]
pub struct SchedulerService {
    pub config: SchedulerConfig,
}

impl SchedulerService {
    pub fn new(config: SchedulerConfig) -> Self {
        Self { config }
    }
}
