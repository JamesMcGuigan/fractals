// DOCS: https://rustwasm.github.io/book/reference/time-profiling.html

extern crate web_sys;

pub fn now() -> f64 {
    web_sys::window()
        .expect("web_sys::window()")
        .performance()
        .expect("web_sys::window().performance()")
        .now()
}