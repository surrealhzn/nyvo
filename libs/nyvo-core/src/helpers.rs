#[macro_export]
// This code is safe, the value gets checked before being used
macro_rules! unwrap_return_ref {
    ($var:expr) => {
        if $var.is_some() {
            unsafe {
                return Ok($var.as_ref().unwrap_unchecked());
            }
        }
    };
}
