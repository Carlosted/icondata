#[cfg(feature = "CgChanel")]
use leptos::*;
#[cfg(feature = "CgChanel")]
///This icon requires the feature `CgChanel` to be enabled.
#[component]
pub fn Chanel(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M6.07178 3.17291C4.32593 3.5202 2.72229 4.37738 1.46362 5.63605L3.59332 7.76575C4.43079 6.92835 5.4978 6.35803 6.65936 6.12695C7.53564 5.95264 8.43604 5.9773 9.29523 6.19452C9.0791 6.45056 8.87671 6.71936 8.68927 6.99988C7.70032 8.47992 7.17249 10.22 7.17249 12C7.17249 13.78 7.70032 15.5201 8.68927 17.0001C8.87671 17.2806 9.07916 17.5494 9.29529 17.8055C8.4361 18.0227 7.53564 18.0474 6.65936 17.873C5.4978 17.642 4.43079 17.0717 3.59332 16.2343L1.46362 18.364C2.72229 19.6226 4.32593 20.4798 6.07178 20.8271C7.81757 21.1743 9.6272 20.9961 11.2717 20.3149C11.5201 20.212 11.7632 20.0983 12.0001 19.9744C12.2369 20.0983 12.4799 20.212 12.7283 20.3149C14.3729 20.9961 16.1825 21.1743 17.9283 20.8271C19.6741 20.4798 21.2778 19.6226 22.5364 18.364L20.4067 16.2343C19.5693 17.0717 18.5023 17.642 17.3407 17.873C16.4644 18.0474 15.564 18.0227 14.7048 17.8055C14.9209 17.5494 15.1234 17.2806 15.3108 17.0001C16.2997 15.5201 16.8276 13.78 16.8276 12C16.8276 10.22 16.2997 8.47992 15.3108 6.99988C15.1234 6.71936 14.921 6.45056 14.7048 6.19452C15.564 5.9773 16.4644 5.95264 17.3407 6.12695C18.5023 6.35803 19.5693 6.92835 20.4067 7.76575L22.5364 5.63605C21.2778 4.37738 19.6741 3.5202 17.9283 3.17291C16.1825 2.82568 14.3729 3.00391 12.7283 3.68506C12.4799 3.78796 12.2369 3.90167 12.0001 4.02564C11.7632 3.90167 11.5201 3.78796 11.2717 3.68506C9.6272 3.00391 7.81757 2.82568 6.07178 3.17291ZM12.0001 7.7049C11.6994 7.99695 11.4288 8.32117 11.1935 8.67316C10.5356 9.6579 10.1844 10.8157 10.1844 12C10.1844 13.1843 10.5356 14.3421 11.1935 15.3268C11.4288 15.6788 11.6994 16.0031 12.0001 16.2951C12.3007 16.0031 12.5713 15.6788 12.8065 15.3268C13.4645 14.3421 13.8157 13.1843 13.8157 12C13.8157 10.8157 13.4645 9.6579 12.8065 8.67316C12.5713 8.32117 12.3007 7.99695 12.0001 7.7049Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
