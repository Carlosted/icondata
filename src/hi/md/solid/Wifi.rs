#[cfg(feature = "HiMdSolidWifi")]
use leptos::*;
#[cfg(feature = "HiMdSolidWifi")]
///This icon requires the feature `HiMdSolidWifi` to be enabled.
#[component]
pub fn Wifi(
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
        "M0.675986 6.94117C3.03728 4.51116 6.34284 3 10 3C13.6572 3 16.9628 4.51116 19.3241 6.94117C19.6098 7.23522 19.6064 7.70424 19.3165 7.99417L18.9629 8.34776C18.8209 8.48979 18.6278 8.56891 18.427 8.56741C18.2261 8.56591 18.0343 8.48391 17.8944 8.33977C15.8944 6.27907 13.0973 5 10 5C6.90281 5 4.10564 6.27907 2.10566 8.33977C1.96577 8.48391 1.77392 8.56591 1.57307 8.56741C1.37222 8.56891 1.17916 8.48979 1.03713 8.34776L0.683537 7.99417C0.39361 7.70424 0.390247 7.23522 0.675986 6.94117ZM3.50123 9.77378C5.13848 8.06527 7.44548 7 10 7C12.5546 7 14.8616 8.06527 16.4988 9.77378C16.7811 10.0684 16.7762 10.5345 16.4877 10.823L16.134 11.1767C15.9913 11.3194 15.797 11.3986 15.5952 11.3963C15.3933 11.394 15.2009 11.3104 15.0615 11.1645C13.786 9.82985 11.9906 9 10 9C8.00951 9 6.21406 9.82985 4.93858 11.1645C4.79912 11.3104 4.60672 11.394 4.40487 11.3963C4.20302 11.3986 4.00877 11.3194 3.86604 11.1767L3.5124 10.823C3.22391 10.5345 3.21894 10.0684 3.50123 9.77378ZM6.32061 12.6144C7.23335 11.6229 8.5445 11 10 11C11.4556 11 12.7667 11.6229 13.6794 12.6144C13.9518 12.9103 13.9423 13.3683 13.658 13.6527L13.3041 14.0066C13.1591 14.1516 12.9611 14.2309 12.7562 14.2261C12.5512 14.2213 12.3572 14.1328 12.2192 13.9812C11.6694 13.3773 10.8793 13 10 13C9.12075 13 8.33066 13.3773 7.7809 13.9812C7.64289 14.1328 7.44884 14.2213 7.24388 14.2261C7.03893 14.2309 6.84094 14.1516 6.69598 14.0066L6.34208 13.6527C6.05771 13.3683 6.04824 12.9103 6.32061 12.6144ZM9.11615 15.3661C9.34153 15.1407 9.65506 15 10 15C10.345 15 10.6585 15.1407 10.8839 15.3661C11.1768 15.659 11.1768 16.1339 10.8839 16.4268L10.5304 16.7803C10.2375 17.0732 9.76259 17.0732 9.4697 16.7803L9.11615 16.4268C8.82325 16.1339 8.82325 15.659 9.11615 15.3661Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
