#[cfg(feature = "FaSolidNotEqual")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidNotEqual")]
/// *This icon requires the feature* `FaSolidNotEqual` *to be enabled*.
#[component]
pub fn NotEqual(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M353.8 37.4c14.7 9.8 18.7 29.7 8.9 44.4L321.1 144H384c17.7 0 32 14.3 32 32s-14.3 32-32 32H278.5l-64 96H384c17.7 0 32 14.3 32 32s-14.3 32-32 32H171.8l-65.2 97.7c-9.8 14.7-29.7 18.7-44.4 8.9s-18.7-29.7-8.9-44.4L94.9 368H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H137.5l64-96H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H244.2l65.2-97.7c9.8-14.7 29.7-18.7 44.4-8.9z" /></svg>
   }
}