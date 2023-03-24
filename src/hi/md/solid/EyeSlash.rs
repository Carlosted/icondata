#[cfg(feature = "HiMdSolidEyeSlash")]
use leptos::*;
#[cfg(feature = "HiMdSolidEyeSlash")]
///This icon requires the feature `HiMdSolidEyeSlash` to be enabled.
#[component]
pub fn EyeSlash(
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
        "M3.28033 2.21967C2.98744 1.92678 2.51256 1.92678 2.21967 2.21967C1.92678 2.51256 1.92678 2.98744 2.21967 3.28033L16.7197 17.7803C17.0126 18.0732 17.4874 18.0732 17.7803 17.7803C18.0732 17.4874 18.0732 17.0126 17.7803 16.7197L16.0352 14.9745C17.5064 13.8594 18.6595 12.3465 19.3344 10.5959C19.4814 10.2144 19.4816 9.79127 19.3347 9.40962C17.892 5.66051 14.256 3 9.99859 3C8.28207 3 6.66657 3.43249 5.2551 4.19444L3.28033 2.21967ZM7.75194 6.69128L8.84367 7.78301C9.18951 7.60223 9.58291 7.5 10.0002 7.5C11.3809 7.5 12.5002 8.61929 12.5002 10C12.5002 10.4173 12.398 10.8107 12.2172 11.1565L13.3091 12.2484C13.7454 11.6077 14.0004 10.8336 14.0004 10C14.0004 7.79086 12.2095 6 10.0004 6C9.16675 6 8.39268 6.25501 7.75194 6.69128Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.7484 13.9302L13.2711 16.4529C12.2462 16.8074 11.1458 17 10.0004 17C5.74298 17 2.10698 14.3395 0.664255 10.5904C0.517392 10.2087 0.517518 9.78563 0.66461 9.40408C1.15603 8.12932 1.90108 6.98057 2.83791 6.01969L6.0702 9.25198C6.02436 9.4943 6.00037 9.74435 6.00037 10C6.00037 12.2091 7.79123 14 10.0004 14C10.256 14 10.5061 13.976 10.7484 13.9302Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
