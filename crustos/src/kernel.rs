
pub struct PerCoreState {

    current_task: *mut TaskControlBlock, 
    idle_task: TaskControlBlock,
    yield_pending: bool,

}

pub struct Kernel {
    // Per Core
    cores: [PerCoreState, MAX_CORES],

    scheduler: ActiveScheduler,

    //shared
    delayed_list: List,
    suspended_list: List,
    terminated_list: List, 
    pending_ready_list: List,

    tick_count: u64,


    scheduler_running: bool, 
    scheduler_suspended: bool,
    task_count: u16,

}
