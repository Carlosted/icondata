#[cfg(feature = "CgUsb")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgUsb")]
/// *This icon requires the feature* `CgUsb` *to be enabled*.
#[component]
pub fn Usb(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M10 4.5H11V6.5H10V4.5Z" fill="currentColor" /><path d="M14 4.5H13V6.5H14V4.5Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M7 8.5V1.5H17V8.5H19V19.5C19 21.1569 17.6569 22.5 16 22.5H8C6.34315 22.5 5 21.1569 5 19.5V8.5H7ZM9 3.5H15V8.5H9V3.5ZM17 10.5H7V19.5C7 20.0523 7.44772 20.5 8 20.5H16C16.5523 20.5 17 20.0523 17 19.5V10.5Z" fill="currentColor" /></svg>
   }
}