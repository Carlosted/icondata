#[cfg(feature = "RiLogosFillSteam")]
use leptos::*;
#[cfg(feature = "RiLogosFillSteam")]
///This icon requires the feature `RiLogosFillSteam` to be enabled.
#[component]
pub fn Steam(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M12.004 2c-5.25 0-9.556 4.05-9.964 9.197l5.36 2.216c.454-.31 1.002-.492 1.593-.492.053 0 .104.003.157.005l2.384-3.452v-.049c0-2.08 1.69-3.77 3.77-3.77 2.079 0 3.77 1.692 3.77 3.772s-1.692 3.771-3.77 3.771h-.087l-3.397 2.426c0 .043.003.088.003.133 0 1.562-1.262 2.83-2.825 2.83-1.362 0-2.513-.978-2.775-2.273l-3.838-1.589C3.573 18.922 7.427 22 12.005 22c5.522 0 9.998-4.477 9.998-10 0-5.522-4.477-10-9.999-10zM7.078 16.667c.218.452.595.832 1.094 1.041 1.081.45 2.328-.063 2.777-1.145.22-.525.22-1.1.004-1.625-.215-.525-.625-.934-1.147-1.152-.52-.217-1.075-.208-1.565-.025l1.269.525c.797.333 1.174 1.25.84 2.046-.33.797-1.247 1.175-2.044.843l-1.228-.508zm10.74-7.245c0-1.385-1.128-2.512-2.513-2.512-1.387 0-2.512 1.127-2.512 2.512 0 1.388 1.125 2.513 2.512 2.513 1.386 0 2.512-1.125 2.512-2.513zM15.31 7.53c1.04 0 1.888.845 1.888 1.888s-.847 1.888-1.888 1.888c-1.044 0-1.888-.845-1.888-1.888s.845-1.888 1.888-1.888z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
