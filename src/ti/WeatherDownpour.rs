#[cfg(feature = "TiWeatherDownpour")]
use leptos::*;
#[cfg(feature = "TiWeatherDownpour")]
///This icon requires the feature `TiWeatherDownpour` to be enabled.
#[component]
pub fn WeatherDownpour(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M15 22c-.552 0-1-.447-1-1v-6c0-.553.448-1 1-1s1 .447 1 1v6c0 .553-.448 1-1 1zM9 22c-.552 0-1-.447-1-1v-6c0-.553.448-1 1-1s1 .447 1 1v6c0 .553-.448 1-1 1zM12 24c-.552 0-1-.447-1-1v-6c0-.553.448-1 1-1s1 .447 1 1v6c0 .553-.448 1-1 1zM6 18c-2.206 0-4-1.794-4-4 0-1.861 1.277-3.429 3.001-3.874l-.001-.126c0-3.309 2.691-6 6-6 2.587 0 4.824 1.639 5.65 4.015 2.936-.244 5.35 2.113 5.35 4.985 0 2.241-1.507 4.223-3.666 4.819-.535.146-1.083-.166-1.23-.697-.147-.532.165-1.083.698-1.23 1.294-.358 2.198-1.547 2.198-2.892 0-1.654-1.346-3-3-3-.242 0-.499.041-.811.13l-1.074.306-.185-1.102c-.326-1.932-1.978-3.334-3.93-3.334-2.206 0-4 1.794-4 4 0 .272.027.545.082.808l.248 1.202-1.422-.016c-1.011.006-1.908.903-1.908 2.006s.897 2 2 2c.552 0 1 .447 1 1s-.448 1-1 1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
