#[cfg(feature = "TiBrush")]
use leptos::*;
#[cfg(feature = "TiBrush")]
///This icon requires the feature `TiBrush` to be enabled.
#[component]
pub fn Brush(
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
        "M20.177 3.823c-.382-.383-.894-.586-1.415-.586-.25 0-.503.047-.744.144-4.449 1.787-7.792 4.76-10.517 9.357-.102.172-.163.355-.209.542-1.38.215-2.6.903-3.442 1.993-.916 1.185-1.295 2.695-1.066 4.254l.216 1.473 1.473.217c.293.043.589.064.88.064 2.743 0 4.949-1.909 5.367-4.564.188-.047.371-.115.544-.218 4.598-2.728 7.571-6.069 9.355-10.517.298-.743.123-1.593-.442-2.159zm-14.824 15.458c-.192 0-.389-.016-.59-.044-.309-2.104 1.055-3.81 3-4.021l1.021 1.021c-.192 1.76-1.605 3.044-3.431 3.044zm4.89-4.502l-1.021-1.021c.38-.641.774-1.233 1.178-1.804.027.041 1.639 1.653 1.639 1.653-.568.401-1.158.794-1.796 1.172zm2.608-1.773s-1.821-1.801-1.879-1.824c2.147-2.784 4.651-4.685 7.791-5.943-1.255 3.127-3.144 5.623-5.912 7.767z"
        /> < title > { title } < / title > < / svg >
    }
}
