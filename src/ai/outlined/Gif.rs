#[cfg(feature = "AiOutlinedGif")]
use leptos::*;
#[cfg(feature = "AiOutlinedGif")]
///This icon requires the feature `AiOutlinedGif` to be enabled.
#[component]
pub fn Gif(
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
        stroke_witdh = "0" style = style t = "1569683621237" class = "icon" viewBox =
        "0 0 1024 1024" version = "1.1" p - id = "12475" width = "200" height = "200"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < defs
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" >< style type = "text/css" /></ defs >< path xmlns
        = "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" d =
        "M944 299H692c-4.4 0-8 3.6-8 8v406c0 4.4 3.6 8 8 8h59.2c4.4 0 8-3.6 8-8V549.9h168.2c4.4 0 8-3.6 8-8V495c0-4.4-3.6-8-8-8H759.2V364.2H944c4.4 0 8-3.6 8-8V307c0-4.4-3.6-8-8-8zM588 300h-56c-4.4 0-8 3.6-8 8v406c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8V308c0-4.4-3.6-8-8-8zM452 500.9H290.5c-4.4 0-8 3.6-8 8v43.7c0 4.4 3.6 8 8 8h94.9l-0.3 8.9c-1.2 58.8-45.6 98.5-110.9 98.5-76.2 0-123.9-59.7-123.9-156.7 0-95.8 46.8-155.2 121.5-155.2 54.8 0 93.1 26.9 108.5 75.4h76.2c-13.6-87.2-86-143.4-184.7-143.4C150 288 72 375.2 72 511.9 72 650.2 149.1 736 273 736c114.1 0 187-70.7 187-181.6v-45.5c0-4.4-3.6-8-8-8z"
        p - id = "12476" /> < title > { title } < / title > < / svg >
    }
}
