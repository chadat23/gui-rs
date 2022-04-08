use std::env;

use wgpu::{Backends, PowerPreference};

/// Represents the backends that gui-rs will use.
#[derive(Debug)]
pub enum GUIBackend {
    /// Supported on Windows, Linux/Android, and macOS/iOS via Vulkan Portability (with the Vulkan feature enabled)
    Vulkan,
    /// Supported on macOS/iOS
    Metal,
    /// Supported on Windows 10
    Dx12,
    /// Supported on Windows 7+ but has a less ritch featureset
    Dx11,
    /// Currently unsupported
    Gl,
    /// Supported when targeting the web through webassembly
    BrowserWebGpu,
}

impl Default for GUIBackend {
    /// Returns an array of Backends in the order in which they
    /// should be tested for compatability with the Adapter.
    fn default() -> Self {
        //https://doc.rust-lang.org/std/env/consts/constant.OS.html
        match env::consts::OS {
            "linux" | "android" => Self::Vulkan,
            "macos" | "ios" => Self::Metal,
            "windows" => Self::Dx12,
            _ => Self::Vulkan,
        }
    }
}

/// Power Preference when choosing a physical adapter.
pub enum GUIPowerPreference {
    /// Adapter that uses the least possible power. This is often an integrated GPU.
    LowPower,
    /// Adapter that has the highest performance. This is often a discrete GPU.
    HighPerformance,
}

impl Default for GUIPowerPreference {
    fn default() -> Self {
        Self::LowPower
    }
}

/// Represents the the resourcess that are to be used while processing the gui.
pub struct GUIResources {
    /// Power Preference when choosing a physical adapter.
    power_preference: GUIPowerPreference,
    /// Represents the backends that gui-rs will use.
    backend: GUIBackend,
}

impl Default for GUIResources {
    fn default() -> Self {
        GUIResources {
            power_preference: GUIPowerPreference::default(),
            backend: GUIBackend::default(),
        }
    }
}

impl GUIResources {
    /// Returns the wgpu PowerPreference that corrisponds to the
    /// previously selected GUIPowerPreference
    pub fn power_preference(&self) -> PowerPreference {
        match self.power_preference {
            GUIPowerPreference::LowPower => PowerPreference::LowPower,
            GUIPowerPreference::HighPerformance => PowerPreference::HighPerformance,
        }
    }
    /// Returns the wgpu Backend that corrisponds to the
    /// previously selected GUIBackend
    pub fn backend(&self) -> Backends {
        use GUIBackend::*;
        match self.backend {
            Vulkan => Backends::VULKAN,
            Metal => Backends::METAL,
            Dx12 => Backends::DX12,
            Dx11 => Backends::DX11,
            Gl => Backends::GL,
            BrowserWebGpu => Backends::BROWSER_WEBGPU,
        }
    }
}
