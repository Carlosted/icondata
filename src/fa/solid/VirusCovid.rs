#[cfg(feature = "FaSolidVirusCovid")]
use leptos::*;
#[cfg(feature = "FaSolidVirusCovid")]
///This icon requires the feature `FaSolidVirusCovid` to be enabled.
#[component]
pub fn VirusCovid(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M192 24c0-13.3 10.7-24 24-24h80c13.3 0 24 10.7 24 24s-10.7 24-24 24H280V81.6c30.7 4.2 58.8 16.3 82.3 34.1L386.1 92 374.8 80.6c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l56.6 56.6c9.4 9.4 9.4 24.6 0 33.9s-24.6 9.4-33.9 0L420 125.9l-23.8 23.8c17.9 23.5 29.9 51.7 34.1 82.3H464V216c0-13.3 10.7-24 24-24s24 10.7 24 24v80c0 13.3-10.7 24-24 24s-24-10.7-24-24V280H430.4c-4.2 30.7-16.3 58.8-34.1 82.3L420 386.1l11.3-11.3c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9l-56.6 56.6c-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9L386.1 420l-23.8-23.8c-23.5 17.9-51.7 29.9-82.3 34.1V464h16c13.3 0 24 10.7 24 24s-10.7 24-24 24H216c-13.3 0-24-10.7-24-24s10.7-24 24-24h16V430.4c-30.7-4.2-58.8-16.3-82.3-34.1L125.9 420l11.3 11.3c9.4 9.4 9.4 24.6 0 33.9s-24.6 9.4-33.9 0L46.7 408.7c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0L92 386.1l23.8-23.8C97.9 338.8 85.8 310.7 81.6 280H48v16c0 13.3-10.7 24-24 24s-24-10.7-24-24V216c0-13.3 10.7-24 24-24s24 10.7 24 24v16H81.6c4.2-30.7 16.3-58.8 34.1-82.3L92 125.9 80.6 137.2c-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9l56.6-56.6c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9L125.9 92l23.8 23.8c23.5-17.9 51.7-29.9 82.3-34.1V48H216c-13.3 0-24-10.7-24-24zm48 200a48 48 0 1 0 -96 0 48 48 0 1 0 96 0zm64 104a24 24 0 1 0 0-48 24 24 0 1 0 0 48z"
        /> < title > { title } < / title > < / svg >
    }
}
