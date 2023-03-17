#[cfg(feature = "VsSymbolKeyword")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSymbolKeyword")]
/// *This icon requires the feature* `VsSymbolKeyword` *to be enabled*.
#[component]
pub fn SymbolKeyword(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M15 4h-5V3h5v1zm-1 3h-2v1h2V7zm-4 0H1v1h9V7zm2 6H1v1h11v-1zm-5-3H1v1h6v-1zm8 0h-5v1h5v-1zM8 2v3H1V2h7zM7 3H2v1h5V3z" /></svg>
   }
}