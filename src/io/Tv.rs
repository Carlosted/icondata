#[cfg(feature = "IoTv")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoTv")]
/// *This icon requires the feature* `IoTv` *to be enabled*.
#[component]
pub fn Tv(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M447.86,384H64.14A48.2,48.2,0,0,1,16,335.86V128.14A48.2,48.2,0,0,1,64.14,80H447.86A48.2,48.2,0,0,1,496,128.14V335.86A48.2,48.2,0,0,1,447.86,384Z" /><line x1="128" y1="416" x2="384" y2="416" style="stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}