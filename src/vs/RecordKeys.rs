#[cfg(feature = "VsRecordKeys")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsRecordKeys")]
/// *This icon requires the feature* `VsRecordKeys` *to be enabled*.
#[component]
pub fn RecordKeys(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M14 3H3a1 1 0 0 0-1 1v7a1 1 0 0 0 1 1h11a1 1 0 0 0 1-1V4a1 1 0 0 0-1-1zm0 8H3V4h11v7zm-3-6h-1v1h1V5zm-1 2H9v1h1V7zm2-2h1v1h-1V5zm1 4h-1v1h1V9zM6 9h5v1H6V9zm7-2h-2v1h2V7zM8 5h1v1H8V5zm0 2H7v1h1V7zM4 9h1v1H4V9zm0-4h1v1H4V5zm3 0H6v1h1V5zM4 7h2v1H4V7z" /></svg>
   }
}