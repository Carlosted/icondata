#[cfg(feature = "HiLgSolidBarsArrowDown")]
use leptos::*;
#[cfg(feature = "HiLgSolidBarsArrowDown")]
///This icon requires the feature `HiLgSolidBarsArrowDown` to be enabled.
#[component]
pub fn BarsArrowDown(
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
        "M2.25 4.5C2.25 4.08579 2.58579 3.75 3 3.75H17.25C17.6642 3.75 18 4.08579 18 4.5C18 4.91421 17.6642 5.25 17.25 5.25H3C2.58579 5.25 2.25 4.91421 2.25 4.5ZM2.25 9C2.25 8.58579 2.58579 8.25 3 8.25H12.75C13.1642 8.25 13.5 8.58579 13.5 9C13.5 9.41421 13.1642 9.75 12.75 9.75H3C2.58579 9.75 2.25 9.41421 2.25 9ZM17.25 8.25C17.6642 8.25 18 8.58579 18 9V19.1893L20.4697 16.7197C20.7626 16.4268 21.2374 16.4268 21.5303 16.7197C21.8232 17.0126 21.8232 17.4874 21.5303 17.7803L17.7803 21.5303C17.4874 21.8232 17.0126 21.8232 16.7197 21.5303L12.9697 17.7803C12.6768 17.4874 12.6768 17.0126 12.9697 16.7197C13.2626 16.4268 13.7374 16.4268 14.0303 16.7197L16.5 19.1893V9C16.5 8.58579 16.8358 8.25 17.25 8.25ZM2.25 13.5C2.25 13.0858 2.58579 12.75 3 12.75H12.75C13.1642 12.75 13.5 13.0858 13.5 13.5C13.5 13.9142 13.1642 14.25 12.75 14.25H3C2.58579 14.25 2.25 13.9142 2.25 13.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
