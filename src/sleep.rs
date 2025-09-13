use crate::js;
use crate::time::Instant;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::time::Duration;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

#[derive(Debug)]
struct SleepInner {
    future: JsFuture,
    timeout_id: i32,
    resolved: Rc<RefCell<bool>>,
    _closure: Closure<dyn FnMut() -> Result<(), JsValue>>,
}

#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Debug)]
pub struct Sleep {
    deadline: Instant,
    inner: Option<SleepInner>,
}

impl Sleep {
    pub(crate) fn new(deadline: Instant) -> Self {
        Self {
            deadline,
            inner: None,
        }
    }

    pub fn deadline(&self) -> Instant {
        self.deadline
    }

    fn cancel(&mut self) {
        if let Some(inner) = self.inner.take() {
            // Call clearTimeout() if not resolved yet, because the Closure
            // will be destroyed (and to avoid keeping browser timers alive)
            if !*inner.resolved.borrow() {
                let global = js_sys::global();
                let global_scope = global.unchecked_ref::<js::GlobalScope>();
                global_scope
                    .clear_timeout_with_handle(inner.timeout_id)
                    .unwrap();
            }
        }
    }

    pub fn reset(self: Pin<&mut Self>, deadline: Instant) {
        let this = self.get_mut();
        this.cancel();
        this.deadline = deadline;
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.inner.is_none() {
            let now = Instant::now();
            if now >= this.deadline {
                // Already ready
                return Poll::Ready(());
            }

            assert!(now < this.deadline);
            let duration_ms: i32 = (this.deadline - now)
                .as_millis()
                .try_into()
                .unwrap_or(i32::MAX);

            // Flag set when the promise is resolved
            let resolved = Rc::new(RefCell::new(false));
            let resolved2 = resolved.clone();

            // Resolve function, set by the Promise::new() callback
            let resolve = Rc::new(RefCell::new(None::<js_sys::Function>));
            let resolve2 = resolve.clone();

            let promise = js_sys::Promise::new(&mut |resolve, _| {
                *resolve2.borrow_mut() = Some(resolve);
            });

            // Keep the closure alive until necessary. An alternative would be
            // to call Closure::once_into_js() and to never clearTimeout() on
            // drop, but this would keep timers alive in the browser more than
            // necessary if the corresponding Future is dropped.
            let closure: Closure<dyn FnMut() -> Result<(), JsValue>> =
                Closure::wrap(Box::new(move || {
                    let resolve = resolve2.borrow_mut().take().unwrap();
                    let res = resolve.call0(&JsValue::NULL);
                    // Set the resolved flag AFTER the resolve function is called
                    *resolved2.borrow_mut() = true;
                    res?;
                    Ok(())
                }));
            let timeout_cb = closure.as_ref().unchecked_ref();

            // call in javacript: let timeout_id = setTimeout(timeout_cb, duration_ms)
            let global = js_sys::global();
            let global_scope = global.unchecked_ref::<js::GlobalScope>();
            let timeout_id = global_scope
                .set_timeout_with_callback_and_timeout_and_arguments_0(&timeout_cb, duration_ms)
                .unwrap();

            this.inner = Some(SleepInner {
                future: JsFuture::from(promise),
                timeout_id,
                resolved,
                _closure: closure, // keep alive
            });
        }

        let fut = &mut this.inner.as_mut().unwrap().future;
        let fut = Pin::new(fut);
        match fut.poll(cx) {
            Poll::Ready(result) => {
                result.unwrap();
                Poll::Ready(())
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Drop for Sleep {
    fn drop(&mut self) {
        self.cancel();
    }
}

pub fn sleep_until(deadline: Instant) -> Sleep {
    Sleep::new(deadline)
}

pub fn sleep(duration: Duration) -> Sleep {
    sleep_until(Instant::now() + duration)
}
