#[cfg(feature = "SiFeathub")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiFeathub")]
/// *This icon requires the feature* `SiFeathub` *to be enabled*.
#[component]
pub fn Feathub(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M8.571 0v6.857h6.858V0zM0 8.571v6.858h24V8.57zm8.571 8.572V24h6.858v-6.857z" /></svg>
   }
}