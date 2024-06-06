
#[macro_export]
macro_rules! execute_once {
    ($block:expr) => {
        {
            static ONCE: std::sync::Once = std::sync::Once::new();
            ONCE.call_once(|| {
                $block;
            });
        }
    };
}

#[macro_export]
macro_rules! execute_once_ok {
    ($block:expr) => {
        {
            static ONCE: std::sync::Once = std::sync::Once::new();
            static mut OK: bool = false;
            ONCE.call_once(|| {
                unsafe { OK = $block; }
            });
            unsafe { OK }
        }
    };
}