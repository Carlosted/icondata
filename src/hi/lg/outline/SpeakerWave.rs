#[cfg(feature = "HiLgOutlineSpeakerWave")]
use leptos::*;
#[cfg(feature = "HiLgOutlineSpeakerWave")]
///This icon requires the feature `HiLgOutlineSpeakerWave` to be enabled.
#[component]
pub fn SpeakerWave(
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
        "M19.114 5.63591C22.6287 9.15063 22.6287 14.8491 19.114 18.3638M16.4626 8.28765C18.5129 10.3379 18.5129 13.662 16.4626 15.7123M6.75 8.24993L11.4697 3.53026C11.9421 3.05778 12.75 3.39241 12.75 4.06059V19.9393C12.75 20.6074 11.9421 20.9421 11.4697 20.4696L6.75 15.7499H4.50905C3.62971 15.7499 2.8059 15.2435 2.57237 14.3957C2.36224 13.6329 2.25 12.8295 2.25 11.9999C2.25 11.1703 2.36224 10.367 2.57237 9.60416C2.8059 8.7564 3.62971 8.24993 4.50905 8.24993H6.75Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
