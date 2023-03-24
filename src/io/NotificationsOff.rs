#[cfg(feature = "IoNotificationsOff")]
use leptos::*;
#[cfg(feature = "IoNotificationsOff")]
///This icon requires the feature `IoNotificationsOff` to be enabled.
#[component]
pub fn NotificationsOff(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M448,464a15.92,15.92,0,0,1-11.31-4.69l-384-384A16,16,0,0,1,75.31,52.69l384,384A16,16,0,0,1,448,464Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M440.08,341.31c-1.66-2-3.29-4-4.89-5.93-22-26.61-35.31-42.67-35.31-118,0-39-9.33-71-27.72-95-13.56-17.73-31.89-31.18-56.05-41.12a3,3,0,0,1-.82-.67C306.6,51.49,282.82,32,256,32s-50.59,19.49-59.28,48.56a3.13,3.13,0,0,1-.81.65,157.88,157.88,0,0,0-21.88,11,8,8,0,0,0-1.49,12.49L434.32,366.44a8,8,0,0,0,13.6-6.63A35.39,35.39,0,0,0,440.08,341.31Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M112.14,217.35c0,75.36-13.29,91.42-35.31,118-1.6,1.93-3.23,3.89-4.89,5.93a35.16,35.16,0,0,0-4.65,37.62c6.17,13,19.32,21.07,34.33,21.07H312.8a8,8,0,0,0,5.66-13.66l-192-192a8,8,0,0,0-13.62,5Q112.14,208,112.14,217.35Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,480a80.06,80.06,0,0,0,70.44-42.13A4,4,0,0,0,322.9,432H189.12a4,4,0,0,0-3.55,5.87A80.06,80.06,0,0,0,256,480Z"
        /> < title > { title } < / title > < / svg >
    }
}
