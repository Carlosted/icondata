#[cfg(feature = "SiRsocket")]
use leptos::*;
#[cfg(feature = "SiRsocket")]
///This icon requires the feature `SiRsocket` to be enabled.
#[component]
pub fn Rsocket(
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
        "M22.0017.0027a10.519 10.519 0 0 1-.707.0254c-2.729-.0032-5.4588.0152-8.1873-.0235-1.3307-.0188-2.5864.2591-3.8065.7383-1.888.7414-3.491 1.8872-4.8105 3.4335C3.3771 5.4811 2.6038 6.9602 2.1 8.5981c-.351 1.1412-.4421 2.316-.457 3.4843-.0491 3.8519-.03 7.704-.045 11.5563-.001.272.12.3397.3594.3399a747.047 747.047 0 0 1 3.8808.0156c.2057.0013.3034-.0698.3027-.2988-.0065-2.0774-.004-4.1549-.004-6.2323h.004c0-1.6147-.0051-3.2289.002-4.8436.0032-.7354.0408-1.4628.2011-2.1913.2935-1.3334.8763-2.4931 1.8007-3.5019 1.4113-1.5399 3.095-2.3874 5.2264-2.4062 2.7385-.024 5.4765-.017 8.2147-.0097.758.002.7573.0062.7597-.748.0038-1.1425.0089-2.2853.0137-3.4277.001-.2785-.081-.3503-.3574-.332zm.0722 6.4686a8.0464 8.0464 0 0 1-.5097.0175c-2.7098.0032-5.419.0064-8.1287.0078-.4452.0003-.899.0172-1.3222.1348-2.3604.6557-3.8665 2.6802-3.9921 4.9784-.1171 2.1443.8097 3.7936 2.5624 4.992.639.437 1.3534.7418 2.121.8027 1.5251.121 3.059.0446 4.5878.084.19.005.2812-.0765.2656-.2734-.007-.089-.0564-.2059.0528-.254.1102-.0484.1893.0542.2695.1153.5025.3832.9995.775 1.5078 1.1504.2049.1513.162.2592-.0058.3945-.4774.3848-.9511.776-1.4258 1.164-.0732.0599-.145.1317-.25.0977-.1193-.0386-.1036-.1468-.0976-.2383.0159-.243-.1108-.3032-.332-.3027-2.9551.0058-5.91.0073-8.865.0058-.4322-.0002-.4198-.0086-.416.412.0117 1.3027.0395 2.6042-.0176 3.9062-.0114.2586.094.3345.3281.334 1.5294-.0033 3.0584-.002 4.5878-.002v-.0332c.9158 0 1.8322.0067 2.748-.002.7262-.0069 1.4622.0436 2.1757-.1055 1.4784-.3089 2.6552-1.106 3.5057-2.3515 1.0026-1.4681 1.1873-3.0805.6836-4.7596-.3397-1.1324-1.0219-2.0666-1.9824-2.7577-.8843-.6363-1.8522-1.0606-2.9862-1.0644-1.1798-.004-2.3591-.0068-3.539-.0235-.2507-.0035-.2778.1322-.25.33.0116.0822.027.1789-.0644.2286-.0936.051-.1617-.0238-.2246-.0762-.515-.4283-1.0257-.8616-1.543-1.287-.1355-.1115-.1546-.2131-.0098-.3164.5468-.39 1.0515-.829 1.543-1.2852.0603-.056.1264-.1333.2207-.0761.0785.0475.051.1323.0449.207-.0266.3245.025.3792.3515.3808.6797.0034 1.3583-.03 2.037-.037 2.1146-.0223 4.2309-.0069 6.3456 0 .2361.0007.3523-.079.3516-.3243-.004-1.2932-.0035-2.5875-.0058-3.8807-.0004-.213-.1099-.3068-.3223-.293z"
        /> < title > { title } < / title > < / svg >
    }
}
