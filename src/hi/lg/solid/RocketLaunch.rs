#[cfg(feature = "HiLgSolidRocketLaunch")]
use leptos::*;
#[cfg(feature = "HiLgSolidRocketLaunch")]
///This icon requires the feature `HiLgSolidRocketLaunch` to be enabled.
#[component]
pub fn RocketLaunch(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M9.315 7.58365C12.1956 3.88296 16.6946 1.50021 21.75 1.5C21.9489 1.49999 22.1397 1.57901 22.2803 1.71966C22.421 1.86031 22.5 2.05108 22.5 2.25C22.5 7.30564 20.1173 11.805 16.4165 14.6859C16.4715 15.0329 16.5 15.3883 16.5 15.75C16.5 19.4779 13.4779 22.5 9.75 22.5C9.33579 22.5 9 22.1642 9 21.75V17.6185C8.99075 17.6118 8.98163 17.6049 8.97264 17.5978C8.02063 16.8429 7.15799 15.9803 6.40312 15.0282C6.39577 15.019 6.38866 15.0096 6.38179 15H2.25C1.83579 15 1.5 14.6642 1.5 14.25C1.5 10.5221 4.52208 7.5 8.25 7.5C8.61198 7.5 8.96772 7.52856 9.315 7.58365ZM15 6.75C13.7574 6.75 12.75 7.75736 12.75 9C12.75 10.2426 13.7574 11.25 15 11.25C16.2426 11.25 17.25 10.2426 17.25 9C17.25 7.75736 16.2426 6.75 15 6.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.26036 17.2418C5.59237 16.9942 5.66074 16.5242 5.41306 16.1922C5.16539 15.8602 4.69546 15.7918 4.36345 16.0395C3.08209 16.9954 2.25 18.5256 2.25 20.2499C2.25 20.5255 2.27129 20.7966 2.31246 21.0615C2.36259 21.3842 2.61574 21.6373 2.93842 21.6875C3.20336 21.7286 3.47445 21.7499 3.75 21.7499C5.47434 21.7499 7.00452 20.9178 7.9604 19.6365C8.20808 19.3045 8.13971 18.8345 7.8077 18.5869C7.47569 18.3392 7.00576 18.4075 6.75809 18.7396C6.07313 19.6577 4.98081 20.2499 3.75 20.2499C3.75 19.0191 4.34218 17.9268 5.26036 17.2418Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
