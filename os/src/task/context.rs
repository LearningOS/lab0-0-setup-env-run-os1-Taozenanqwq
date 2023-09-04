
global_asm!(include_str!("switch.S"));

use super::TaskContext;
pub struct TaskContext {
  ra: usize,
  sp: usize,
  s: [usize; 12],
}


extern "C" {
    pub fn __switch(
        current_task_cx_ptr: *mut TaskContext,
        next_task_cx_ptr: *const TaskContext
    );
}