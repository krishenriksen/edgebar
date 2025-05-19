use anyhow::Context;
use tauri::{PhysicalPosition, PhysicalSize, Runtime, Window};
use windows::Win32::{
  Foundation::HWND,
  UI::WindowsAndMessaging::{SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW},
};

use super::app_bar;
use crate::config::DockEdge;

pub trait WindowExtWindows {
  fn set_tool_window(&self, enable: bool) -> anyhow::Result<()>;

  fn allocate_app_bar(
    &self,
    size: PhysicalSize<i32>,
    position: PhysicalPosition<i32>,
    edge: DockEdge,
  ) -> anyhow::Result<(PhysicalSize<i32>, PhysicalPosition<i32>)>;

  fn deallocate_app_bar(&self) -> anyhow::Result<()>;
}

impl<R: Runtime> WindowExtWindows for Window<R> {
  fn set_tool_window(&self, enable: bool) -> anyhow::Result<()> {
    let hwnd = self.hwnd().context("Failed to get window handle.")?;

    unsafe {
      let ex_style = if enable {
        WS_EX_TOOLWINDOW.0 as isize
      } else {
        WS_EX_APPWINDOW.0 as isize
      };

      SetWindowLongPtrW(HWND(hwnd.0), GWL_EXSTYLE, ex_style);
    }

    Ok(())
  }

  fn allocate_app_bar(
    &self,
    size: PhysicalSize<i32>,
    position: PhysicalPosition<i32>,
    edge: DockEdge,
  ) -> anyhow::Result<(PhysicalSize<i32>, PhysicalPosition<i32>)> {
    let handle = self.hwnd().context("Failed to get window handle.")?;
    app_bar::create_app_bar(handle.0 as _, size, position, edge)
  }

  fn deallocate_app_bar(&self) -> anyhow::Result<()> {
    let handle = self.hwnd().context("Failed to get window handle.")?;
    app_bar::remove_app_bar(handle.0 as _)
  }
}
