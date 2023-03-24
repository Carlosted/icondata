#[cfg(feature = "HiMdSolidMegaphone")]
use leptos::*;
#[cfg(feature = "HiMdSolidMegaphone")]
///This icon requires the feature `HiMdSolidMegaphone` to be enabled.
#[component]
pub fn Megaphone(
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
        "M13.9202 3.84536C11.9859 4.84219 9.86368 5.52477 7.62054 5.82595C6.76376 5.94099 5.88902 6.00044 5 6.00044C2.79086 6.00044 1 7.7913 1 10.0004C1 12.0389 2.52477 13.7211 4.49597 13.969C4.78782 15.1067 5.20979 16.2273 5.76704 17.3103C6.1636 18.0809 7.10902 18.3091 7.81763 17.9L8.68366 17.4C9.41014 16.9805 9.62418 16.0789 9.27228 15.3786C9.10619 15.0482 8.95684 14.7133 8.82394 14.3751C10.6243 14.733 12.3353 15.3388 13.9201 16.1555C14.6189 14.2352 15 12.1623 15 10.0004C15 7.83855 14.6189 5.76569 13.9202 3.84536Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.2428 3.09699C16.0553 5.24298 16.5 7.56977 16.5 10.0004C16.5 12.4311 16.0553 14.7579 15.2428 16.9039C15.2428 16.9039 15.2427 16.9038 15.2428 16.9039L15.2135 16.9811C15.0652 17.3679 15.2585 17.8016 15.6452 17.95C16.032 18.0983 16.4657 17.905 16.6141 17.5183C16.7002 17.2938 16.7825 17.0674 16.8611 16.8393C17.4152 15.2298 17.7791 13.532 17.9262 11.7733C18.5645 11.4393 19 10.7712 19 10.0004C19 9.22971 18.5645 8.56163 17.9262 8.22758C17.7791 6.46885 17.4152 4.77112 16.8611 3.16156C16.7825 2.93344 16.7002 2.7071 16.6141 2.48262C16.4657 2.09587 16.032 1.90259 15.6452 2.05091C15.2585 2.19924 15.0652 2.633 15.2135 3.01974L15.2428 3.09699C15.2427 3.09702 15.2428 3.09695 15.2428 3.09699Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
