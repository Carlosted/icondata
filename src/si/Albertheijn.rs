#[cfg(feature = "SiAlbertheijn")]
use leptos::*;
#[cfg(feature = "SiAlbertheijn")]
///This icon requires the feature `SiAlbertheijn` to be enabled.
#[component]
pub fn Albertheijn(
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
        "M15.652 0c-.354.002-.714.09-1.047.276L4.475 5.924c-.456.26-.881.716-1.075 1.362l-2.718 9.44c-.326 1.13.311 2.314 1.423 2.646l15.968 4.542c1.111.33 2.277-.318 2.603-1.448l2.641-9.258c.172-.543.076-1.213-.192-1.737L17.59 1.162A2.19 2.19 0 0 0 15.652 0zM12.97 6.373s-.008 4.23 0 4.225c.899-1.295 1.712-2.577 3.234-2.577 1.684-.001 2.597 1.409 2.602 2.595l-.007 7.596h-1.891l-.009-7.19c0-.988-.793-.986-.804-.986-.64 0-1.816 1.605-3.125 3.386v4.793l-1.913.002-.002-2.219S9.79 18.217 7.897 18.22c-2.148 0-2.877-1.476-2.882-5.015-.004-3.37.474-5.175 2.777-5.177 1.751-.001 3.256 2.55 3.256 2.55V8.963zm-5.15 3.658c-.88 0-.957.93-.954 3.17.003 2.242.124 3.115.95 3.115 1.124-.001 2.895-2.86 2.895-2.86S8.955 10.03 7.82 10.03z"
        /> < title > { title } < / title > < / svg >
    }
}
