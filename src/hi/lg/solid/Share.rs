#[cfg(feature = "HiLgSolidShare")]
use leptos::*;
#[cfg(feature = "HiLgSolidShare")]
///This icon requires the feature `HiLgSolidShare` to be enabled.
#[component]
pub fn Share(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M15.75 4.5C15.75 2.84315 17.0931 1.5 18.75 1.5C20.4069 1.5 21.75 2.84315 21.75 4.5C21.75 6.15685 20.4069 7.5 18.75 7.5C17.8933 7.5 17.1212 7.14074 16.5751 6.56624L8.15392 11.2447C8.21665 11.4863 8.25 11.7395 8.25 12C8.25 12.2605 8.21665 12.5137 8.15392 12.7553L16.5751 17.4338C17.1212 16.8593 17.8933 16.5 18.75 16.5C20.4069 16.5 21.75 17.8431 21.75 19.5C21.75 21.1569 20.4069 22.5 18.75 22.5C17.0931 22.5 15.75 21.1569 15.75 19.5C15.75 19.2395 15.7833 18.9863 15.8461 18.7447L7.42488 14.0662C6.87885 14.6407 6.10667 15 5.25 15C3.59315 15 2.25 13.6569 2.25 12C2.25 10.3431 3.59315 9 5.25 9C6.10667 9 6.87885 9.35926 7.42488 9.93377L15.8461 5.25532C15.7833 5.01367 15.75 4.76045 15.75 4.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
