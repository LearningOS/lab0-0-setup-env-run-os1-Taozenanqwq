use super::TaskContext;
use crate::sync::UPSafeCell;
use crate::config::MAX_APP_NUM;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit, // 未初始化
    Ready, // 准备运行
    Running, // 正在运行
    Exited, // 已退出
}

#[derive(Copy,Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
}

pub struct TaskManager {
  pub num_app: usize,
  pub inner: UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner {
  pub tasks: [TaskControlBlock; MAX_APP_NUM],
  pub current_task: usize,
}
