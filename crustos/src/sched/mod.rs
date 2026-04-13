mod fixed_priority;
use crate::task::TaskControlBlock;

pub trait SchedContext {
    fn compare(&self, other: &Self) -> core::cmp::Ordering;
}
pub trait Scheduler {

    type Context: SchedContext;

    pub fn new() -> Self;
    pub fn pick_next_task(&self) -> *const TaskControlBlock;
    pub fn on_tick(&mut self);
    pub fn add_task(&mut self, task: &mut TaskControlBlock);
    pub fn remove_task(&mut self, task: &mut TaskControlBlock);
    pub fn on_task_yield(&mut self, task: &mut TaskControlBlock);
    pub fn task_blocks_on(&mut self, holder: &mut TaskControlBlock, blocker: &mut TaskControlBlock);
    pub fn task_release_resource(&mut self, holder: &mut TaskControlBlock);

}
