#[cfg(feature = "IoFootsteps")]
use leptos::*;
#[cfg(feature = "IoFootsteps")]
///This icon requires the feature `IoFootsteps` to be enabled.
#[component]
pub fn Footsteps(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M133.83,361.27c-22.61,0-41-8.17-54.79-24.39S56.2,296.59,50.93,261.57c-7.76-51.61-.06-95.11,21.68-122.48,12.8-16.12,29.6-25.44,48.58-26.94,16.25-1.3,40.54,5.29,64,44,14.69,24.24,25.86,56.44,30.65,88.34h0c5.79,38.51,1.48,66.86-13.18,86.65-11.64,15.72-29.54,25.46-53.21,29A106.46,106.46,0,0,1,133.83,361.27Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M173,496c-13.21,0-26.6-4.23-38.66-12.36a79.79,79.79,0,0,1-33.52-50.6c-2.85-14.66-1.14-26.31,5.22-35.64,10.33-15.15,28.87-18.56,48.49-22.18,2.07-.38,4.17-.76,6.3-1.17,4.52-.86,9.14-2,13.62-3.11,16.78-4.14,34.14-8.43,48.47,1.75,9.59,6.8,15,18.36,16.62,35.32h0c1.84,19.57-2.36,39.1-11.83,55-10.19,17.11-25.47,28.42-43,31.86A61,61,0,0,1,173,496Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M378.17,265.27a106.69,106.69,0,0,1-15.6-1.2c-23.66-3.5-41.56-13.25-53.2-29-14.66-19.79-19-48.13-13.18-86.65,4.79-31.93,15.93-64.1,30.55-88.25,23.34-38.57,47.66-45.26,64-44.08,18.92,1.38,35.69,10.57,48.51,26.6,21.89,27.37,29.65,71,21.86,122.84-5.27,35-14.2,58.95-28.11,75.31S400.78,265.27,378.17,265.27Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M339,400a61,61,0,0,1-11.68-1.13c-17.56-3.44-32.84-14.75-43-31.86-9.47-15.9-13.67-35.43-11.83-55h0c1.6-17,7-28.52,16.62-35.33,14.33-10.17,31.69-5.89,48.47-1.74,4.48,1.1,9.1,2.24,13.62,3.11l6.29,1.17c19.63,3.61,38.17,7,48.5,22.17,6.36,9.33,8.07,21,5.22,35.64a79.78,79.78,0,0,1-33.52,50.61C365.56,395.78,352.17,400,339,400Z"
        /> < title > { title } < / title > < / svg >
    }
}
