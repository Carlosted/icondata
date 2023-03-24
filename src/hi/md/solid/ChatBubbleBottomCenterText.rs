#[cfg(feature = "HiMdSolidChatBubbleBottomCenterText")]
use leptos::*;
#[cfg(feature = "HiMdSolidChatBubbleBottomCenterText")]
///This icon requires the feature `HiMdSolidChatBubbleBottomCenterText` to be enabled.
#[component]
pub fn ChatBubbleBottomCenterText(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M10 2C7.76407 2 5.56943 2.17905 3.42976 2.52374C1.99339 2.75513 1 4.01325 1 5.42588V10.5741C1 11.9867 1.99338 13.2449 3.42976 13.4763C4.59764 13.6644 5.78193 13.8032 6.9804 13.8905C7.2601 13.9108 7.5012 14.0703 7.62247 14.3035L9.33459 17.596C9.46367 17.8443 9.7202 18 10 18C10.2798 18 10.5363 17.8443 10.6654 17.596L12.3775 14.3035C12.4988 14.0703 12.7399 13.9108 13.0196 13.8905C14.2181 13.8032 15.4024 13.6644 16.5702 13.4763C18.0066 13.2449 19 11.9867 19 10.5741V5.42588C19 4.01325 18.0066 2.75513 16.5702 2.52374C14.4306 2.17905 12.2359 2 10 2ZM6.75 6C6.33579 6 6 6.33579 6 6.75C6 7.16421 6.33579 7.5 6.75 7.5H13.25C13.6642 7.5 14 7.16421 14 6.75C14 6.33579 13.6642 6 13.25 6H6.75ZM6.75 8.5C6.33579 8.5 6 8.83579 6 9.25C6 9.66421 6.33579 10 6.75 10H10.25C10.6642 10 11 9.66421 11 9.25C11 8.83579 10.6642 8.5 10.25 8.5H6.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
