#[cfg(feature = "SiTalend")]
use leptos::*;
#[cfg(feature = "SiTalend")]
///This icon requires the feature `SiTalend` to be enabled.
#[component]
pub fn Talend(
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
        "M10.875.025c-.41.043-1.233.19-1.795.324-.653.156-1.099.303-1.856.632A11.96 11.96 0 0 0 .974 7.23C.531 8.25.367 8.817.12 10.166c-.117.61-.121.722-.121 1.838s.004 1.228.121 1.838c.247 1.349.411 1.915.852 2.936a11.96 11.96 0 0 0 6.251 6.249c1.021.441 1.588.605 2.937.852.61.117.723.121 1.839.121s1.229-.004 1.839-.121c1.35-.247 1.916-.41 2.937-.852a11.96 11.96 0 0 0 6.25-6.249c.442-1.02.606-1.587.853-2.936.117-.61.121-.722.121-1.838s-.004-1.228-.121-1.838c-.247-1.35-.411-1.916-.852-2.936-1.315-3.062-3.842-5.415-7.06-6.582C15.513.483 14.764.302 13.95.15c-.645-.12-.822-.134-1.735-.147-.558-.008-1.163 0-1.34.022zm1.536 5.34.108.104v2.859h2.293l.073.117c.139.212.06.735-.134.934-.069.065-.194.073-1.155.073h-1.081l.013 3.49c.013 3.174.021 3.516.09 3.715.148.445.364.597.831.592.329 0 .597-.064 1.224-.302.381-.139.411-.143.485-.078.1.09.26.424.26.536 0 .143-.347.398-.926.68-.922.444-1.661.613-2.47.557-1.519-.104-2.367-.614-2.678-1.617-.087-.277-.09-.398-.104-3.931l-.013-3.642h-.554c-.618 0-.679-.026-.722-.311-.035-.203.1-.342.459-.467a6.057 6.057 0 0 0 2.496-1.717c.354-.415.48-.601.748-1.116.125-.237.272-.467.32-.506a.318.318 0 0 1 .437.03z"
        /> < title > { title } < / title > < / svg >
    }
}
