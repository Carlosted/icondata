#[cfg(feature = "HiMdSolidQueueList")]
use leptos::*;
#[cfg(feature = "HiMdSolidQueueList")]
///This icon requires the feature `HiMdSolidQueueList` to be enabled.
#[component]
pub fn QueueList(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 4.5C2 3.11929 3.11929 2 4.5 2H15.5C16.8807 2 18 3.11929 18 4.5C18 5.88071 16.8807 7 15.5 7H4.5C3.11929 7 2 5.88071 2 4.5Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.75 9.08337C2.33579 9.08337 2 9.41916 2 9.83337C2 10.2476 2.33579 10.5834 2.75 10.5834H17.25C17.6642 10.5834 18 10.2476 18 9.83337C18 9.41916 17.6642 9.08337 17.25 9.08337H2.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.75 12.6633C2.33579 12.6633 2 12.9991 2 13.4133C2 13.8275 2.33579 14.1633 2.75 14.1633H17.25C17.6642 14.1633 18 13.8275 18 13.4133C18 12.9991 17.6642 12.6633 17.25 12.6633H2.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.75 16.25C2.33579 16.25 2 16.5858 2 17C2 17.4143 2.33579 17.75 2.75 17.75H17.25C17.6642 17.75 18 17.4143 18 17C18 16.5858 17.6642 16.25 17.25 16.25H2.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
