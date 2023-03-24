#[cfg(feature = "SiVenmo")]
use leptos::*;
#[cfg(feature = "SiVenmo")]
///This icon requires the feature `SiVenmo` to be enabled.
#[component]
pub fn Venmo(
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
        "M3.94 9.72c.17.27.24.54.24.89 0 1.11-.95 2.55-1.72 3.57H.71L0 9.96l1.54-.15.37 3c.35-.56.78-1.45.78-2.06 0-.33-.06-.56-.15-.75l1.4-.28zM5.94 11.57c.28 0 1-.13 1-.53 0-.2-.14-.3-.3-.3-.29 0-.66.35-.7.83zm-.03.8c0 .5.27.7.64.7.4 0 .77-.1 1.27-.35l-.19 1.26c-.35.17-.89.28-1.42.28-1.33 0-1.81-.8-1.81-1.82 0-1.31.78-2.7 2.38-2.7.88 0 1.38.49 1.38 1.18 0 1.1-1.43 1.45-2.25 1.46zM12.6 10.7c0 .17-.03.4-.05.56l-.46 2.92h-1.5l.42-2.68.03-.3c0-.2-.12-.24-.26-.24-.2 0-.4.09-.52.15l-.48 3.07h-1.5l.68-4.37h1.3l.02.35c.31-.2.72-.43 1.29-.43.76 0 1.03.4 1.03.98zM17.05 10.21c.43-.3.83-.48 1.4-.48.76 0 1.03.4 1.03.98a4 4 0 0 1-.05.55l-.46 2.92h-1.5l.43-2.73.02-.22c0-.22-.12-.27-.27-.27-.18 0-.37.08-.5.15l-.48 3.07h-1.5l.43-2.74.02-.21c0-.22-.12-.27-.27-.27-.2 0-.39.09-.52.15l-.47 3.06h-1.51l.69-4.36h1.28l.05.36c.3-.22.7-.44 1.24-.44.48 0 .78.2.94.48zM22.46 11.48c0-.35-.09-.6-.35-.6-.6 0-.72 1.05-.72 1.58 0 .41.11.66.38.66.56 0 .69-1.1.69-1.64zm-2.6.92c0-1.38.74-2.67 2.41-2.67 1.27 0 1.73.75 1.73 1.78 0 1.36-.72 2.77-2.44 2.77-1.27 0-1.7-.83-1.7-1.88z"
        /> < title > { title } < / title > < / svg >
    }
}
