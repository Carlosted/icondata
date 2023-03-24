#[cfg(feature = "TiLinkOutline")]
use leptos::*;
#[cfg(feature = "TiLinkOutline")]
///This icon requires the feature `TiLinkOutline` to be enabled.
#[component]
pub fn LinkOutline(
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
        "M17.5 5.999c.282 0 .562.106.777.321.431.431.431 1.127 0 1.557l-1.72 1.723.307.308c.585.584.906 1.362.906 2.192s-.321 1.607-.906 2.191l-4.172 4.172c-.584.584-1.361.906-2.191.906s-1.607-.322-2.191-.906l-.31-.309-1.723 1.723c-.215.215-.495.322-.777.322s-.562-.107-.777-.322c-.431-.43-.431-1.126 0-1.557l1.72-1.72-.308-.309c-.583-.584-.905-1.361-.905-2.191s.321-1.608.905-2.192l4.173-4.173c.584-.584 1.387-.875 2.191-.875s1.607.291 2.191.875l.31.308 1.723-1.723c.215-.215.495-.321.777-.321m0-2c-.828 0-1.605.321-2.191.908l-.492.491c-.707-.351-1.504-.539-2.316-.539-1.363 0-2.677.533-3.605 1.461l-4.172 4.173c-.964.962-1.494 2.241-1.494 3.607 0 .822.192 1.616.558 2.327l-.479.48c-.586.585-.909 1.364-.909 2.193 0 .827.322 1.605.908 2.191.584.586 1.363.908 2.191.908s1.605-.322 2.191-.908l.48-.48c.711.365 1.504.559 2.328.559 1.363 0 2.645-.53 3.605-1.492l4.172-4.172c.963-.962 1.492-2.242 1.492-3.605 0-.824-.192-1.617-.558-2.328l.479-.48c.589-.587.912-1.366.912-2.193 0-.828-.322-1.606-.908-2.192-.587-.588-1.364-.909-2.192-.909zM11.4 11.168c.017.535.233 1.036.613 1.416.381.38.881.598 1.416.614l-1.832 1.831c-.017-.534-.234-1.035-.613-1.415-.381-.38-.881-.597-1.416-.614l1.832-1.832m1.1-2.139c-.242 0-.468.094-.637.262l-4.172 4.172c-.168.168-.26.395-.26.637 0 .24.092.467.26.635l.309.308.723-.723c.215-.215.495-.321.777-.321s.562.106.777.321c.431.431.431 1.127 0 1.557l-.72.723.308.308c.168.168.401.253.636.253s.468-.084.637-.253l4.172-4.173c.168-.168.26-.395.26-.635 0-.242-.092-.469-.26-.637l-.31-.309-.723.723c-.215.215-.495.322-.777.322s-.562-.107-.777-.322c-.431-.43-.431-1.126 0-1.557l.72-.72-.307-.309c-.169-.168-.395-.262-.636-.262z"
        /> < title > { title } < / title > < / svg >
    }
}
