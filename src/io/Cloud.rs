#[cfg(feature = "IoCloud")]
use leptos::*;
#[cfg(feature = "IoCloud")]
///This icon requires the feature `IoCloud` to be enabled.
#[component]
pub fn Cloud(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M396,432H136c-36.44,0-70.36-12.57-95.51-35.41C14.38,372.88,0,340,0,304c0-36.58,13.39-68.12,38.72-91.22,18.11-16.53,42.22-28.25,69.18-33.87a16,16,0,0,0,11.37-9.15,156.24,156.24,0,0,1,42.05-56C187.76,91.69,220.5,80,256,80a153.57,153.57,0,0,1,107.14,42.9c24.73,23.81,41.5,55.28,49.18,92a16,16,0,0,0,12.12,12.39C470,237.42,512,270.43,512,328c0,33.39-12.24,60.78-35.41,79.23C456.23,423.43,428.37,432,396,432Z"
        /> < title > { title } < / title > < / svg >
    }
}
