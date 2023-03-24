#[cfg(feature = "HiLgSolidHandThumbDown")]
use leptos::*;
#[cfg(feature = "HiLgSolidHandThumbDown")]
///This icon requires the feature `HiLgSolidHandThumbDown` to be enabled.
#[component]
pub fn HandThumbDown(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.7303 5.25L16.7647 5.25C17.5455 6.43343 18 7.85114 18 9.375C18 10.8989 17.5455 12.3166 16.7647 13.5L16.6174 13.5C15.8111 13.5 15.0835 13.9458 14.5859 14.5803C13.8127 15.5662 12.8383 16.3866 11.7245 16.9798C11.0023 17.3644 10.3757 17.9357 10.0719 18.6954C9.85923 19.2269 9.75 19.7941 9.75 20.3666L9.75 21C9.75 21.4142 9.41421 21.75 9 21.75C7.75736 21.75 6.75 20.7426 6.75 19.5C6.75 18.3484 7.00956 17.2574 7.47337 16.2823C7.73895 15.724 7.36638 15 6.74809 15L3.62227 15C2.59563 15 1.6767 14.306 1.56801 13.2851C1.52305 12.8629 1.5 12.4341 1.5 12C1.5 9.15238 2.49188 6.53642 4.149 4.47878C4.5366 3.99749 5.13581 3.75 5.75377 3.75L9.76975 3.75C10.2534 3.75 10.7339 3.82798 11.1928 3.98093L14.3072 5.01908C14.7661 5.17203 15.2466 5.25 15.7303 5.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21.6685 13.7729C22.2052 12.4111 22.5 10.9275 22.5 9.37501C22.5 8.15493 22.3179 6.97738 21.9794 5.86805C21.7201 5.01802 20.8958 4.5 20.0071 4.5L19.0993 4.5C18.6538 4.5 18.3786 4.99827 18.5758 5.3978C19.1675 6.59709 19.5 7.94722 19.5 9.375C19.5 11.0832 19.0241 12.6802 18.1977 14.0408C17.9527 14.4441 18.226 15 18.6979 15L19.7506 15C20.5827 15 21.3635 14.547 21.6685 13.7729Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
