#[cfg(feature = "ImConnection")]
use leptos::*;
#[cfg(feature = "ImConnection")]
///This icon requires the feature `ImConnection` to be enabled.
#[component]
pub fn Connection(
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
        stroke_witdh = "0" style = style version = "1.1" width = "20" height = "16"
        viewBox = "0 0 20 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M10 9c1.654 0 3.154 0.673 4.241 1.759l-1.414 1.414c-0.724-0.724-1.724-1.173-2.827-1.173s-2.103 0.449-2.827 1.173l-1.414-1.414c1.086-1.086 2.586-1.759 4.241-1.759zM2.929 7.929c1.889-1.889 4.4-2.929 7.071-2.929s5.182 1.040 7.071 2.929l-1.414 1.414c-1.511-1.511-3.52-2.343-5.657-2.343s-4.146 0.832-5.657 2.343l-1.414-1.414zM15.45 2.101c1.667 0.705 3.164 1.715 4.45 3v0l-1.414 1.414c-2.267-2.266-5.28-3.515-8.485-3.515s-6.219 1.248-8.485 3.515l-1.414-1.414c1.285-1.285 2.783-2.295 4.45-3 1.727-0.73 3.56-1.101 5.45-1.101s3.723 0.37 5.45 1.101zM9 14c0-0.552 0.448-1 1-1s1 0.448 1 1c0 0.552-0.448 1-1 1s-1-0.448-1-1z"
        /> < title > { title } < / title > < / svg >
    }
}
