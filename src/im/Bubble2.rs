#[cfg(feature = "ImBubble2")]
use leptos::*;
#[cfg(feature = "ImBubble2")]
///This icon requires the feature `ImBubble2` to be enabled.
#[component]
pub fn Bubble2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M8 3c-0.858 0-1.687 0.135-2.464 0.402-0.73 0.251-1.38 0.605-1.932 1.054-1.035 0.841-1.604 1.922-1.604 3.044 0 0.63 0.175 1.24 0.52 1.815 0.356 0.592 0.89 1.134 1.547 1.566 0.474 0.312 0.793 0.812 0.878 1.373 0.028 0.187 0.046 0.376 0.053 0.564 0.117-0.097 0.23-0.201 0.342-0.312 0.377-0.377 0.887-0.586 1.414-0.586 0.084 0 0.168 0.005 0.252 0.016 0.328 0.042 0.662 0.063 0.995 0.063 0.858 0 1.687-0.135 2.464-0.402 0.73-0.251 1.38-0.605 1.932-1.054 1.035-0.841 1.604-1.922 1.604-3.044s-0.57-2.203-1.604-3.044c-0.552-0.448-1.202-0.803-1.932-1.054-0.777-0.267-1.606-0.402-2.464-0.402zM8 1v0c4.418 0 8 2.91 8 6.5s-3.582 6.5-8 6.5c-0.424 0-0.841-0.027-1.247-0.079-1.718 1.718-3.77 2.027-5.753 2.072v-0.421c1.071-0.525 2-1.48 2-2.572 0-0.152-0.012-0.302-0.034-0.448-1.809-1.192-2.966-3.012-2.966-5.052 0-3.59 3.582-6.5 8-6.5z"
        /> < title > { title } < / title > < / svg >
    }
}
