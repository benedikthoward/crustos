
use crate::lists::List;
use crate::sched::Scheduler;
use crate::sched::SchedContext;

pub struct FixedPriorityScheduler {
    ready_list: [List; CONFIGU_MAX_PRIORITY],
}

pub struct FixedPrioritySchedCtx {
    priority: u8,
}

impl SchedContext for FixedPrioritySchedCtx {
    fn compare(&self, other: &Self) -> core::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
} 

impl Scheduler for FixedPriorityScheduler {
    
}

