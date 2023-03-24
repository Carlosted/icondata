#[cfg(feature = "IoRose")]
use leptos::*;
#[cfg(feature = "IoRose")]
///This icon requires the feature `IoRose` to be enabled.
#[component]
pub fn Rose(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M429.55,119.49a16,16,0,0,0-17.06-7.1c-18.64,4.19-37.06,9-54.73,14.22C322.7,137,288.43,150.53,249.91,169.2c-18.62,9.05-26,13.35-48,26.13L197.41,198c-32.95,19-57.09,40-73.79,64.29C105.29,288.89,96,320,96,354.64c0,40.74,15.71,77.1,44.24,102.37C169,482.52,209.06,496,256,496c46.76,0,86.89-14.33,116-41.43,28.35-26.35,44-63.39,44-104.29,0-25-6.19-47-12.17-68.22-12.59-44.69-23.46-83.29,24.71-144.13A16,16,0,0,0,429.55,119.49Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M219,119.55C168.46,92.08,101.46,80.69,98.63,80.22A16,16,0,0,0,81,90.55a16.47,16.47,0,0,0,3.79,16.84c31.84,33.78,32.86,68.79,28.65,104.63a4.45,4.45,0,0,0,2.5,4.54h0a4.44,4.44,0,0,0,5.08-.9c16.39-16.51,36.37-31.52,60.4-45.39l4.48-2.6C208,154.8,216.23,150,236,140.41l2.69-1.3a4,4,0,0,0,.64-6.83A178.59,178.59,0,0,0,219,119.55Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M234.26,91.45c3.44,1.87,7.09,4,10.9,6.29a189.31,189.31,0,0,1,29.57,22.39,4,4,0,0,0,4.28.76,672,672,0,0,1,69.65-25q7-2.07,14.08-4a4,4,0,0,0,2.53-5.62C357,69.44,350.6,57.37,350.12,56.48A16,16,0,0,0,336,48c-1.91,0-33.28.36-76.87,21.3a279,279,0,0,0-26.39,14.51,4,4,0,0,0,.22,6.94Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M209.33,60.79c7.3-4.77,14.74-9.22,22.25-13.31a2,2,0,0,0,.24-3.36c-26-19.57-49.73-27-51.15-27.42a16,16,0,0,0-17.56,5.82A217.63,217.63,0,0,0,143.83,54.9a2,2,0,0,0,1.29,2.81C158.73,61.28,174.52,66,190.73,72a2,2,0,0,0,1.79-.2Z"
        /> < title > { title } < / title > < / svg >
    }
}
