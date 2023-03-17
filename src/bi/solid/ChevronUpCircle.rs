#[cfg(feature = "BiSolidChevronUpCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidChevronUpCircle")]
/// *This icon requires the feature* `BiSolidChevronUpCircle` *to be enabled*.
#[component]
pub fn ChevronUpCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm4.293 12.707L12 10.414l-4.293 4.293-1.414-1.414L12 7.586l5.707 5.707-1.414 1.414z" /></svg>
   }
}