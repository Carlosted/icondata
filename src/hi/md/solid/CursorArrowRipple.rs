#[cfg(feature = "HiMdSolidCursorArrowRipple")]
use leptos::*;
#[cfg(feature = "HiMdSolidCursorArrowRipple")]
///This icon requires the feature `HiMdSolidCursorArrowRipple` to be enabled.
#[component]
pub fn CursorArrowRipple(
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
        "M6.11097 11.8891C3.96309 9.7412 3.96309 6.2588 6.11097 4.11091C8.25886 1.96303 11.7413 1.96303 13.8891 4.11091C14.9632 5.185 15.5001 6.59127 15.5001 8C15.5001 8.41421 15.8358 8.75 16.2501 8.75C16.6643 8.75 17.0001 8.41421 17.0001 8C17.0001 6.2097 16.3165 4.41694 14.9498 3.05025C12.2161 0.316583 7.78398 0.316582 5.05031 3.05025C2.31664 5.78392 2.31664 10.2161 5.05031 12.9497C5.34321 13.2426 5.81808 13.2426 6.11097 12.9497C6.40387 12.6569 6.40387 12.182 6.11097 11.8891ZM8.23233 6.23223C7.25602 7.20854 7.25602 8.79146 8.23233 9.76777C8.52522 10.0607 8.52522 10.5355 8.23233 10.8284C7.93944 11.1213 7.46456 11.1213 7.17167 10.8284C5.60957 9.26633 5.60957 6.73367 7.17167 5.17157C8.73377 3.60948 11.2664 3.60948 12.8285 5.17157C13.6094 5.95247 14.0001 6.97747 14.0001 8C14.0001 8.41421 13.6643 8.75 13.2501 8.75C12.8359 8.75 12.5001 8.41421 12.5001 8C12.5001 7.35903 12.2562 6.72054 11.7679 6.23223C10.7916 5.25592 9.20864 5.25592 8.23233 6.23223ZM10.7657 7.51062C10.5871 7.24492 10.2596 7.12184 9.95027 7.20417C9.64092 7.2865 9.41793 7.5561 9.39508 7.8754L8.90407 14.7363C8.88301 15.0306 9.03648 15.3099 9.2962 15.4499C9.55592 15.59 9.87363 15.5647 10.108 15.3854L11.1508 14.5873L12.1363 18.2653C12.2435 18.6654 12.6548 18.9028 13.0549 18.7956C13.455 18.6884 13.6924 18.2771 13.5852 17.877L12.6083 14.2312L13.9005 14.4349C14.1951 14.4814 14.4892 14.3489 14.6496 14.0974C14.81 13.846 14.8062 13.5233 14.6398 13.2758L10.7657 7.51062Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
