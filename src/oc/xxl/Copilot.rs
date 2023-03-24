#[cfg(feature = "OcXxlCopilot")]
use leptos::*;
#[cfg(feature = "OcXxlCopilot")]
///This icon requires the feature `OcXxlCopilot` to be enabled.
#[component]
pub fn Copilot(
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
        stroke_witdh = "0" style = style width = "96" height = "96" viewBox = "0 0 96 96"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M38 54a4 4 0 0 1 4 4v8a4 4 0 0 1-8 0v-8a4 4 0 0 1 4-4Zm24 4a4 4 0 0 0-8 0v8a4 4 0 1 0 8 0v-8Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M25.013 15.024C31.008 10.628 39.145 8 48 8c8.855 0 16.992 2.628 22.987 7.024 3.64 2.67 6.553 6.05 8.278 9.92 1.615 2.744 2.114 5.94 1.968 9.397l3.898 9.026.355.07a10.996 10.996 0 0 1 6.794 4.394l2.416 3.382A7 7 0 0 1 96 55.282V65c0 2.116-1.238 3.947-2.258 5.17-1.122 1.347-2.547 2.616-3.868 3.673a52.899 52.899 0 0 1-3.647 2.653c-.51.34-1.025.671-1.546.993l-.061.037-.036.022-.002.001-.024.015-.158.099c-.134.083-.327.201-.576.348-.704.415-1.417.815-2.139 1.199a76.495 76.495 0 0 1-7.768 3.58C67.381 85.377 58.245 88 48 88c-10.245 0-19.381-2.622-25.917-5.21a76.495 76.495 0 0 1-7.768-3.58 55.966 55.966 0 0 1-2.14-1.199 34.102 34.102 0 0 1-.733-.447l-.024-.015-.002-.001-.038-.023-.059-.036-.345-.215a54.57 54.57 0 0 1-1.2-.778 52.918 52.918 0 0 1-3.648-2.653c-1.321-1.057-2.746-2.326-3.868-3.672C1.238 68.946 0 67.116 0 65v-9.718a7 7 0 0 1 1.304-4.069l2.416-3.382a10.998 10.998 0 0 1 6.794-4.393l.355-.071 3.898-9.026c-.146-3.457.353-6.654 1.968-9.398 1.725-3.87 4.637-7.249 8.278-9.919ZM16 46.62v26.671c.326.184.706.394 1.138.624a70.399 70.399 0 0 0 7.154 3.295C30.381 79.622 38.745 82 48 82c9.255 0 17.618-2.378 23.708-4.789a70.406 70.406 0 0 0 7.154-3.296c.432-.23.812-.44 1.138-.624V46.62l-.983-2.275a11.56 11.56 0 0 1-2.281 3.065c-3.167 3.016-7.836 3.88-13.914 3.348-5.658-.495-9.622-2.8-12.098-6.457a13.9 13.9 0 0 1-.213-.324c-1.674.031-3.348.031-5.022 0-.07.109-.141.217-.213.324-2.476 3.657-6.44 5.962-12.098 6.457-6.077.532-10.747-.332-13.914-3.348a11.586 11.586 0 0 1-2.282-3.065Zm24.307-5.683c1.427-2.108 2.097-5.107 2.247-8.826.185-4.606-.683-7.105-1.965-8.337-1.187-1.139-3.597-1.991-8.71-.981-5.191 1.025-7.925 2.568-9.368 4.328-1.388 1.692-2.022 4.13-1.677 8.076.199 2.276.53 4.003 1.007 5.316.358.985.799 1.738 1.328 2.315.075.082.153.161.233.237 1.21 1.153 3.471 2.073 8.261 1.789.32-.019.65-.043.992-.073 1.952-.171 3.472-.603 4.67-1.216a7.8 7.8 0 0 0 2.982-2.628Zm15.386 0a7.8 7.8 0 0 0 2.982 2.628c1.198.613 2.718 1.045 4.67 1.216 5.475.479 7.962-.486 9.253-1.716.634-.604 1.151-1.425 1.56-2.551.478-1.314.808-3.04 1.008-5.317.345-3.946-.289-6.384-1.677-8.076-1.443-1.76-4.177-3.303-9.368-4.328-5.113-1.01-7.523-.158-8.71.981-1.282 1.232-2.15 3.731-1.965 8.337.15 3.719.82 6.718 2.247 8.826Zm-9.27-5.997a6.498 6.498 0 0 1 3.154 0l.06.015a1.498 1.498 0 0 0 1.463-.399 1.498 1.498 0 0 0-.736-2.511l-.06-.015a9.504 9.504 0 0 0-4.608 0l-.06.015a1.498 1.498 0 0 0-.73 2.508 1.5 1.5 0 0 0 1.458.402l.06-.015Z"
        /> < title > { title } < / title > < / svg >
    }
}
