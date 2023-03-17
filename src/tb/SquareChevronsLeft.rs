#[cfg(feature = "TbSquareChevronsLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSquareChevronsLeft")]
/// *This icon requires the feature* `TbSquareChevronsLeft` *to be enabled*.
#[component]
pub fn SquareChevronsLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-square-chevrons-left" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M15 15l-3 -3l3 -3" /><path d="M11 15l-3 -3l3 -3" /><path d="M3 3m0 2a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2z" /></svg>
   }
}