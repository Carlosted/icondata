#[cfg(feature = "VsAzureDevops")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsAzureDevops")]
/// *This icon requires the feature* `VsAzureDevops` *to be enabled*.
#[component]
pub fn AzureDevops(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M15 3.62172V12.1336L11.5 15L6.075 13.025V14.9825L3.00375 10.9713L11.955 11.6704V4.00624L15 3.62172ZM12.0163 4.04994L6.99375 1V3.00125L2.3825 4.35581L1 6.12984V10.1586L2.9775 11.0325V5.86767L12.0163 4.04994Z" /></svg>
   }
}