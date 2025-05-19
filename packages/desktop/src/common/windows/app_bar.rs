use std::mem::transmute;

use anyhow::bail;
use tauri::{PhysicalPosition, PhysicalSize};
use tracing::info;
use windows::Win32::{
  Foundation::{GetLastError, HWND, LPARAM, LRESULT, RECT, WPARAM},
  UI::{
    Shell::{
      SHAppBarMessage, ABE_BOTTOM, ABE_LEFT, ABE_RIGHT, ABE_TOP, ABM_NEW, ABM_QUERYPOS, ABM_REMOVE,
      ABM_SETPOS, ABN_POSCHANGED, APPBARDATA,
    },
    WindowsAndMessaging::{
      CallWindowProcA, DefWindowProcA, GetWindowRect, RegisterWindowMessageA, SetWindowLongPtrA,
      GWLP_WNDPROC, WNDPROC,
    },
  },
};

use crate::config::DockEdge;

// Store the original window procedure
static mut PREV_WND_PROC: Option<WNDPROC> = None;

// Define a global variable to store the edge.  This is necessary because
// the window proc doesn't have access to the edge directly.
static mut DOCK_EDGE: Option<DockEdge> = None;
static mut CALLBACK_MESSAGE: u32 = 0;

// New window procedure to handle ABN_POSCHANGED
unsafe extern "system" fn new_wnd_proc(
  hwnd: HWND,
  msg: u32,
  wparam: WPARAM,
  lparam: LPARAM,
) -> LRESULT {
  match msg {
    msg if msg == CALLBACK_MESSAGE => {
      if wparam.0 == ABN_POSCHANGED as usize {
        info!("Received ABN_POSCHANGED message");
        // Call our function to update the appbar position
        #[allow(static_mut_refs)]
        let edge = unsafe { DOCK_EDGE.clone() };
        if let Some(edge) = edge {
          if let Err(e) = update_app_bar_position(hwnd, Some(edge)) {
            eprintln!("Error updating app bar position: {}", e);
          }
        }
      }
      // Call the original window procedure to ensure default processing
      if let Some(prev_wnd_proc) = PREV_WND_PROC {
        CallWindowProcA(prev_wnd_proc, hwnd, msg, wparam, lparam)
      } else {
        DefWindowProcA(hwnd, msg, wparam, lparam)
      }
    }
    _ => {
      // For other messages, call the original window procedure
      if let Some(prev_wnd_proc) = PREV_WND_PROC {
        CallWindowProcA(prev_wnd_proc, hwnd, msg, wparam, lparam)
      } else {
        DefWindowProcA(hwnd, msg, wparam, lparam)
      }
    }
  }
}

pub fn update_app_bar_position(hwnd: HWND, edge: Option<DockEdge>) -> anyhow::Result<()> {
  let edge = edge.unwrap_or(DockEdge::Top);
  let mut rect = RECT::default();

  unsafe {
    if GetWindowRect(hwnd, &mut rect).is_err() {
      bail!("GetWindowRect failed: {:?}", GetLastError());
    }
  }

  let size = PhysicalSize::new(rect.right - rect.left, rect.bottom - rect.top);
  let position = PhysicalPosition::new(rect.left, rect.top);

  let mut data = APPBARDATA {
    cbSize: std::mem::size_of::<APPBARDATA>() as u32,
    hWnd: hwnd,
    uCallbackMessage: unsafe { CALLBACK_MESSAGE },
    uEdge: match edge {
      DockEdge::Left => ABE_LEFT,
      DockEdge::Top => ABE_TOP,
      DockEdge::Right => ABE_RIGHT,
      DockEdge::Bottom => ABE_BOTTOM,
    },
    rc: RECT {
      left: position.x,
      top: position.y,
      right: position.x + size.width,
      bottom: position.y + size.height,
    },
    ..Default::default()
  };

  if unsafe { SHAppBarMessage(ABM_QUERYPOS, &mut data) } == 0 {
    bail!("Failed to query for app bar position.");
  }

  data.rc.left = position.x;
  data.rc.top = position.y;
  data.rc.right = position.x + size.width;
  data.rc.bottom = position.y + size.height;

  if unsafe { SHAppBarMessage(ABM_SETPOS, &mut data) } == 0 {
    bail!("Failed to set app bar position.");
  }

  info!("App bar position updated: {:?}", rect);

  Ok(())
}

