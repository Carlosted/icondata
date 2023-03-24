#[cfg(feature = "SiAircanada")]
use leptos::*;
#[cfg(feature = "SiAircanada")]
///This icon requires the feature `SiAircanada` to be enabled.
#[component]
pub fn Aircanada(
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
        "M12.394 16.958c0-.789.338-.902 1.127-.451a54.235 54.235 0 0 0 2.704 1.465c0-.45.451-.789 1.24-.564.789.226 1.577.338 1.577.338s-.45-1.014-.676-1.464c-.338-.789 0-1.24.338-1.352 0 0-.45-.338-.789-.564-.676-.45-.563-1.014.113-1.24.902-.45 2.141-.9 2.141-.9-.338-.226-.789-.79-.338-1.578.45-.676 1.24-1.69 1.24-1.69H18.93c-.79 0-1.015-.676-1.015-1.127 0 0-1.239.901-2.14 1.465-.79.563-1.465 0-1.352-.902a37 37 0 0 0 .338-2.93c-.451.451-1.24.339-1.69-.337-.564-1.127-1.127-2.48-1.127-2.48S11.38 4 10.817 5.128c-.338.676-1.127.788-1.578.45a37 37 0 0 0 .338 2.93c.113.789-.563 1.352-1.352.789-.901-.564-2.253-1.465-2.253-1.465 0 .45-.226 1.014-1.014 1.127H2.817s.789 1.014 1.24 1.69c.45.676 0 1.352-.339 1.577 0 0 1.127.564 2.141.902.676.338.902.788.113 1.24-.226.225-.789.563-.789.563.45.112.789.563.45 1.352-.225.45-.675 1.464-.675 1.464s.788-.225 1.577-.338c.789-.225 1.127.226 1.24.564 0 0 1.352-.789 2.704-1.465.676-.45 1.127-.225 1.127.45v1.916c0 1.127-.226 2.254-.564 2.93-5.07-.564-9.352-4.62-9.352-10.028 0-5.521 4.62-10.029 10.366-10.029 5.747 0 10.367 4.508 10.367 10.029 0 5.183-4.057 9.464-9.24 10.028v1.352C19.268 22.592 24 17.746 24 11.775 24 5.352 18.592.282 11.944.282 5.408.282 0 5.352 0 11.662c0 5.521 4.169 10.14 9.69 11.155.902.225 1.465.338 2.028.901.564-1.126.676-3.38.676-4.62Z"
        /> < title > { title } < / title > < / svg >
    }
}
