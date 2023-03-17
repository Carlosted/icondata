#[cfg(feature = "BiSolidFastForwardCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFastForwardCircle")]
/// *This icon requires the feature* `BiSolidFastForwardCircle` *to be enabled*.
#[component]
pub fn FastForwardCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.485 2 2 6.485 2 12s4.485 10 10 10c5.514 0 10-4.485 10-10S17.514 2 12 2zm1 14v-4l-6 4V8l6 4V8l6 4-6 4z" /></svg>
   }
}