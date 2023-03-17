#[cfg(feature = "IoGitBranchOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoGitBranchOutline")]
/// *This icon requires the feature* `IoGitBranchOutline` *to be enabled*.
#[component]
pub fn GitBranchOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="160" cy="96" r="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="160" cy="416" r="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="160" y1="368" x2="160" y2="144" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><circle cx="352" cy="160" r="48" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M352,208c0,128-192,48-192,160" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}