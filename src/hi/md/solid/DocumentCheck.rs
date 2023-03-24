#[cfg(feature = "HiMdSolidDocumentCheck")]
use leptos::*;
#[cfg(feature = "HiMdSolidDocumentCheck")]
///This icon requires the feature `HiMdSolidDocumentCheck` to be enabled.
#[component]
pub fn DocumentCheck(
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
        "M3 3.5C3 2.67157 3.67157 2 4.5 2H11.3787C11.7765 2 12.158 2.15804 12.4393 2.43934L16.5607 6.56066C16.842 6.84197 17 7.2235 17 7.62132V16.5C17 17.3284 16.3284 18 15.5 18H4.5C3.67157 18 3 17.3284 3 16.5V3.5ZM13.8566 9.19113C14.1002 8.85614 14.0261 8.38708 13.6911 8.14345C13.3561 7.89982 12.8871 7.97388 12.6434 8.30887L9.15969 13.099L7.28033 11.2197C6.98744 10.9268 6.51256 10.9268 6.21967 11.2197C5.92678 11.5126 5.92678 11.9874 6.21967 12.2803L8.71967 14.7803C8.87477 14.9354 9.08999 15.0149 9.30867 14.9977C9.52734 14.9805 9.72754 14.8685 9.85655 14.6911L13.8566 9.19113Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
