#[cfg(feature = "FiType")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiType")]
/// *This icon requires the feature* `FiType` *to be enabled*.
#[component]
pub fn Type(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 7 4 4 20 4 20 7" /><line x1="9" y1="20" x2="15" y2="20" /><line x1="12" y1="4" x2="12" y2="20" /></svg>
   }
}