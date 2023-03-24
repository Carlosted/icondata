#[cfg(feature = "IoLogoDocker")]
use leptos::*;
#[cfg(feature = "IoLogoDocker")]
///This icon requires the feature `IoLogoDocker` to be enabled.
#[component]
pub fn LogoDocker(
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
        stroke_witdh = "0" style = style id = "Layer_1" data - name = "Layer 1" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M507,211.16c-1.42-1.19-14.25-10.94-41.79-10.94a132.55,132.55,0,0,0-21.61,1.9c-5.22-36.4-35.38-54-36.57-55l-7.36-4.28-4.75,6.9a101.65,101.65,0,0,0-13.06,30.45c-5,20.7-1.9,40.2,8.55,56.85-12.59,7.14-33,8.8-37.28,9H15.94A15.93,15.93,0,0,0,0,262.07,241.25,241.25,0,0,0,14.75,348.9C26.39,379.35,43.72,402,66,415.74,91.22,431.2,132.3,440,178.6,440a344.23,344.23,0,0,0,62.45-5.71,257.44,257.44,0,0,0,81.69-29.73,223.55,223.55,0,0,0,55.57-45.67c26.83-30.21,42.74-64,54.38-94h4.75c29.21,0,47.26-11.66,57.23-21.65a63.31,63.31,0,0,0,15.2-22.36l2.14-6.18Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M47.29,236.37H92.4a4,4,0,0,0,4-4h0V191.89a4,4,0,0,0-4-4H47.29a4,4,0,0,0-4,4h0v40.44a4.16,4.16,0,0,0,4,4h0"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M109.5,236.37h45.12a4,4,0,0,0,4-4h0V191.89a4,4,0,0,0-4-4H109.5a4,4,0,0,0-4,4v40.44a4.16,4.16,0,0,0,4,4"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M172.9,236.37H218a4,4,0,0,0,4-4h0V191.89a4,4,0,0,0-4-4H172.9a4,4,0,0,0-4,4h0v40.44a3.87,3.87,0,0,0,4,4h0"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M235.36,236.37h45.12a4,4,0,0,0,4-4V191.89a4,4,0,0,0-4-4H235.36a4,4,0,0,0-4,4h0v40.44a4,4,0,0,0,4,4h0"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M109.5,178.57h45.12a4.16,4.16,0,0,0,4-4V134.09a4,4,0,0,0-4-4H109.5a4,4,0,0,0-4,4v40.44a4.34,4.34,0,0,0,4,4"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M172.9,178.57H218a4.16,4.16,0,0,0,4-4V134.09a4,4,0,0,0-4-4H172.9a4,4,0,0,0-4,4h0v40.44a4,4,0,0,0,4,4"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M235.36,178.57h45.12a4.16,4.16,0,0,0,4-4V134.09a4.16,4.16,0,0,0-4-4H235.36a4,4,0,0,0-4,4h0v40.44a4.16,4.16,0,0,0,4,4"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M235.36,120.53h45.12a4,4,0,0,0,4-4V76a4.16,4.16,0,0,0-4-4H235.36a4,4,0,0,0-4,4h0v40.44a4.17,4.17,0,0,0,4,4"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M298.28,236.37H343.4a4,4,0,0,0,4-4V191.89a4,4,0,0,0-4-4H298.28a4,4,0,0,0-4,4h0v40.44a4.16,4.16,0,0,0,4,4"
        /> < title > { title } < / title > < / svg >
    }
}
