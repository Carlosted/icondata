#[cfg(feature = "IoImageSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoImageSharp")]
/// *This icon requires the feature* `IoImageSharp` *to be enabled*.
#[component]
pub fn ImageSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M456,64H56A24,24,0,0,0,32,88V424a24,24,0,0,0,24,24H456a24,24,0,0,0,24-24V88A24,24,0,0,0,456,64ZM331.62,128.2a48,48,0,1,1-43.42,43.42A48,48,0,0,1,331.62,128.2ZM76,416a12,12,0,0,1-12-12V316.37L192.64,202l96.95,96.75L172.37,416Zm372-12a12,12,0,0,1-12,12H217.63L367.16,266.47,448,333.84Z" /></svg>
   }
}