// Source: https://github.com/hgzimmerman/FullstackRustDemo/blob/master/frontend/boids/src/resize_service.rs

use stdweb::{js, Value};
use yew::Callback;

#[allow(dead_code)]
pub struct ResizeServiceJs {}

pub struct ResizeTask(Option<Value>);

impl ResizeServiceJs {

    #[allow(dead_code)]
    pub fn new() -> ResizeServiceJs {
        ResizeServiceJs {}
    }

    #[allow(dead_code)]
    pub fn register(&mut self, callback: Callback<()>) -> ResizeTask {
        let callback = move || {
            callback.emit(());
        };
        let handle = js! {
            var callback = @{callback};
            var action = function() {
                callback();
            };
            return window.addEventListener("resize", action);
        };
        ResizeTask(Some(handle))
    }
}

impl Drop for ResizeTask {
    fn drop(&mut self) {
        let handle = self.0.take().expect("Resize task already empty.");
        js! {
            @(no_return)
            var handle = @{handle};
            handle.callback.drop();
        }
    }
}