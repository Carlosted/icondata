#[cfg(feature = "BiRegularExit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularExit")]
/// *This icon requires the feature* `BiRegularExit` *to be enabled*.
#[component]
pub fn Exit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19.002 3h-14c-1.103 0-2 .897-2 2v4h2V5h14v14h-14v-4h-2v4c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V5c0-1.103-.898-2-2-2z" /><path d="m11 16 5-4-5-4v3.001H3v2h8z" /></svg>
   }
}