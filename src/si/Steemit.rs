#[cfg(feature = "SiSteemit")]
use leptos::*;
#[cfg(feature = "SiSteemit")]
///This icon requires the feature `SiSteemit` to be enabled.
#[component]
pub fn Steemit(
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
        "M20.982 7.246c-1-1-2.163-1.73-3.4-2.202.721-2.12 3.277-2.892 3.277-2.892S14.895-.944 8.31.29C6.114.64 4.008 1.988 2.468 3.768a9.152 9.152 0 0 0 .991 12.984c.598.512 1.934 1.308 1.958 1.349-.876 2.226-3.678 2.835-3.678 2.835s3.703 2.316 8.676 2.917c1.319.163 2.72.204 4.17.05 2.302-.188 4.596-1.406 6.34-3.12a9.525 9.525 0 0 0 .058-13.538zm-1.704 12.058c-1.41 1.382-3.515 2.26-5.137 2.381-1.147.114-2.376.13-3.555-.016-1.737-.212-2.99-.69-4.22-1.154.623-.601 1.246-1.43 1.5-2.145a1.652 1.652 0 0 0-.05-1.308 6.995 6.995 0 0 1 1.426-8.077 7.196 7.196 0 0 1 5.931-1.99 7.142 7.142 0 0 1 4.228 2.047c2.785 2.795 2.687 7.524-.123 10.262z"
        /> < title > { title } < / title > < / svg >
    }
}
