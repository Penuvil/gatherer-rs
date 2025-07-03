use windows_sys::{
    Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*, core::*,
};

fn main() {
    unsafe {
        let instance = GetModuleHandleA(std::ptr::null());
        if instance.is_null() {
            panic!("Failed to get module handle");
        }

        let window_class = s!("window");

        let mut wc = WNDCLASSA::default();
        wc.lpfnWndProc = Some(wndproc);
        wc.hInstance = instance;
        wc.lpszClassName = window_class;

        let atom = RegisterClassA(&wc);
        if atom == 0 {
            panic!("Failed to register  class");
        }

        let hwnd = CreateWindowExA(
            0,
            window_class,
            s!("The Gatherer"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            instance,
            std::ptr::null(),
        );

        if hwnd.is_null() {
            panic!("Failed to create window")
        }

        let mut message = std::mem::zeroed();

        while GetMessageA(&mut message, std::ptr::null_mut(), 0, 0) != 0 {
            DispatchMessageA(&message);
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_DESTROY => {
                PostQuitMessage(0);
                return 0;
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
