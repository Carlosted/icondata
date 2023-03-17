#[cfg(feature = "BiRegularShower")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularShower")]
/// *This icon requires the feature* `BiRegularShower` *to be enabled*.
#[component]
pub fn Shower(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M21 18.33A6.78 6.78 0 0 0 19.5 15a6.73 6.73 0 0 0-1.5 3.33 1.51 1.51 0 1 0 3 0zM11 20.33A6.78 6.78 0 0 0 9.5 17 6.73 6.73 0 0 0 8 20.33 1.59 1.59 0 0 0 9.5 22a1.59 1.59 0 0 0 1.5-1.67zM14.5 22a1.59 1.59 0 0 0 1.5-1.67A6.78 6.78 0 0 0 14.5 17a6.73 6.73 0 0 0-1.5 3.33A1.59 1.59 0 0 0 14.5 22zM3 18.33A1.59 1.59 0 0 0 4.5 20 1.59 1.59 0 0 0 6 18.33 6.78 6.78 0 0 0 4.5 15 6.73 6.73 0 0 0 3 18.33zM13 4.07V2h-2v2.07A8 8 0 0 0 4 12H2v2h20v-2h-2a8 8 0 0 0-7-7.93zM6 12a6 6 0 0 1 12 0z" /></svg>
   }
}