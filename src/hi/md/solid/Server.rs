#[cfg(feature = "HiMdSolidServer")]
use leptos::*;
#[cfg(feature = "HiMdSolidServer")]
///This icon requires the feature `HiMdSolidServer` to be enabled.
#[component]
pub fn Server(
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
        "M4.63196 3.53326C4.84778 2.63402 5.65196 2 6.57674 2H13.4231C14.3479 2 15.1521 2.63401 15.3679 3.53325L17.3441 11.7674C16.9303 11.5951 16.4762 11.5 15.9999 11.5H3.99994C3.52368 11.5 3.06963 11.5951 2.65576 11.7674L4.63196 3.53326Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M4 13C2.89543 13 2 13.8954 2 15C2 16.1046 2.89543 17 4 17H16C17.1046 17 18 16.1046 18 15C18 13.8954 17.1046 13 16 13H4ZM15.2402 15C15.2402 14.5858 15.576 14.25 15.9902 14.25H16.0002C16.4144 14.25 16.7502 14.5858 16.7502 15V15.01C16.7502 15.4242 16.4144 15.76 16.0002 15.76H15.9902C15.576 15.76 15.2402 15.4242 15.2402 15.01V15ZM12.9902 14.25C12.576 14.25 12.2402 14.5858 12.2402 15V15.01C12.2402 15.4242 12.576 15.76 12.9902 15.76H13.0002C13.4144 15.76 13.7502 15.4242 13.7502 15.01V15C13.7502 14.5858 13.4144 14.25 13.0002 14.25H12.9902Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
