#[cfg(feature = "HiMdSolidFolderPlus")]
use leptos::*;
#[cfg(feature = "HiMdSolidFolderPlus")]
///This icon requires the feature `HiMdSolidFolderPlus` to be enabled.
#[component]
pub fn FolderPlus(
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
        "M3.75 3C2.7835 3 2 3.7835 2 4.75V15.25C2 16.2165 2.7835 17 3.75 17H16.25C17.2165 17 18 16.2165 18 15.25V6.75C18 5.7835 17.2165 5 16.25 5H11.4142C11.3479 5 11.2843 4.97366 11.2374 4.92678L9.82322 3.51256C9.49503 3.18437 9.04992 3 8.58579 3H3.75ZM10 8C10.4142 8 10.75 8.33579 10.75 8.75V10.25H12.25C12.6642 10.25 13 10.5858 13 11C13 11.4142 12.6642 11.75 12.25 11.75H10.75V13.25C10.75 13.6642 10.4142 14 10 14C9.58579 14 9.25 13.6642 9.25 13.25V11.75H7.75C7.33579 11.75 7 11.4142 7 11C7 10.5858 7.33579 10.25 7.75 10.25H9.25V8.75C9.25 8.33579 9.58579 8 10 8Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
