#[cfg(feature = "SiUblockorigin")]
use leptos::*;
#[cfg(feature = "SiUblockorigin")]
///This icon requires the feature `SiUblockorigin` to be enabled.
#[component]
pub fn Ublockorigin(
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
        "M12 0C7.502 3 6.002 3 1.5 3c0 15.002 0 15.002 10.5 21 10.5-5.998 10.5-5.998 10.5-21-4.498 0-5.998 0-10.5-3zM5.956 7.472h1.512v4.536c0 1.322.19 1.508 1.512 1.508 1.323 0 1.512-.19 1.512-1.512V7.472H12v.767a3.75 3.75 0 012.268-.767 3.79 3.79 0 013.776 3.78 3.79 3.79 0 01-3.78 3.775 3.765 3.764 0 01-2.684-1.133c-.464.77-1.315 1.133-2.6 1.133-2.079 0-3.024-.944-3.024-3.023zm8.308 1.512A2.254 2.254 0 0012 11.252a2.254 2.254 0 002.268 2.264 2.254 2.254 0 002.264-2.268 2.254 2.254 0 00-2.268-2.264z"
        /> < title > { title } < / title > < / svg >
    }
}
