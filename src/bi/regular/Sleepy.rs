#[cfg(feature = "BiRegularSleepy")]
use leptos::*;
#[cfg(feature = "BiRegularSleepy")]
///This icon requires the feature `BiRegularSleepy` to be enabled.
#[component]
pub fn Sleepy(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 18c-4.411 0-8-3.589-8-8s3.589-8 8-8 8 3.589 8 8-3.589 8-8 8z"
        />< ellipse xmlns = "http://www.w3.org/2000/svg" cx = "12" cy = "15.5" rx = "3"
        ry = "2.5" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 7c-2.905 0-3.983 2.386-4 3.99l2 .021C8.002 10.804 8.076 9 10 9V7zm4 0v2c1.826 0 1.992 1.537 2 2.007L17 11h1c0-1.608-1.065-4-4-4z"
        /> < title > { title } < / title > < / svg >
    }
}
