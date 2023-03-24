#[cfg(feature = "IoConstruct")]
use leptos::*;
#[cfg(feature = "IoConstruct")]
///This icon requires the feature `IoConstruct` to be enabled.
#[component]
pub fn Construct(
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
        "M503.58,126.2a16.85,16.85,0,0,0-27.07-4.55L425.36,172.8h0a11.15,11.15,0,0,1-15.66,0l-22.48-22.48a11.17,11.17,0,0,1,0-15.67L438.1,83.76a16.85,16.85,0,0,0-5.27-27.4c-39.71-17-89.08-7.45-120,23.29-26.81,26.61-34.83,68-22,113.7a11,11,0,0,1-3.16,11.1L114.77,365.1a56.76,56.76,0,1,0,80.14,80.18L357,272.08a11,11,0,0,1,10.9-3.17c45,12,86,4,112.43-22,15.2-15,25.81-36.17,29.89-59.71C514.05,165,511.63,142.76,503.58,126.2Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M437.33,378.41c-13.94-11.59-43.72-38.4-74.07-66.22L297.19,382.8c28.24,30,53.8,57.85,65,70.88l.07.08A30,30,0,0,0,383.72,464l1.1,0a30.11,30.11,0,0,0,21-8.62l.07-.07,33.43-33.37a29.46,29.46,0,0,0-2-43.53Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M118.54,214.55a20.48,20.48,0,0,0-3-10.76,2.76,2.76,0,0,1,2.62-4.22h.06c.84.09,5.33.74,11.7,4.61,4.73,2.87,18.23,12.08,41.73,35.54a34.23,34.23,0,0,0,7.22,22.12l66.23-61.55a33.73,33.73,0,0,0-21.6-9.2,2.65,2.65,0,0,1-.21-.26l-.65-.69L198.1,156.3a28.45,28.45,0,0,1-4-26.11,35.23,35.23,0,0,1,11.78-16.35c5.69-4.41,18.53-9.72,29.44-10.62a52.92,52.92,0,0,1,15.19.94,65.57,65.57,0,0,1,7.06,2.13,15.46,15.46,0,0,0,2.15.63,16,16,0,0,0,16.38-25.06c-.26-.35-1.32-1.79-2.89-3.73a91.85,91.85,0,0,0-9.6-10.36c-8.15-7.36-29.27-19.77-57-19.77a123.13,123.13,0,0,0-46.3,9C121.94,72.45,96.84,93.58,85.3,104.79l-.09.09A222.14,222.14,0,0,0,63.7,129.5,27,27,0,0,0,59,141.27a7.33,7.33,0,0,1-7.71,6.17c-.36,0-.73,0-1.09,0a20.65,20.65,0,0,0-14.59,5.9L6.16,182.05l-.32.32a20.89,20.89,0,0,0-.24,28.72c.19.2.37.39.57.58L53.67,258A21,21,0,0,0,68.32,264a20.65,20.65,0,0,0,14.59-5.9l29.46-28.79A20.51,20.51,0,0,0,118.54,214.55Z"
        /> < title > { title } < / title > < / svg >
    }
}
