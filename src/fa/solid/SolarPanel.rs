#[cfg(feature = "FaSolidSolarPanel")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidSolarPanel")]
/// *This icon requires the feature* `FaSolidSolarPanel` *to be enabled*.
#[component]
pub fn SolarPanel(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><path d="M115.2 0C84.7 0 58.5 21.5 52.5 51.4L1.3 307.4C-6.6 347 23.6 384 64 384H281v64H217c-17.7 0-32 14.3-32 32s14.3 32 32 32H409c17.7 0 32-14.3 32-32s-14.3-32-32-32H345V384H562c40.4 0 70.7-36.9 62.8-76.6l-51.2-256C567.5 21.5 541.3 0 510.8 0H115.2zM253.9 64H372.1l10.4 104h-139L253.9 64zM195.3 168H94.4L115.2 64h90.4L195.3 168zM84.8 216H190.5L180.1 320H64L84.8 216zm153.9 0H387.3l10.4 104-169.4 0 10.4-104zm196.8 0H541.2L562 320h-116L435.5 216zm96-48H430.7L420.3 64h90.4l31.4-6.3L510.8 64l20.8 104z" /></svg>
   }
}