#[cfg(feature = "VsArrowLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsArrowLeft")]
/// *This icon requires the feature* `VsArrowLeft` *to be enabled*.
#[component]
pub fn ArrowLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M7 3.093l-5 5V8.8l5 5 .707-.707-4.146-4.147H14v-1H3.56L7.708 3.8 7 3.093z" /></svg>
   }
}