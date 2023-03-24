#[cfg(feature = "SiPlanet")]
use leptos::*;
#[cfg(feature = "SiPlanet")]
///This icon requires the feature `SiPlanet` to be enabled.
#[component]
pub fn Planet(
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
        "M12.891 6.582c-1.159 0-2.4.457-3.217 1.633h-.033a1.59 1.59 0 0 0-1.59-1.59h-.048v10.86a1.792 1.792 0 0 0 1.784 1.784v-4.703h.034c.343.571 1.29 1.536 3.185 1.536 2.857 0 4.572-2.352 4.572-4.638.002-2.416-1.616-4.882-4.687-4.882zm-.066 7.975c-1.714 0-3.07-1.388-3.07-3.217 0-1.666 1.242-3.2 3.023-3.2 1.845 0 3.103 1.616 3.103 3.233-.001 1.905-1.455 3.184-3.056 3.184zM12.001 24A12 12 0 1 1 24 12.001 12.013 12.013 0 0 1 12.001 24zm0-22.856a10.861 10.861 0 1 0 10.861 10.862 10.87 10.87 0 0 0-10.86-10.862z"
        /> < title > { title } < / title > < / svg >
    }
}
