use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use winapi::um::winuser::{LockWorkStation, MessageBoxW, MB_ICONINFORMATION, MB_OK};
use std::thread;

/// 锁屏状态结构体
pub struct LockState {
    last_locked: Option<Instant>,
    lock_duration: Duration,
}

impl LockState {
    pub fn new(lock_duration: Duration) -> Self {
        Self {
            last_locked: None,
            lock_duration,
        }
    }

    /// 锁屏
    fn lock_screen(&mut self) {
        unsafe {
            if LockWorkStation() == 0 {
                eprintln!("锁屏失败");
            } else {
                println!("锁屏成功");
                self.last_locked = Some(Instant::now());
            }
        }
    }

    /// 当前状态是否可锁屏
    fn can_unlock(&self) -> bool {
        match self.last_locked {
            // 当前状态较锁屏维持时间大于预设值时，解锁
            Some(last_locked) => Instant::now().duration_since(last_locked) >= self.lock_duration,
            None => true,
        }
    }
}

/// 解锁管理结构体
pub struct LockManager {
    state: Arc<Mutex<LockState>>,
}

impl LockManager {
    pub fn new(lock_duration: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(LockState::new(lock_duration)))
        }
    }

    pub fn lock_screen(&self) {
        let mut state = self.state.lock().unwrap();
        state.lock_screen();
    }

    pub fn can_unlock(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.can_unlock()
    }
}

/// 字符串转换为宽字符（UTF-16）
fn wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

/// 显示系统弹窗
fn show_message(message: &str, title: &str) {
    unsafe {
        MessageBoxW(
            null_mut(),
            wide_string(message).as_ptr() as *const u16,
            wide_string(title).as_ptr() as *const u16,
            MB_OK | MB_ICONINFORMATION);
    }
}

/// 显示提示框并在事件后开始锁屏
pub fn begin_lock(lock_duration: Duration) {
    let lock_manager = LockManager::new(lock_duration);

    show_message("将在5s后锁屏", "锁屏提示");

    // 等待五s
    thread::sleep(Duration::from_secs(5));
    lock_manager.lock_screen();

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lock() {
        begin_lock(Duration::from_secs(1));
    }
}