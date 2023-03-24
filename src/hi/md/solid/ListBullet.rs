#[cfg(feature = "HiMdSolidListBullet")]
use leptos::*;
#[cfg(feature = "HiMdSolidListBullet")]
///This icon requires the feature `HiMdSolidListBullet` to be enabled.
#[component]
pub fn ListBullet(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M6 4.75C6 4.33579 6.33579 4 6.75 4H17.25C17.6642 4 18 4.33579 18 4.75C18 5.16421 17.6642 5.5 17.25 5.5H6.75C6.33579 5.5 6 5.16421 6 4.75ZM6 10C6 9.58579 6.33579 9.25 6.75 9.25H17.25C17.6642 9.25 18 9.58579 18 10C18 10.4142 17.6642 10.75 17.25 10.75H6.75C6.33579 10.75 6 10.4142 6 10ZM6 15.25C6 14.8358 6.33579 14.5 6.75 14.5H17.25C17.6642 14.5 18 14.8358 18 15.25C18 15.6642 17.6642 16 17.25 16H6.75C6.33579 16 6 15.6642 6 15.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M1.98999 4.75C1.98999 4.19772 2.43771 3.75 2.98999 3.75H2.99999C3.55228 3.75 3.99999 4.19772 3.99999 4.75V4.76C3.99999 5.31229 3.55228 5.76 2.99999 5.76H2.98999C2.43771 5.76 1.98999 5.31229 1.98999 4.76V4.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M1.98999 15.25C1.98999 14.6977 2.43771 14.25 2.98999 14.25H2.99999C3.55228 14.25 3.99999 14.6977 3.99999 15.25V15.26C3.99999 15.8123 3.55228 16.26 2.99999 16.26H2.98999C2.43771 16.26 1.98999 15.8123 1.98999 15.26V15.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M1.98999 10C1.98999 9.44772 2.43771 9 2.98999 9H2.99999C3.55228 9 3.99999 9.44772 3.99999 10V10.01C3.99999 10.5623 3.55228 11.01 2.99999 11.01H2.98999C2.43771 11.01 1.98999 10.5623 1.98999 10.01V10Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
