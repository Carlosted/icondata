#[cfg(feature = "SiDcentertainment")]
use leptos::*;
#[cfg(feature = "SiDcentertainment")]
///This icon requires the feature `SiDcentertainment` to be enabled.
#[component]
pub fn Dcentertainment(
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
        "M5.215 8.787h2.154c.601 0 1.088.487 1.088 1.088v4.954c0 .6-.487 1.088-1.088 1.088H6.05V9.475a.159.159 0 00-.066-.129zM12 23.099a11.078 11.078 0 01-8.659-4.155.046.046 0 01.036-.074h5.936a.26.26 0 00.153-.05l2.27-1.648a.159.159 0 00.064-.128V7.616a.159.159 0 00-.065-.129L9.466 5.84a.261.261 0 00-.153-.05H2.886a.046.046 0 01-.037-.071A11.087 11.087 0 0112 .9c3.798 0 7.15 1.907 9.151 4.817a.046.046 0 01-.038.071h-1.597c-.052 0-.1.03-.123.079l-.353.757-1.082-.786a.26.26 0 00-.153-.05h-2.553a.261.261 0 00-.154.05L12.83 7.487a.159.159 0 00-.065.129v9.428c0 .05.024.098.065.128l2.27 1.648a.26.26 0 00.153.05h5.371c.038 0 .06.045.036.074A11.078 11.078 0 0112 23.1zM1.602 8.3l1.038.755c.043.03.068.08.068.132v8.73c0 .046-.06.063-.084.025A11.046 11.046 0 01.901 12c0-1.289.22-2.526.624-3.677a.05.05 0 01.077-.024zm13.67.488h3.225v1.776c0 .046.038.084.084.084h2.701a.098.098 0 00.096-.083l.535-3.374c.007-.044.066-.053.086-.013a11.053 11.053 0 011.1 4.823 11.05 11.05 0 01-1.39 5.382c-.022.04-.084.024-.084-.023v-3.084a.084.084 0 00-.084-.084h-2.96a.084.084 0 00-.084.084v1.642h-1.301a1.089 1.089 0 01-1.089-1.088V9.475a.159.159 0 00-.065-.129zM12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
