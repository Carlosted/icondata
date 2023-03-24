#[cfg(feature = "SiHibob")]
use leptos::*;
#[cfg(feature = "SiHibob")]
///This icon requires the feature `SiHibob` to be enabled.
#[component]
pub fn Hibob(
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
        "M12.527 17.538c-2.077 0-3.767-1.705-3.767-3.8 0-2.096 1.69-3.8 3.767-3.8s3.767 1.704 3.767 3.8c0 2.095-1.69 3.8-3.766 3.8m0-6.408c-1.426 0-2.586 1.17-2.586 2.607 0 1.439 1.159 2.609 2.585 2.609s2.585-1.17 2.585-2.609c0-1.437-1.16-2.606-2.585-2.606M1.707 10.4V7.082a.858.858 0 0 0-.853-.86.856.856 0 0 0-.854.86v6.632c.002 2.244 1.81 4.065 4.03 4.065 2.221 0 4.029-1.823 4.029-4.065 0-2.24-1.808-4.063-4.029-4.063a3.98 3.98 0 0 0-2.323.748zm2.323 5.658a2.335 2.335 0 0 1-2.323-2.343 2.335 2.335 0 0 1 2.323-2.342 2.335 2.335 0 0 1 2.323 2.342 2.335 2.335 0 0 1-2.323 2.343m16.465-5.854a3.49 3.491 0 0 0-2.848 1.484V6.704a.33.33 0 0 0-.327-.331.33.33 0 0 0-.328.331v7.036c.002 1.949 1.572 3.533 3.504 3.533 1.931 0 3.504-1.586 3.504-3.536 0-1.949-1.573-3.534-3.505-3.534m0 6.408c-1.57 0-2.847-1.289-2.847-2.873s1.278-2.872 2.847-2.872c1.57 0 2.848 1.288 2.848 2.872 0 1.585-1.277 2.873-2.848 2.873"
        /> < title > { title } < / title > < / svg >
    }
}
