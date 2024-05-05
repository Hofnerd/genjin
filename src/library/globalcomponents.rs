pub struct ScreenInfo {
    pub screen_size: ScreenSize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScreenSize {
    Size { width: u32, height: u32 },
}
