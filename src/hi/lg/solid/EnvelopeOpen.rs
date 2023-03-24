#[cfg(feature = "HiLgSolidEnvelopeOpen")]
use leptos::*;
#[cfg(feature = "HiLgSolidEnvelopeOpen")]
///This icon requires the feature `HiLgSolidEnvelopeOpen` to be enabled.
#[component]
pub fn EnvelopeOpen(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M19.5 22.5002C21.1569 22.5002 22.5 21.1571 22.5 19.5002V11.3264L15.6212 15.3483L19.1056 17.2245C19.4703 17.4209 19.6067 17.8757 19.4104 18.2404C19.214 18.6051 18.7591 18.7416 18.3944 18.5452L12.7112 15.485C12.2672 15.2459 11.7328 15.2459 11.2889 15.485L5.60558 18.5452C5.24087 18.7416 4.78603 18.6051 4.58965 18.2404C4.39327 17.8757 4.52972 17.4209 4.89442 17.2245L8.37878 15.3483L1.5 11.3264V19.5002C1.5 21.1571 2.84315 22.5002 4.5 22.5002L19.5 22.5002Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M1.5 9.58886V8.84412C1.5 7.74048 2.10597 6.72595 3.0777 6.20271L10.5777 2.16425C11.4656 1.68614 12.5344 1.68614 13.4223 2.16425L20.9223 6.20271C21.894 6.72595 22.5 7.74048 22.5 8.84413V9.58886L14.0742 14.5153L13.4223 14.1643C12.5344 13.6862 11.4656 13.6862 10.5777 14.1643L9.92585 14.5153L1.5 9.58886Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
