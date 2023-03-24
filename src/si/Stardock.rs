#[cfg(feature = "SiStardock")]
use leptos::*;
#[cfg(feature = "SiStardock")]
///This icon requires the feature `SiStardock` to be enabled.
#[component]
pub fn Stardock(
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
        "M22.337 3.28c-.108 0-.22.007-.336.017-1.553.129-3.886.917-6.557 2.217a7.326 7.326 0 0 0-3.71-.994c-4.124 0-7.478 3.354-7.478 7.496 0 .674.093 1.33.262 1.95-3.224 2.697-5.04 5.153-4.385 6.221.712 1.125 3.992.412 8.115-1.556a7.55 7.55 0 0 0 3.484.863c4.124 0 7.48-3.356 7.48-7.478 0-.544-.058-1.086-.17-1.592 3.504-2.867 5.529-5.491 4.816-6.615-.24-.375-.768-.545-1.521-.53Zm-4.324 1.708c-1.912.769-4.666 1.706-5.64 3.711-.564 1.143.371 2.436.84 3.035.47.62 1.35 2.174-.13 3.786-1.5 1.63-7.028 3.318-7.028 3.318 1.78-.843 4.91-2.06 5.396-4.16.375-1.593-1.142-2.493-1.555-3.205-.412-.712-.842-1.93 1.313-3.54 2.156-1.631 6.804-2.945 6.804-2.945Zm1.02.758c.67-.007 1.153.151 1.378.498.43.675-.207 1.95-1.556 3.393a7.514 7.514 0 0 0-2.323-3.393c.975-.318 1.832-.49 2.502-.498zM4.8 14.79a7.627 7.627 0 0 0 2.305 3.074c-1.762.525-3.074.524-3.467-.113-.394-.618.075-1.706 1.162-2.96z"
        /> < title > { title } < / title > < / svg >
    }
}
