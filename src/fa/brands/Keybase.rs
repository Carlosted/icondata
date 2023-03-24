#[cfg(feature = "FaBrandsKeybase")]
use leptos::*;
#[cfg(feature = "FaBrandsKeybase")]
///This icon requires the feature `FaBrandsKeybase` to be enabled.
#[component]
pub fn Keybase(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M286.17 419a18 18 0 1 0 18 18 18 18 0 0 0-18-18zm111.92-147.6c-9.5-14.62-39.37-52.45-87.26-73.71q-9.1-4.06-18.38-7.27a78.43 78.43 0 0 0-47.88-104.13c-12.41-4.1-23.33-6-32.41-5.77-.6-2-1.89-11 9.4-35L198.66 32l-5.48 7.56c-8.69 12.06-16.92 23.55-24.34 34.89a51 51 0 0 0-8.29-1.25c-41.53-2.45-39-2.33-41.06-2.33-50.61 0-50.75 52.12-50.75 45.88l-2.36 36.68c-1.61 27 19.75 50.21 47.63 51.85l8.93.54a214 214 0 0 0-46.29 35.54C14 304.66 14 374 14 429.77v33.64l23.32-29.8a148.6 148.6 0 0 0 14.56 37.56c5.78 10.13 14.87 9.45 19.64 7.33 4.21-1.87 10-6.92 3.75-20.11a178.29 178.29 0 0 1-15.76-53.13l46.82-59.83-24.66 74.11c58.23-42.4 157.38-61.76 236.25-38.59 34.2 10.05 67.45.69 84.74-23.84.72-1 1.2-2.16 1.85-3.22a156.09 156.09 0 0 1 2.8 28.43c0 23.3-3.69 52.93-14.88 81.64-2.52 6.46 1.76 14.5 8.6 15.74 7.42 1.57 15.33-3.1 18.37-11.15C429 443 434 414 434 382.32c0-38.58-13-77.46-35.91-110.92zM142.37 128.58l-15.7-.93-1.39 21.79 13.13.78a93 93 0 0 0 .32 19.57l-22.38-1.34a12.28 12.28 0 0 1-11.76-12.79L107 119c1-12.17 13.87-11.27 13.26-11.32l29.11 1.73a144.35 144.35 0 0 0-7 19.17zm148.42 172.18a10.51 10.51 0 0 1-14.35-1.39l-9.68-11.49-34.42 27a8.09 8.09 0 0 1-11.13-1.08l-15.78-18.64a7.38 7.38 0 0 1 1.34-10.34l34.57-27.18-14.14-16.74-17.09 13.45a7.75 7.75 0 0 1-10.59-1s-3.72-4.42-3.8-4.53a7.38 7.38 0 0 1 1.37-10.34L214 225.19s-18.51-22-18.6-22.14a9.56 9.56 0 0 1 1.74-13.42 10.38 10.38 0 0 1 14.3 1.37l81.09 96.32a9.58 9.58 0 0 1-1.74 13.44zM187.44 419a18 18 0 1 0 18 18 18 18 0 0 0-18-18z"
        /> < title > { title } < / title > < / svg >
    }
}
