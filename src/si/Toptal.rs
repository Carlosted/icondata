#[cfg(feature = "SiToptal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiToptal")]
/// *This icon requires the feature* `SiToptal` *to be enabled*.
#[component]
pub fn Toptal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M20.227 10.038L10.188 0l-2.04 2.04 3.773 3.769-8.155 8.153L13.807 24l2.039-2.039-3.772-3.771 8.16-8.152h-.007zM8.301 14.269l6.066-6.063 1.223 1.223-6.064 6.113-1.223-1.26-.002-.013z" /></svg>
   }
}