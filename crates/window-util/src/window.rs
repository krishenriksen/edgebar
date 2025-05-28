use once_cell::sync::OnceCell;
use std::sync::Mutex;
use tokio::{
  sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
  task,
  time::{Duration, sleep},
};
use windows::Win32::{
  Foundation::HWND,
  System::LibraryLoader::GetModuleHandleW,
  UI::{
    Accessibility::{SetWinEventHook, UnhookWinEvent},
    WindowsAndMessaging::{
      DispatchMessageW, EVENT_SYSTEM_FOREGROUND, GetWindowTextLengthW, GetWindowTextW, MSG,
      PM_REMOVE, PeekMessageW, SetForegroundWindow, TranslateMessage,
    },
  },
};

/// Represents a foreground window event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowEvent {
  pub hwnd: isize,
  pub title: String,
}

#[derive(Debug)]
pub struct Window {
  event_rx: UnboundedReceiver<WindowEvent>,
  #[allow(dead_code)]
  event_tx: UnboundedSender<WindowEvent>,
}

// Define EVENT_TX globally using OnceCell
static EVENT_TX: OnceCell<Mutex<Option<UnboundedSender<WindowEvent>>>> = OnceCell::new();

impl Window {
  pub fn new() -> crate::Result<Self> {
    let (event_tx, event_rx) = mpsc::unbounded_channel();

    // Initialize the static EVENT_TX if not already initialized.
    EVENT_TX.get_or_init(|| Mutex::new(None));
    *EVENT_TX.get().unwrap().lock().unwrap() = Some(event_tx.clone());

    Self::start_window_event_listener();

    Ok(Window { event_rx, event_tx })
  }
  /// Returns the next event from the `Window`.
  pub async fn events(&mut self) -> Option<WindowEvent> {
    while let Some(event) = self.event_rx.recv().await {
      if let Some(event) = self.on_event(event) {
        return Some(event);
      }
    }
    None
  }

  /// Returns the next event from the `Window` (synchronously).
  pub fn events_blocking(&mut self) -> Option<WindowEvent> {
    while let Some(event) = self.event_rx.blocking_recv() {
      if let Some(event) = self.on_event(event) {
        return Some(event);
      }
    }
    None
  }

  fn on_event(&mut self, event: WindowEvent) -> Option<WindowEvent> {
    // Example: Filter out events with empty titles
    if event.title.is_empty() {
      println!("Ignored event with empty title: {:?}", event);
      return None;
    }
    Some(event)
  }

  fn start_window_event_listener() {
    task::spawn_blocking(move || {
      if let Some(_hook) = WinEventHook::new() {
        println!("Listening for window events...");

        let mut msg = MSG::default();
        loop {
          let has_message =
            unsafe { PeekMessageW(&mut msg, HWND(std::ptr::null_mut()), 0, 0, PM_REMOVE).into() };

          if has_message {
            unsafe {
              let _ = TranslateMessage(&msg);
              DispatchMessageW(&msg);
            }
          } else {
            tokio::runtime::Handle::current().block_on(sleep(Duration::from_millis(10)));
          }
        }

        // The `hook` variable remains in scope until the thread exits,
        // ensuring the `Drop` implementation is called.
      }
    });
  }

  pub fn set_foreground_window(hwnd: isize) -> anyhow::Result<(), String> {
    unsafe {
      let hwnd = HWND(hwnd as *mut _); // Convert `isize` to a raw pointer
      if SetForegroundWindow(hwnd).as_bool() {
        Ok(())
      } else {
        Err("Failed to set foreground window".to_string())
      }
    }
  }
}

impl Drop for Window {
  fn drop(&mut self) {
    if let Some(mutex) = EVENT_TX.get() {
      *mutex.lock().unwrap() = None;
    }
  }
}

// Struct to manage the lifetime of the event hook
struct WinEventHook {
  hook: windows::Win32::UI::Accessibility::HWINEVENTHOOK,
}

impl WinEventHook {
  fn new() -> Option<Self> {
    let hook = unsafe {
      SetWinEventHook(
        EVENT_SYSTEM_FOREGROUND,
        EVENT_SYSTEM_FOREGROUND,
        GetModuleHandleW(None).unwrap(),
        Some(event_callback),
        0,
        0,
        0,
      )
    };

    if hook.0.is_null() {
      eprintln!("Failed to set foreground event hook");
      None
    } else {
      Some(Self { hook })
    }
  }
}

impl Drop for WinEventHook {
  fn drop(&mut self) {
    unsafe {
      let _ = UnhookWinEvent(self.hook);
    }
  }
}

// Callback function for foreground window changes
unsafe extern "system" fn event_callback(
  _: windows::Win32::UI::Accessibility::HWINEVENTHOOK,
  _: u32,
  hwnd: HWND,
  _: i32,
  _: i32,
  _: u32,
  _: u32,
) {
  // List of window titles to ignore
  static IGNORED_TITLES: &[&str] = &["EdgeBar - macos/macos", "Dropdown - EdgeBar", "Tauri App"];

  // Retrieve the window title
  let length = unsafe { GetWindowTextLengthW(hwnd) } + 1;
  let mut buffer = vec![0u16; length as usize];
  let copied_length = unsafe { GetWindowTextW(hwnd, &mut buffer) };

  if copied_length > 0 {
    let window_title = String::from_utf16_lossy(&buffer[..copied_length as usize]);

    // Ignore events with any of the specific titles (exact match)
    if IGNORED_TITLES.iter().any(|t| window_title == *t) {
      println!("Ignored window with title: {:?}", window_title);
      return;
    }

    // Access the static `EVENT_TX` to send the event
    if let Some(mutex) = EVENT_TX.get() {
      if let Some(sender) = mutex.lock().unwrap().as_ref() {
        if let Err(err) = sender.send(WindowEvent {
          hwnd: hwnd.0 as isize,
          title: window_title,
        }) {
          eprintln!("Failed to send event: {}", err);
        }
      }
    }
  }
}
