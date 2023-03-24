#[cfg(feature = "TiThSmall")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiThSmall")]
/// *This icon requires the feature* `TiThSmall` *to be enabled*.
#[component]
pub fn ThSmall(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><circle cx="5" cy="19" r="2.5" /><circle cx="5" cy="12" r="2.5" /><circle cx="5" cy="5" r="2.5" /><circle cx="12" cy="19" r="2.5" /><circle cx="12" cy="12" r="2.5" /><circle cx="12" cy="5" r="2.5" /><circle cx="19" cy="19" r="2.5" /><circle cx="19" cy="12" r="2.5" /><circle cx="19" cy="5" r="2.5" /></svg>
   }
}