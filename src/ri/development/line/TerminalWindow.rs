#[cfg(feature = "RiDevelopmentLineTerminalWindow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDevelopmentLineTerminalWindow")]
/// *This icon requires the feature* `RiDevelopmentLineTerminalWindow` *to be enabled*.
#[component]
pub fn TerminalWindow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M20 9V5H4v4h16zm0 2H4v8h16v-8zM3 3h18a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm2 9h3v5H5v-5zm0-6h2v2H5V6zm4 0h2v2H9V6z" /></g></svg>
   }
}