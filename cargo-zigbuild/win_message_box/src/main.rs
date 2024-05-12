// reference: https://blog.foresta.me/posts/rust-cross-build-for-windows-on-wsl/

use windows::{core::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        MessageBoxA(None, s!("Ansi"), s!("World"), MB_OK);
        MessageBoxW(None, w!("Wide"), w!("World"), MB_OK);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        assert_eq!(1, 1);
    }
}
