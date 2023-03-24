#[cfg(feature = "HiLgSolidBarsArrowUp")]
use leptos::*;
#[cfg(feature = "HiLgSolidBarsArrowUp")]
///This icon requires the feature `HiLgSolidBarsArrowUp` to be enabled.
#[component]
pub fn BarsArrowUp(
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
        "M2.25 4.5C2.25 4.08579 2.58579 3.75 3 3.75H17.25C17.6642 3.75 18 4.08579 18 4.5C18 4.91421 17.6642 5.25 17.25 5.25H3C2.58579 5.25 2.25 4.91421 2.25 4.5ZM16.7197 8.46967C17.0126 8.17678 17.4874 8.17678 17.7803 8.46967L21.5303 12.2197C21.8232 12.5126 21.8232 12.9874 21.5303 13.2803C21.2374 13.5732 20.7626 13.5732 20.4697 13.2803L18 10.8107L18 21C18 21.4142 17.6642 21.75 17.25 21.75C16.8358 21.75 16.5 21.4142 16.5 21L16.5 10.8107L14.0303 13.2803C13.7374 13.5732 13.2626 13.5732 12.9697 13.2803C12.6768 12.9874 12.6768 12.5126 12.9697 12.2197L16.7197 8.46967ZM2.25 9C2.25 8.58579 2.58579 8.25 3 8.25H12.75C13.1642 8.25 13.5 8.58579 13.5 9C13.5 9.41421 13.1642 9.75 12.75 9.75H3C2.58579 9.75 2.25 9.41421 2.25 9ZM2.25 13.5C2.25 13.0858 2.58579 12.75 3 12.75H8.25C8.66421 12.75 9 13.0858 9 13.5C9 13.9142 8.66421 14.25 8.25 14.25H3C2.58579 14.25 2.25 13.9142 2.25 13.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
