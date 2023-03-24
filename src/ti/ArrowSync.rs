#[cfg(feature = "TiArrowSync")]
use leptos::*;
#[cfg(feature = "TiArrowSync")]
///This icon requires the feature `TiArrowSync` to be enabled.
#[component]
pub fn ArrowSync(
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
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.5 12.473c0-1.948-.618-3.397-2.066-4.844-.391-.39-1.023-.39-1.414 0-.391.391-.391 1.024 0 1.415 1.079 1.078 1.48 2.007 1.48 3.429 0 1.469-.572 2.85-1.611 3.888-1.004 1.003-2.078 1.502-3.428 1.593l1.246-1.247c.391-.391.391-1.023 0-1.414s-1.023-.391-1.414 0l-3.707 3.707 3.707 3.707c.195.195.451.293.707.293s.512-.098.707-.293c.391-.391.391-1.023 0-1.414l-1.337-1.336c1.923-.082 3.542-.792 4.933-2.181 1.417-1.416 2.197-3.299 2.197-5.303zM6.5 12.5c0-1.469.572-2.85 1.611-3.889 1.009-1.009 2.092-1.508 3.457-1.594l-1.275 1.275c-.391.391-.391 1.023 0 1.414.195.196.451.294.707.294s.512-.098.707-.293l3.707-3.707-3.707-3.707c-.391-.391-1.023-.391-1.414 0s-.391 1.023 0 1.414l1.311 1.311c-1.914.086-3.525.796-4.907 2.179-1.417 1.416-2.197 3.299-2.197 5.303 0 1.948.618 3.397 2.066 4.844.195.195.451.292.707.292s.512-.098.707-.293c.391-.391.391-1.024 0-1.415-1.079-1.077-1.48-2.006-1.48-3.428z"
        /> < title > { title } < / title > < / svg >
    }
}
