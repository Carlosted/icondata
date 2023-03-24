#[cfg(feature = "ImYahoo2")]
use leptos::*;
#[cfg(feature = "ImYahoo2")]
///This icon requires the feature `ImYahoo2` to be enabled.
#[component]
pub fn Yahoo2(
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
        "M8.019 1.087c-2.828 0-5.5-0.372-8.019-1.087 0 5.653 0 14.581 0 16 2.522-0.716 5.194-1.088 8.019-1.088 2.794 0 5.459 0.363 7.981 1.088 0-5.444 0-10.153 0-16-2.522 0.725-5.184 1.087-7.981 1.087zM12.45 2.453l-0.097 0.153c-0.091 0.144-0.172 0.266-0.284 0.438-0.15 0.225-0.431 0.672-0.769 1.247-0.094 0.159-0.209 0.35-0.328 0.556-0.228 0.384-0.484 0.819-0.688 1.162-0.084 0.147-0.169 0.297-0.256 0.447-0.225 0.391-0.456 0.794-0.678 1.181-0.228 0.403-0.453 0.8-0.678 1.194v0.397c0 0.55 0.012 1.15 0.031 1.684 0.009 0.244 0.019 0.678 0.031 1.137 0.012 0.547 0.025 1.113 0.041 1.4l0.003 0.088v0.009l-0.094-0.025c-0.037-0.009-0.072-0.019-0.109-0.028-0.113-0.025-0.234-0.044-0.353-0.056-0.072-0.006-0.147-0.009-0.222-0.009 0 0 0 0 0 0s0 0 0 0c-0.075 0-0.15 0.003-0.222 0.009-0.119 0.012-0.241 0.031-0.353 0.056-0.037 0.009-0.075 0.019-0.109 0.028l-0.094 0.025v-0.009l0.003-0.088c0.013-0.284 0.028-0.853 0.041-1.4 0.009-0.459 0.022-0.894 0.031-1.137 0.022-0.537 0.031-1.134 0.031-1.684v-0.397c-0.225-0.397-0.45-0.791-0.678-1.194-0.222-0.391-0.453-0.791-0.675-1.181-0.088-0.15-0.172-0.3-0.256-0.447-0.2-0.347-0.459-0.781-0.688-1.162-0.122-0.203-0.237-0.397-0.328-0.556-0.338-0.575-0.619-1.019-0.769-1.247-0.112-0.172-0.194-0.294-0.284-0.438l-0.097-0.153 0.175 0.050c0.222 0.063 0.45 0.094 0.694 0.094s0.478-0.031 0.697-0.094l0.053-0.016 0.028 0.047c0.431 0.778 1.591 2.684 2.284 3.825 0.237 0.394 0.428 0.703 0.522 0.862 0 0 0 0 0-0.003 0 0 0 0 0 0.003 0.094-0.156 0.284-0.469 0.522-0.862 0.694-1.138 1.853-3.044 2.284-3.825l0.028-0.047 0.053 0.016c0.219 0.063 0.453 0.094 0.697 0.094s0.472-0.031 0.694-0.094l0.166-0.050z"
        /> < title > { title } < / title > < / svg >
    }
}
