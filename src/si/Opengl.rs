#[cfg(feature = "SiOpengl")]
use leptos::*;
#[cfg(feature = "SiOpengl")]
///This icon requires the feature `SiOpengl` to be enabled.
#[component]
pub fn Opengl(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M7.921 11.382v.376h.009a.696.696 0 0 1 .362-.336c.165-.07.346-.105.543-.105.219 0 .411.039.574.118.163.079.298.185.406.319a1.4 1.4 0 0 1 .244.464c.055.175.082.361.082.558 0 .197-.027.383-.08.558a1.325 1.325 0 0 1-.241.459 1.126 1.126 0 0 1-.406.308 1.345 1.345 0 0 1-.568.113 1.457 1.457 0 0 1-.488-.091.984.984 0 0 1-.239-.132.722.722 0 0 1-.189-.207h-.009v1.432H7.45v-3.835h.471zm1.671.986a1.01 1.01 0 0 0-.159-.33.78.78 0 0 0-.274-.231.849.849 0 0 0-.392-.086c-.157 0-.29.03-.4.091a.783.783 0 0 0-.268.239.992.992 0 0 0-.151.335 1.577 1.577 0 0 0 .003.775.966.966 0 0 0 .156.335.785.785 0 0 0 .276.233c.113.059.25.088.411.088.161 0 .295-.03.402-.091a.744.744 0 0 0 .26-.241c.066-.1.113-.214.142-.343.029-.129.044-.261.044-.397a1.343 1.343 0 0 0-.05-.377zm2.951 1.611c-.213.157-.48.236-.803.236a1.5 1.5 0 0 1-.591-.107 1.17 1.17 0 0 1-.421-.301 1.272 1.272 0 0 1-.256-.461 2.157 2.157 0 0 1-.096-.585c0-.211.033-.404.099-.579.066-.175.159-.327.278-.456a1.25 1.25 0 0 1 .424-.3c.163-.072.342-.107.537-.107.253 0 .463.051.63.153.167.102.301.232.402.39.101.158.171.33.209.516.039.186.054.364.047.532h-2.127c-.004.121.011.237.044.345a.779.779 0 0 0 .159.289.778.778 0 0 0 .28.201c.113.05.247.075.401.075a.826.826 0 0 0 .486-.134.654.654 0 0 0 .25-.408h.462c-.064.31-.201.544-.414.701zm-.114-1.78a.792.792 0 0 0-.743-.477.827.827 0 0 0-.326.062.737.737 0 0 0-.249.169.81.81 0 0 0-.164.249.926.926 0 0 0-.071.302h1.628a.93.93 0 0 0-.075-.305zm1.327-.817v.44h.009c.195-.337.504-.505.928-.505.188 0 .344.025.469.075.125.05.226.12.304.21.077.09.132.196.163.32s.047.261.047.411v1.827h-.471v-1.879a.546.546 0 0 0-.154-.408.582.582 0 0 0-.424-.15.978.978 0 0 0-.372.065.696.696 0 0 0-.262.183.785.785 0 0 0-.157.276 1.096 1.096 0 0 0-.052.346v1.568h-.471v-2.777h.443zm5.174 2.747a1.67 1.67 0 0 1-.644.131c-.342 0-.649-.058-.922-.174a1.976 1.976 0 0 1-.691-.48 2.112 2.112 0 0 1-.431-.719c-.1-.275-.15-.572-.15-.89 0-.326.05-.629.15-.909.1-.279.243-.523.43-.731.187-.208.417-.371.69-.49a2.3 2.3 0 0 1 .922-.177c.229 0 .451.034.665.101.215.068.408.167.581.297a1.6 1.6 0 0 1 .634 1.144h-.937c-.058-.244-.171-.427-.338-.55a1 1 0 0 0-.606-.183c-.221 0-.408.042-.563.125s-.279.196-.375.337a1.444 1.444 0 0 0-.209.48 2.327 2.327 0 0 0 0 1.092c.044.173.114.329.21.468.096.139.221.25.375.333.154.084.342.125.563.125.325 0 .577-.08.754-.241.177-.16.281-.393.31-.698h-.987v-.717h1.872v2.358h-.623l-.1-.495a1.44 1.44 0 0 1-.58.463zM21.825 9.8v3.55H24v.809h-3.154V9.8h.979zM3.801 13.98c.053.03.107.059.164.085.267.124.578.186.933.186.355 0 .666-.062.933-.186s.491-.292.67-.503c.179-.211.314-.454.404-.728.09-.274.135-.56.135-.856 0-.297-.045-.582-.135-.856a2.135 2.135 0 0 0-.404-.728 1.966 1.966 0 0 0-.67-.506 2.17 2.17 0 0 0-.933-.189c-.355 0-.666.063-.933.189l-.03.015c1.425-1.199 4.034-2.001 7.017-2.001 2.512 0 4.765.516 6.263 1.412-1.635-1.501-4.566-2.555-7.918-2.556C4.162 6.757 0 9.103 0 11.999c0 2.895 4.161 5.243 9.294 5.244 3.338.001 6.262-1.051 7.901-2.541-1.498.89-3.741 1.397-6.244 1.397-3.078-.001-5.759-.856-7.15-2.119zm.395-3.638c.196-.104.43-.156.702-.156.272 0 .506.052.702.156.196.104.357.241.483.412.125.171.217.363.276.577a2.43 2.43 0 0 1 0 1.3c-.059.214-.15.406-.276.576a1.393 1.393 0 0 1-.483.412c-.197.104-.43.155-.702.155a1.49 1.49 0 0 1-.702-.155 1.402 1.402 0 0 1-.483-.412 1.765 1.765 0 0 1-.276-.576 2.43 2.43 0 0 1 0-1.3 1.74 1.74 0 0 1 .276-.577c.125-.171.286-.308.483-.412z"
        /> < title > { title } < / title > < / svg >
    }
}
