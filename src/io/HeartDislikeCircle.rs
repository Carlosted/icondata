#[cfg(feature = "IoHeartDislikeCircle")]
use leptos::*;
#[cfg(feature = "IoHeartDislikeCircle")]
///This icon requires the feature `IoHeartDislikeCircle` to be enabled.
#[component]
pub fn HeartDislikeCircle(
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
        "M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm23.3,299.19c-4.41,3.2-9.16,6.55-14.31,10a15.93,15.93,0,0,1-18,0c-39.3-26.68-56.32-45-65.7-56.41-20-24.37-29.58-49.4-29.3-76.5,0-.21,0-.43,0-.64a4,4,0,0,1,6.82-2.72L279.76,341.12A4,4,0,0,1,279.3,347.19Zm68,16.12a16,16,0,0,1-22.62,0l-176-176a16,16,0,0,1,22.62-22.62l176,176A16,16,0,0,1,347.31,363.31ZM333.2,297.69a3.92,3.92,0,0,1-6,.37l-124-123.21A4,4,0,0,1,206,168l1.55,0c20.4,0,35,10.64,44.11,20.42a5.93,5.93,0,0,0,8.7,0c9.11-9.78,23.71-20.42,44.11-20.42,30.31,0,55.22,25.27,55.53,56.33C360.26,250.26,351.48,274.3,333.2,297.69Z"
        /> < title > { title } < / title > < / svg >
    }
}
