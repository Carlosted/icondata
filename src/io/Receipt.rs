#[cfg(feature = "IoReceipt")]
use leptos::*;
#[cfg(feature = "IoReceipt")]
///This icon requires the feature `IoReceipt` to be enabled.
#[component]
pub fn Receipt(
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
        "M483.82,32.45a16.28,16.28,0,0,0-11.23,1.37L448,46.1h0l-24.8-12.4a16,16,0,0,0-14.31,0L383.78,46.11h0L359,33.7a16,16,0,0,0-14.36,0L320,46.07H320L295.55,33.73a16,16,0,0,0-14.35-.06L256,46.12h0l-24.8-12.43a16.05,16.05,0,0,0-14.33,0L192,46.1h0L167.16,33.69a16,16,0,0,0-19.36,3.94A16.25,16.25,0,0,0,144,48.28V288a0,0,0,0,0,.05.05H336a32,32,0,0,1,32,32V424c0,30.93,33.07,56,64,56h12a52,52,0,0,0,52-52V48A16,16,0,0,0,483.82,32.45ZM416,240H288.5c-8.64,0-16.1-6.64-16.48-15.28A16,16,0,0,1,288,208H415.5c8.64,0,16.1,6.64,16.48,15.28A16,16,0,0,1,416,240Zm0-80H224.5c-8.64,0-16.1-6.64-16.48-15.28A16,16,0,0,1,224,128H415.5c8.64,0,16.1,6.64,16.48,15.28A16,16,0,0,1,416,160Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M336,424V336a16,16,0,0,0-16-16H48a32.1,32.1,0,0,0-32,32.05c0,50.55,5.78,71.57,14.46,87.57C45.19,466.79,71.86,480,112,480H357.68a4,4,0,0,0,2.85-6.81C351.07,463.7,336,451,336,424Z"
        /> < title > { title } < / title > < / svg >
    }
}
