#[cfg(feature = "BiRegularSidebar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSidebar")]
/// *This icon requires the feature* `BiRegularSidebar` *to be enabled*.
#[component]
pub fn Sidebar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 3H4c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2zM4 19V7h6v12H4zm8 0V7h8V5l.002 14H12z" /><path d="M6 10h2v2H6zm0 4h2v2H6z" /></svg>
   }
}