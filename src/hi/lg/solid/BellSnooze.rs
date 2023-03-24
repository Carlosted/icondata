#[cfg(feature = "HiLgSolidBellSnooze")]
use leptos::*;
#[cfg(feature = "HiLgSolidBellSnooze")]
///This icon requires the feature `HiLgSolidBellSnooze` to be enabled.
#[component]
pub fn BellSnooze(
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
        "M12 2.25C8.27216 2.25 5.25012 5.27197 5.25001 8.9998L5.24981 9.75C5.24981 11.8731 4.44879 13.8074 3.13126 15.2699C2.96476 15.4547 2.90073 15.71 2.96033 15.9516C3.01992 16.1931 3.19539 16.3893 3.42875 16.4755C4.97287 17.0455 6.58934 17.4659 8.2604 17.7192C8.25351 17.812 8.25001 17.9056 8.25001 18C8.25001 20.0711 9.92894 21.75 12 21.75C14.0711 21.75 15.75 20.0711 15.75 18C15.75 17.9056 15.7465 17.812 15.7396 17.7192C17.4105 17.4659 19.0269 17.0455 20.5709 16.4755C20.8042 16.3893 20.9797 16.1931 21.0393 15.9516C21.0989 15.71 21.0349 15.4547 20.8684 15.2699C19.5508 13.8074 18.7498 11.8731 18.7498 9.75V9.04919L18.75 9C18.75 5.27208 15.7279 2.25 12 2.25ZM9.75001 18C9.75001 17.9662 9.75075 17.9326 9.75221 17.8993C10.4927 17.966 11.2424 18 11.9998 18C12.7574 18 13.5072 17.9659 14.2478 17.8992C14.2493 17.9326 14.25 17.9662 14.25 18C14.25 19.2426 13.2427 20.25 12 20.25C10.7574 20.25 9.75001 19.2426 9.75001 18ZM10.5 7.5C10.0858 7.5 9.75 7.83579 9.75 8.25C9.75 8.66421 10.0858 9 10.5 9H12.0986L9.87596 12.334C9.72253 12.5641 9.70823 12.86 9.83874 13.1039C9.96926 13.3478 10.2234 13.5 10.5 13.5H13.5C13.9142 13.5 14.25 13.1642 14.25 12.75C14.25 12.3358 13.9142 12 13.5 12H11.9014L14.124 8.66602C14.2775 8.43588 14.2918 8.13997 14.1613 7.89611C14.0307 7.65224 13.7766 7.5 13.5 7.5H10.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
