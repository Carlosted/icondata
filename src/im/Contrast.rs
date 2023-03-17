#[cfg(feature = "ImContrast")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImContrast")]
/// *This icon requires the feature* `ImContrast` *to be enabled*.
#[component]
pub fn Contrast(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M8 0c-4.418 0-8 3.582-8 8s3.582 8 8 8 8-3.582 8-8-3.582-8-8-8zM2 8c0-3.314 2.686-6 6-6v12c-3.314 0-6-2.686-6-6z" /></svg>
   }
}