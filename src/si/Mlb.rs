#[cfg(feature = "SiMlb")]
use leptos::*;
#[cfg(feature = "SiMlb")]
///This icon requires the feature `SiMlb` to be enabled.
#[component]
pub fn Mlb(
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
        "M22.003 5.53a1.988 1.988 0 011.99 1.827l.007.177v8.927a2.007 2.007 0 01-1.848 2.002l-.15.006H2.002a2.003 2.003 0 01-1.42-.586 1.979 1.979 0 01-.575-1.245L0 16.46V7.535a1.994 1.994 0 01.583-1.427 2.016 2.016 0 011.242-.57l.176-.007h20.002zm-7.5.703H2.001a1.312 1.312 0 00-.926.375 1.299 1.299 0 00-.374.926v8.927a1.282 1.282 0 00.374.922c.246.245.579.383.926.384h8.116a92.275 92.275 0 00-.495-.843H8.66c0-2.402.812-3.73 1.784-3.995.133-.024.07-.68-.1-.88h-.561c-.09 0-.038-.169-.038-.169l.456-.972-.062-.266H8.455l1.383-.967c.064-2.55 2.68-2.748 4.255-1.686.937.62 1.008 1.849.937 2.698-.012.054-.244.018-.244.018s-.16.937.257.937h1.838c.749-.03 1.472.478 1.472.478l.176-.643-4.026-5.244zm7.5 0h-5.59l3.207 5.36.296.038.187.245v.229l.202.041.188.255v.221l.21.037.211.23v.51c.252.228.536.418.843.563.283.108.315.562.486.803.213.352.505.493.444.69-.143.526-.682 1.414-1.184 1.456h-1.986v.85h2.488a1.3 1.3 0 001.294-1.311V7.534h-.002a1.285 1.285 0 00-1.294-1.3zM4.022 14.404a.888.888 0 01-.638 1.513.879.879 0 01-.88-.888.888.888 0 011.518-.625z"
        /> < title > { title } < / title > < / svg >
    }
}
