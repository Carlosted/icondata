#[cfg(feature = "SiGooglestreetview")]
use leptos::*;
#[cfg(feature = "SiGooglestreetview")]
///This icon requires the feature `SiGooglestreetview` to be enabled.
#[component]
pub fn Googlestreetview(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12.571 5.714a5.714 5.714 0 1 1 11.43 0 5.714 5.714 0 0 1-11.43 0zm2.264 5.165l-3.502 3.502c2.015-1.488 4.48-2.31 6.953-2.31 1.155 0 2.307.182 3.428.53v-1.709a6.176 6.176 0 0 1-3.428 1.037 6.177 6.177 0 0 1-3.45-1.05zm6.88 11.407V13.12a11.074 11.074 0 0 0-3.43-.55 11.25 11.25 0 0 0-6.731 2.265c-.425.34-.697.863-.697 1.45V24H20a1.72 1.72 0 0 0 1.714-1.714zM13.12 9.165L.001 22.285V4a1.72 1.72 0 0 1 1.713-1.714h11.394a6.176 6.176 0 0 0-1.037 3.428c0 1.276.388 2.463 1.05 3.45zm-5.246-1.95a2.7 2.7 0 0 0-.077-.644h-2.94v1.142h1.69c.001.303-.228.755-.625 1.025-.258.176-.606.298-1.066.298-.818 0-1.512-.552-1.76-1.295a1.887 1.887 0 0 1 0-1.196c.248-.743.942-1.295 1.76-1.295.6 0 .987.268 1.19.458l.913-.889A3.018 3.018 0 0 0 4.857 4a3.143 3.143 0 1 0 0 6.287c.848 0 1.563-.279 2.083-.759.593-.547.935-1.356.935-2.313zm2.482 9.07c0-.511.17-.995.471-1.399L1.714 24h8.643v-7.714z"
        /> < title > { title } < / title > < / svg >
    }
}
