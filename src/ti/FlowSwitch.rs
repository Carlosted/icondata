#[cfg(feature = "TiFlowSwitch")]
use leptos::*;
#[cfg(feature = "TiFlowSwitch")]
///This icon requires the feature `TiFlowSwitch` to be enabled.
#[component]
pub fn FlowSwitch(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M8 16.184v-.684c0-.848.512-1.595 1.287-2.047-.667-.282-1.279-.667-1.822-1.131-.904.814-1.465 1.938-1.465 3.178v.684c-1.161.415-2 1.514-2 2.816 0 1.654 1.346 3 3 3s3-1.346 3-3c0-1.302-.839-2.401-2-2.816zm-1 3.816c-.552 0-1-.449-1-1s.448-1 1-1 1 .449 1 1-.448 1-1 1zM16 7.815v.351c0 .985-.535 1.852-1.345 2.36.665.274 1.279.646 1.823 1.1.936-.878 1.522-2.102 1.522-3.459v-.351c1.161-.415 2-1.514 2-2.816 0-1.654-1.346-3-3-3s-3 1.346-3 3c0 1.302.839 2.401 2 2.815zm1-3.815c.552 0 1 .449 1 1s-.448 1-1 1-1-.449-1-1 .448-1 1-1zM17.935 16.164c-.41-2.913-2.911-5.164-5.935-5.164-1.936 0-3.552-1.381-3.92-3.209 1.12-.436 1.92-1.519 1.92-2.791 0-1.654-1.346-3-3-3s-3 1.346-3 3c0 1.326.87 2.44 2.065 2.836.41 2.913 2.911 5.164 5.935 5.164 1.936 0 3.552 1.381 3.92 3.209-1.12.436-1.92 1.519-1.92 2.791 0 1.654 1.346 3 3 3s3-1.346 3-3c0-1.326-.87-2.44-2.065-2.836zm-10.935-12.164c.552 0 1 .449 1 1s-.448 1-1 1-1-.449-1-1 .448-1 1-1zm10 16c-.552 0-1-.449-1-1s.448-1 1-1 1 .449 1 1-.448 1-1 1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
