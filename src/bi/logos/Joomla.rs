#[cfg(feature = "BiLogosJoomla")]
use leptos::*;
#[cfg(feature = "BiLogosJoomla")]
///This icon requires the feature `BiLogosJoomla` to be enabled.
#[component]
pub fn Joomla(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m15.539 14.059-1.874 1.875-1.777 1.777-.347.35a3.993 3.993 0 0 1-3.785 1.048A2.41 2.41 0 0 1 3 18.567c0-1.138.792-2.092 1.852-2.342a3.993 3.993 0 0 1 1.047-3.811l.135-.135 1.777 1.778-.138.135a1.48 1.48 0 0 0 0 2.092 1.462 1.462 0 0 0 2.09 0l.349-.349 1.775-1.778 1.877-1.879 1.775 1.781zm.693 4.988a3.986 3.986 0 0 1-3.996-.988l-.135-.139 1.773-1.777.135.139a1.48 1.48 0 0 0 2.09 0 1.474 1.474 0 0 0-.002-2.086l-.35-.349-1.773-1.777-1.877-1.878 1.777-1.776 1.875 1.879 1.774 1.777.349.349a3.962 3.962 0 0 1 1.058 3.766 2.407 2.407 0 0 1-.336 4.79 2.392 2.392 0 0 1-2.352-1.924l-.01-.006zm-8.001-8.962 1.881-1.879 1.777-1.777.347-.346a3.972 3.972 0 0 1 3.949-1.002 2.408 2.408 0 1 1 2.699 2.716 3.98 3.98 0 0 1-1.012 3.925l-.137.139-1.777-1.777.139-.138a1.474 1.474 0 1 0-2.086-2.085l-.347.346-1.777 1.776-1.879 1.876-1.777-1.774zm-1.99 1.984-.346-.347a3.984 3.984 0 0 1-.999-3.965 2.414 2.414 0 0 1-1.874-2.35A2.41 2.41 0 0 1 5.43 3c1.197 0 2.19.875 2.378 2.019a3.99 3.99 0 0 1 3.734 1.061l.138.14-1.778 1.776-.137-.136a1.481 1.481 0 0 0-2.088 0 1.481 1.481 0 0 0-.004 2.092l.349.35 1.777 1.777 1.879 1.879-1.775 1.777-1.883-1.879-1.778-1.777v-.01h-.001z"
        /> < title > { title } < / title > < / svg >
    }
}
