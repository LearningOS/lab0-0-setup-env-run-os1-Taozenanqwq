mod context;
mod switch;
#[allow(clippy::module_inception)]
mod task;

use switch::__switch;
use context::TaskContext;
use crate::sync::UPSafeCell;
pub use task::{TaskControlBlock,TaskStatus,TaskManager,TaskManagerInner};
use crate::loader::{get_num_app,init_app_cx};
use crate::config::MAX_APP_NUM;
use lazy_static::*;

lazy_static! {
  pub static ref TASK_MANAGER: TaskManager = {
      let num_app = get_num_app();
      let mut tasks = [
          TaskControlBlock {
              task_cx: TaskContext::zero_init(),
              task_status: TaskStatus::UnInit
          };
          MAX_APP_NUM
      ];
      for i in 0..num_app {
          tasks[i].task_cx = TaskContext::goto_restore(init_app_cx(i));
          tasks[i].task_status = TaskStatus::Ready;
      }
      TaskManager {
          num_app,
          inner: unsafe { UPSafeCell::new(TaskManagerInner {
              tasks,
              current_task: 0,
          })},
      }
  };
}

pub fn suspend_current_and_run_next() {
  mark_current_suspended();
  run_next_task();
}

pub fn exit_current_and_run_next() {
  mark_current_exited();
  run_next_task();
}

fn mark_current_suspended() {
  TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
  TASK_MANAGER.mark_current_exited();
}

impl TaskManager {
  fn mark_current_suspended(&self) {
      let mut inner = self.inner.exclusive_access();
      let current = inner.current_task;
      inner.tasks[current].task_status = TaskStatus::Ready;
  }

  fn mark_current_exited(&self) {
      let mut inner = self.inner.exclusive_access();
      let current = inner.current_task;
      inner.tasks[current].task_status = TaskStatus::Exited;
  }
   fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        // before this, we should drop local variables that must be dropped manually
        unsafe {
            __switch(
                &mut _unused as *mut TaskContext,
                next_task_cx_ptr,
            );
        }
        panic!("unreachable in run_first_task!");
    }
    fn run_next_task(&self) {
      if let Some(next) = self.find_next_task() {
          let mut inner = self.inner.exclusive_access();
          let current = inner.current_task;
          inner.tasks[next].task_status = TaskStatus::Running;
          inner.current_task = next;
          let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
          let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
          drop(inner);
          // before this, we should drop local variables that must be dropped manually
          unsafe {
              __switch(
                  current_task_cx_ptr,
                  next_task_cx_ptr,
              );
          }
          // go back to user mode
      } else {
          panic!("All applications completed!");
      }
  }
fn find_next_task(&self) -> Option<usize> {
      let inner = self.inner.exclusive_access();
      let current = inner.current_task;
      (current + 1..current + self.num_app + 1)
          .map(|id| id % self.num_app)
          .find(|id| {
              inner.tasks[*id].task_status == TaskStatus::Ready
          })
  }
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

fn run_next_task() {
  TASK_MANAGER.run_next_task();
}
