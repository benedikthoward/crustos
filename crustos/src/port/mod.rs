
pub trait Port {
    type Context: PortContext;

    fn start_first_task() -> !;
    unsafe fn restore_first_task();
    unsafe fn context_switch_handler();
    fn request_context_swtich();

    fn disable_interrupts();
    fn enable_interrupts();
    fn raise_interrupt_mask() -> usize;
    fn lower_interrupt_mask(mask: usize);

    fn is_in_interrupt() -> bool;
    fn count_leading_zeros(bitmap: uszie) -> u8;

    fn init_stack(stack: &mut [usize], entry: fn(&mut ()), param: &mut ()) -> &mut usize;
    fn setup_tick_timer(tick_rate_hz: u32, clock_rate_hz: u32);
}
