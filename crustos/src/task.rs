
pub struct TaskControlBlock {

    //identity
    task_id: u32,
    task_name: [u8; MAX_TASK_NAME_LEN],

    // stack
    stack_pointer: *mut usize, 
    stack_base_pointer: *mut usize, 
    stack_size: usize,

    //sheduling
    priority: u8, 
    base_priority: u8, 
    state: TaskState, 

    //Timing
    wake_tick: u32, 

    // Critical sections
    critical_nesting: u32, 

    // Sync ownership
    mutexes_held: u8, 

    // notifications
    notification_values: [u32; NOTIFICATION_ARRAY_SIZE],
    notification_states: [NotificationState; NOTIFICATION_ARRAY_SIZE],

    // List memeberships
    state_list_node: ListNode,
    event_list_node: ListNode,

    port_context: PortCtx, 
    sched_context: ShedCtx,

}
