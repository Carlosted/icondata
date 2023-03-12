#[cfg(feature = "VsLayoutPanelLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsLayoutPanelLeft")]
/// *This icon requires the feature* `VsLayoutPanelLeft` *to be enabled*.
#[component]
pub fn LayoutPanelLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M1 2L2 1H14L15 2V14L14 15H2L1 14V2ZM2 2V10H10V2H2ZM11 2V14H14V2H11Z" /></svg>
   }
}