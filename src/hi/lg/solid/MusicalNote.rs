#[cfg(feature = "HiLgSolidMusicalNote")]
use leptos::*;
#[cfg(feature = "HiLgSolidMusicalNote")]
///This icon requires the feature `HiLgSolidMusicalNote` to be enabled.
#[component]
pub fn MusicalNote(
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
        "M19.9516 1.65124C20.1395 1.79297 20.25 2.01466 20.25 2.25001V5.98344C20.2503 5.99476 20.2503 6.00606 20.25 6.01732V16.3028C20.25 17.6423 19.3621 18.8194 18.0742 19.1874L16.7542 19.5645C15.1234 20.0305 13.5 18.806 13.5 17.1099C13.5 15.9701 14.2556 14.9684 15.3515 14.6553L17.6621 13.9951C18.306 13.8111 18.75 13.2225 18.75 12.5528V6.9943L9.75 9.56573V19.3028C9.75 20.6423 8.86207 21.8194 7.57416 22.1874L6.25418 22.5645C4.62337 23.0305 3 21.806 3 20.1099C3 18.9701 3.75559 17.9684 4.85153 17.6553L7.16208 16.9951C7.80603 16.8111 8.25 16.2225 8.25 15.5528V9.01659C8.24974 9.00526 8.24974 8.99395 8.25 8.98268V5.25001C8.25 4.91515 8.47198 4.62086 8.79396 4.52886L19.294 1.52886C19.5202 1.46421 19.7638 1.50952 19.9516 1.65124Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
