#[cfg(feature = "TiDownloadOutline")]
use leptos::*;
#[cfg(feature = "TiDownloadOutline")]
///This icon requires the feature `TiDownloadOutline` to be enabled.
#[component]
pub fn DownloadOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.986 17c0-.105-.004-.211-.038-.316l-2-6c-.136-.409-.516-.684-.948-.684h-.561l.682-.678c1.17-1.17 1.17-3.072 0-4.242-.81-.812-2.068-1.078-3.121-.709v-1.371c0-1.654-1.346-3-3-3s-3 1.346-3 3v1.371c-1.052-.369-2.311-.103-3.121.709-1.17 1.17-1.17 3.072.002 4.244l.68.676h-.561c-.432 0-.812.275-.948.684l-2 6c-.034.105-.038.211-.038.316-.014 0-.014 5-.014 5 0 .553.447 1 1 1h16c.553 0 1-.447 1-1 0 0 0-5-.014-5zm-13.693-10.506c.189-.187.439-.293.707-.293s.518.104.707.293l2.293 2.293v-5.787c0-.552.448-1 1-1s1 .448 1 1v5.787l2.293-2.293c.379-.377 1.035-.377 1.414 0 .391.39.391 1.023.002 1.412l-4.709 4.684-4.707-4.682c-.391-.388-.391-1.024 0-1.414zm-.572 5.506h1.852l3.429 3.41 3.428-3.41h1.852l1.667 5h-13.894l1.666-5zm12.279 9h-14v-3h14v3z"
        /> < title > { title } < / title > < / svg >
    }
}
