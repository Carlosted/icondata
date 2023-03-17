#[cfg(feature = "BiRegularReflectHorizontal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularReflectHorizontal")]
/// *This icon requires the feature* `BiRegularReflectHorizontal` *to be enabled*.
#[component]
pub fn ReflectHorizontal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 21h12l-6-6-6 6zM18 3H6l6 6 6-6zM3 11h3v2H3zm5 0h3v2H8zm5 0h3v2h-3zm5 0h3v2h-3z" /></svg>
   }
}