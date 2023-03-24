#[cfg(feature = "HiMdSolidHandRaised")]
use leptos::*;
#[cfg(feature = "HiMdSolidHandRaised")]
///This icon requires the feature `HiMdSolidHandRaised` to be enabled.
#[component]
pub fn HandRaised(
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
        "M11 2C11 1.44772 10.5523 1 10 1C9.44772 1 9 1.44772 9 2V8V8.5C9 8.77614 8.77614 9 8.5 9C8.22386 9 8 8.77614 8 8.5V8V3C8 2.44772 7.55229 2 7 2C6.44771 2 6 2.44772 6 3V8V8.5C6 8.77614 5.77614 9 5.5 9C5.22386 9 5 8.77614 5 8.5V8V5C5 4.44772 4.55229 4 4 4C3.44771 4 3 4.44772 3 5V11V12C3 15.866 6.13401 19 10 19C13.866 19 17 15.866 17 12V11V8C17 7.44772 16.5523 7 16 7C15.4477 7 15 7.44772 15 8V11V11.5C15 11.7761 14.7761 12 14.5 12C14.2239 12 14 11.7761 14 11.5V11V10V8V3C14 2.44772 13.5523 2 13 2C12.4477 2 12 2.44772 12 3V8V8.5C12 8.77614 11.7761 9 11.5 9C11.2239 9 11 8.77614 11 8.5V8V2Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
