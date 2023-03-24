#[cfg(feature = "CgGitCommit")]
use leptos::*;
#[cfg(feature = "CgGitCommit")]
///This icon requires the feature `CgGitCommit` to be enabled.
#[component]
pub fn GitCommit(
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
        "M11 4C11 3.44772 11.4477 3 12 3C12.5523 3 13 3.44772 13 4V9.17071C14.1652 9.58254 15 10.6938 15 12C15 13.3062 14.1652 14.4175 13 14.8293V20C13 20.5523 12.5523 21 12 21C11.4477 21 11 20.5523 11 20V14.8293C9.83481 14.4175 9 13.3062 9 12C9 10.6938 9.83481 9.58254 11 9.17071V4ZM12 13C12.5523 13 13 12.5523 13 12C13 11.4477 12.5523 11 12 11C11.4477 11 11 11.4477 11 12C11 12.5523 11.4477 13 12 13Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
