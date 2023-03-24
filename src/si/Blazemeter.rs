#[cfg(feature = "SiBlazemeter")]
use leptos::*;
#[cfg(feature = "SiBlazemeter")]
///This icon requires the feature `SiBlazemeter` to be enabled.
#[component]
pub fn Blazemeter(
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
        "M15.04 17.135c-.256 0-.352-.128-.32-.352l1.696-9.566c.032-.224.16-.352.416-.352h3.584c2.4 0 3.584.736 3.584 2.24 0 .607-.16 1.151-.48 1.6-.32.48-.768.8-1.344.991.384.096.736.32.96.672.256.32.384.768.384 1.28 0 1.215-.416 2.08-1.248 2.655-.832.544-1.952.832-3.328.832H15.04zm3.968-1.664c1.664 0 2.496-.608 2.496-1.823 0-.384-.16-.64-.448-.832-.288-.192-.8-.256-1.472-.256h-2.08l-.512 2.88h2.016v.031zm.768-4.383c1.472 0 2.176-.544 2.176-1.663 0-.352-.128-.576-.416-.736-.288-.16-.736-.224-1.344-.224h-1.984l-.48 2.623h2.048zm-6.88-.256a.608.608 0 0 0-.608-.608H.608a.608.608 0 1 0 0 1.216h11.648c.352 0 .64-.256.64-.608zm1.344-2.175a.608.608 0 0 0-.608-.608H6.464a.608.608 0 1 0 0 1.216h7.168c.32 0 .608-.256.608-.608zm-2.464 6.654a.608.608 0 0 0-.608-.608H8.256a.608.608 0 1 0 0 1.216h2.912c.32 0 .608-.256.608-.608zm-.864-2.271a.608.608 0 0 0-.608-.608H3.2a.608.608 0 1 0 0 1.216h7.104a.63.63 0 0 0 .608-.608z"
        /> < title > { title } < / title > < / svg >
    }
}
