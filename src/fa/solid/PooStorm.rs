#[cfg(feature = "FaSolidPooStorm")]
use leptos::*;
#[cfg(feature = "FaSolidPooStorm")]
///This icon requires the feature `FaSolidPooStorm` to be enabled.
#[component]
pub fn PooStorm(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M236.9 .2c-5.5-.7-11 1.4-14.5 5.7s-4.6 10.1-2.8 15.3c2.8 8.2 4.3 16.9 4.3 26.1c0 21.7-8.5 37.2-21.9 47.6c-13.8 10.8-34 17-57.8 17H128c-35.3 0-64 28.7-64 64c0 12.2 3.4 23.5 9.3 33.2C31.7 216.2 0 252.4 0 296c0 41 28 75.4 65.8 85.2c-5.3-18.5 1-38.5 16.2-50.7l160-128c17.6-14.1 42.6-14 60.2 .2s22.8 38.6 12.8 58.8L285.7 320H304c20.4 0 38.5 12.9 45.3 32.1c3.7 10.6 3.5 21.8 0 31.9H360c48.6 0 88-39.4 88-88c0-43.6-31.7-79.8-73.3-86.8c5.9-9.7 9.3-21.1 9.3-33.2c0-35.3-28.7-64-64-64h-1.4c.9-5.4 1.4-10.9 1.4-16.6c0-48.7-36.1-88.9-83.1-95.2zm45.1 227.4c-5.8-4.7-14.2-4.7-20.1-.1l-160 128c-5.3 4.2-7.4 11.4-5.1 17.8s8.3 10.7 15.1 10.7h70.1L129.7 488.8c-3.4 6.7-1.6 14.9 4.3 19.6s14.2 4.7 20.1 .1l160-128c5.3-4.2 7.4-11.4 5.1-17.8s-8.3-10.7-15.1-10.7H233.9l52.4-104.8c3.4-6.7 1.6-14.9-4.2-19.6z"
        /> < title > { title } < / title > < / svg >
    }
}
