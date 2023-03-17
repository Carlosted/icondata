#[cfg(feature = "SiModx")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiModx")]
/// *This icon requires the feature* `SiModx` *to be enabled*.
#[component]
pub fn Modx(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M19.145 9.49l4.47-7.19H12.118l-1.24 2.023zM2.92 0v11.497l2.48 1.55 13.435-3.1zm18.16 24V12.503l-1.984-1.263-5.168 8.267zM5.165 14.053l-4.78 7.648h11.497L18.6 10.953Z" /></svg>
   }
}