pub fn create_app_bar(
  window_handle: isize,
  size: PhysicalSize<i32>,
  position: PhysicalPosition<i32>,
  edge: DockEdge,
) -> anyhow::Result<(PhysicalSize<i32>, PhysicalPosition<i32>)> {
  let rect = RECT {
    left: position.x,
    top: position.y,
    right: position.x + size.width,
    bottom: position.y + size.height,
  };

  info!("Creating app bar with initial rect: {:?}", rect);

  unsafe {
    CALLBACK_MESSAGE = RegisterWindowMessageA(windows::core::s!("AppBarMessage"));
    if CALLBACK_MESSAGE == 0 {
      bail!("Failed to register window message: {:?}", GetLastError());
    }
  }

  let mut data = APPBARDATA {
    cbSize: std::mem::size_of::<APPBARDATA>() as u32,
    hWnd: HWND(window_handle as _),
    uCallbackMessage: unsafe { CALLBACK_MESSAGE },
    uEdge: match edge {
      DockEdge::Left => ABE_LEFT,
      DockEdge::Top => ABE_TOP,
      DockEdge::Right => ABE_RIGHT,
      DockEdge::Bottom => ABE_BOTTOM,
    },
    rc: rect.clone(),
    ..Default::default()
  };

  if unsafe { SHAppBarMessage(ABM_NEW, &mut data) } == 0 {
    bail!("Failed to register new app bar.");
  }

  // Store the edge globally
  unsafe {
    DOCK_EDGE = Some(edge);
  }

  // Subclass the window to receive messages. Store the original window
  // proc.
  unsafe {
    let hwnd = HWND(window_handle as _);
    let prev_wnd_proc = SetWindowLongPtrA(hwnd, GWLP_WNDPROC, new_wnd_proc as *const () as isize);
    if prev_wnd_proc != 0 {
      PREV_WND_PROC = Some(transmute(prev_wnd_proc));
    } else {
      let error = GetLastError();
      bail!("SetWindowLongPtrA failed: {:?}", error);
    }
  }

  // Query to get the adjusted position.
  if unsafe { SHAppBarMessage(ABM_QUERYPOS, &mut data) } == 0 {
    bail!("Failed to query for app bar position.");
  }

  let adjusted_position = PhysicalPosition::new(data.rc.left, data.rc.top);

  let width_delta = match edge {
    DockEdge::Left => rect.right - data.rc.right,
    DockEdge::Right => data.rc.left - rect.left,
    _ => (rect.right - data.rc.right) - (rect.left - data.rc.left),
  };

  let height_delta = match edge {
    DockEdge::Top => rect.bottom - data.rc.bottom,
    DockEdge::Bottom => data.rc.top - rect.top,
    _ => (rect.bottom - data.rc.bottom) - (rect.top - data.rc.top),
  };

  // Size has changed if the edge that is not being docked has been
  // adjusted by ABM_QUERYPOS. For example, if the top edge is docked, then
  // diffs in the left and right edges are the size changes.
  let adjusted_size = PhysicalSize::new(size.width - width_delta, size.height - height_delta);

  data.rc = RECT {
    left: adjusted_position.x,
    top: adjusted_position.y,
    right: adjusted_position.x + adjusted_size.width,
    bottom: adjusted_position.y + adjusted_size.height,
  };

  // Set position for it to actually reserve the size and position.
  if unsafe { SHAppBarMessage(ABM_SETPOS, &mut data) } == 0 {
    bail!("Failed to set app bar position.");
  }

  info!("Successfully registered appbar with rect: {:?}", data.rc);

  Ok((adjusted_size, adjusted_position))
}

/// Deallocate the app bar for given window handle.
///
/// Note that this does not error if handle is invalid.
pub fn remove_app_bar(handle: isize) -> anyhow::Result<()> {
  info!("Removing app bar for {:?}.", handle);

  unsafe {
    let hwnd = HWND(handle as _);
    // Restore the original window procedure
    if let Some(prev_wnd_proc) = PREV_WND_PROC {
      SetWindowLongPtrA(hwnd, GWLP_WNDPROC, transmute(prev_wnd_proc));
      PREV_WND_PROC = None; // Clear the global variable
    }
    DOCK_EDGE = None;
  }

  let mut abd = APPBARDATA {
    cbSize: std::mem::size_of::<APPBARDATA>() as u32,
    hWnd: HWND(handle as _),
    uCallbackMessage: 0,
    ..Default::default()
  };

  match unsafe { SHAppBarMessage(ABM_REMOVE, &mut abd) } {
    0 => bail!("Failed to remove app bar."),
    _ => Ok(()),
  }
}
