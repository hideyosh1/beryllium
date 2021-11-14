use core::ptr::NonNull;

use alloc::{boxed::Box, string::String};
use fermium::{
  c_void,
  prelude::{
    SDL_CreateWindow, SDL_Vulkan_GetVkGetInstanceProcAddr, SDL_Window, VkInstance,
    SDL_WINDOWPOS_CENTERED,
  },
};
use zstring::ZStr;

use crate::{
  get_error,
  init::Sdl,
  window::{Window, WindowFlags},
  SdlError, SdlResult,
};

#[repr(C)]
pub struct VkWindow {
  pub(crate) win: NonNull<SDL_Window>,
  #[allow(unused)]
  sdl: Sdl,
}
impl core::ops::Deref for VkWindow {
  type Target = Window;
  fn deref(&self) -> &Self::Target {
    unsafe { core::mem::transmute(self) }
  }
}

#[allow(non_camel_case_types)]
pub type vkGetInstanceProcAddr_t =
  unsafe extern "system" fn(instance: VkInstance, pName: Option<ZStr<'_>>) -> *mut c_void;

impl Sdl {
  #[inline]
  pub fn create_vk_window(
    &self, title: ZStr<'_>, position: Option<(i32, i32)>, (w, h): (i32, i32), flags: WindowFlags,
  ) -> SdlResult<VkWindow> {
    if (flags & (WindowFlags::OPENGL | WindowFlags::METAL)).0 .0 != 0 {
      return Err(SdlError(Box::new(String::from(
        "beryllium: You can't specify the OPENGL or METAL window flags on a Vulkan window",
      ))));
    }
    let (x, y) = position.unwrap_or((SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED));
    match NonNull::new(unsafe {
      SDL_CreateWindow(title.as_ptr().cast(), x, y, w, h, (WindowFlags::VULKAN | flags).0 .0)
    }) {
      Some(win) => Ok(VkWindow { win, sdl: self.clone() }),
      None => Err(get_error()),
    }
  }
}

impl VkWindow {
  #[inline]
  #[must_use]
  #[allow(non_snake_case)]
  pub fn get_vkGetInstanceProcAddr(&self) -> SdlResult<vkGetInstanceProcAddr_t> {
    unsafe {
      core::mem::transmute::<*mut c_void, Option<vkGetInstanceProcAddr_t>>(
        SDL_Vulkan_GetVkGetInstanceProcAddr(),
      )
    }
    .ok_or_else(|| get_error())
  }
}
