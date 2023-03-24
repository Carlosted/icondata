#[cfg(feature = "FaBrandsWatchmanMonitoring")]
use leptos::*;
#[cfg(feature = "FaBrandsWatchmanMonitoring")]
///This icon requires the feature `FaBrandsWatchmanMonitoring` to be enabled.
#[component]
pub fn WatchmanMonitoring(
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
        "M256,16C123.452,16,16,123.452,16,256S123.452,496,256,496,496,388.548,496,256,388.548,16,256,16ZM121.69,429.122C70.056,388.972,36.741,326.322,36.741,256a218.519,218.519,0,0,1,9.587-64.122l102.9-17.895-.121,10.967-13.943,2.013s-.144,12.5-.144,19.549a12.778,12.778,0,0,0,4.887,10.349l9.468,7.4Zm105.692-283.27,8.48-7.618s6.934-5.38-.143-9.344c-7.188-4.024-39.53-34.5-39.53-34.5-5.348-5.477-8.257-7.347-15.46,0,0,0-32.342,30.474-39.529,34.5-7.078,3.964-.144,9.344-.144,9.344l8.481,7.618-.048,4.369L75.982,131.045c39.644-56.938,105.532-94.3,180.018-94.3A218.754,218.754,0,0,1,420.934,111.77l-193.512,37.7Zm34.063,329.269-33.9-250.857,9.467-7.4a12.778,12.778,0,0,0,4.888-10.349c0-7.044-.144-19.549-.144-19.549l-13.943-2.013-.116-10.474,241.711,31.391A218.872,218.872,0,0,1,475.259,256C475.259,375.074,379.831,472.212,261.445,475.121Z"
        /> < title > { title } < / title > < / svg >
    }
}
