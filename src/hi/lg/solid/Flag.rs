#[cfg(feature = "HiLgSolidFlag")]
use leptos::*;
#[cfg(feature = "HiLgSolidFlag")]
///This icon requires the feature `HiLgSolidFlag` to be enabled.
#[component]
pub fn Flag(
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
        "M3 2.25C3.41421 2.25 3.75 2.58579 3.75 3V3.53942L5.58819 3.07987C7.84613 2.51539 10.2315 2.77724 12.3132 3.8181L12.421 3.87196C14.1472 4.73507 16.1214 4.96567 18.0001 4.52363L21.1096 3.79196C21.3465 3.73622 21.5958 3.79888 21.7781 3.96005C21.9605 4.12121 22.0533 4.36083 22.0271 4.60278C21.844 6.29313 21.75 8.01046 21.75 9.75C21.75 11.504 21.8455 13.2355 22.0317 14.9395C22.0728 15.3161 21.8266 15.6642 21.4579 15.751L18.3436 16.4837C16.1234 17.0062 13.7902 16.7336 11.7501 15.7136L11.6424 15.6597C9.88097 14.779 7.86256 14.5574 5.95199 15.0351L3.75 15.5856V21C3.75 21.4142 3.41421 21.75 3 21.75C2.58579 21.75 2.25 21.4142 2.25 21V3C2.25 2.58579 2.58579 2.25 3 2.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
