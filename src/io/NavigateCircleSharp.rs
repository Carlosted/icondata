#[cfg(feature = "IoNavigateCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoNavigateCircleSharp")]
/// *This icon requires the feature* `IoNavigateCircleSharp` *to be enabled*.
#[component]
pub fn NavigateCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48h0A208.23,208.23,0,0,0,48,256c0,114.68,93.31,208,208,208h0A208.23,208.23,0,0,0,464,256C464,141.31,370.69,48,256,48Zm-8,361V264H104l-1,0,259-114.11Z" /></svg>
   }
}