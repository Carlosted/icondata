#[cfg(feature = "IoFish")]
use leptos::*;
#[cfg(feature = "IoFish")]
///This icon requires the feature `IoFish` to be enabled.
#[component]
pub fn Fish(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M512,256c0-16.54-14.27-46.76-45.61-74a207.06,207.06,0,0,0-60.28-36.12,3.15,3.15,0,0,0-3.93,1.56c-.15.29-.3.57-.47.86l-9.59,15.9a183.24,183.24,0,0,0,.07,183.78l.23.39,8.74,16a4,4,0,0,0,4.94,1.82C479.63,337.42,512,281.49,512,256Zm-93.92-.14a16,16,0,1,1,13.79-13.79A16,16,0,0,1,418.08,255.86Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M335.45,256a214.8,214.8,0,0,1,29.08-108l.12-.21,4.62-7.67a4,4,0,0,0-2.59-6,284.29,284.29,0,0,0-39.26-5.39,7.94,7.94,0,0,1-4.29-1.6c-19.28-14.66-57.5-40.3-96.46-46.89a16,16,0,0,0-18,20.18l10.62,37.17a4,4,0,0,1-2.42,4.84c-36.85,13.69-68.59,38.75-91.74,57.85a8,8,0,0,1-10.06.06q-4.72-3.75-9.69-7.39C65.74,164,19.17,160.19,17.21,160.05A16,16,0,0,0,.38,179.45c.42,1.93,9.19,40.69,31.7,71.61a8.09,8.09,0,0,1,0,9.55C9.57,291.52.8,330.29.38,332.22a16,16,0,0,0,16.83,19.4c2-.14,48.53-4,88.12-32.88q4.85-3.56,9.47-7.22a8,8,0,0,1,10.06.07c23.25,19.19,55.05,44.28,92,58a4,4,0,0,1,2.42,4.83L208.62,411.6a16,16,0,0,0,18,20.18c17.16-2.9,51.88-12.86,96.05-46.83a8.15,8.15,0,0,1,4.36-1.65A287.36,287.36,0,0,0,366.25,378a4,4,0,0,0,2.69-5.83l-4.51-8.29A214.81,214.81,0,0,1,335.45,256Z"
        /> < title > { title } < / title > < / svg >
    }
}
