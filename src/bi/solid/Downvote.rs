#[cfg(feature = "BiSolidDownvote")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDownvote")]
/// *This icon requires the feature* `BiSolidDownvote` *to be enabled*.
#[component]
pub fn Downvote(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20.901 10.566A1.001 1.001 0 0 0 20 10h-4V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v7H4a1.001 1.001 0 0 0-.781 1.625l8 10a1 1 0 0 0 1.562 0l8-10c.24-.301.286-.712.12-1.059z" /></svg>
   }
}