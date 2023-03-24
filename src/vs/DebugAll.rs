#[cfg(feature = "VsDebugAll")]
use leptos::*;
#[cfg(feature = "VsDebugAll")]
///This icon requires the feature `VsDebugAll` to be enabled.
#[component]
pub fn DebugAll(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.29333 9.00631L6.41333 9.88552C6.27949 9.34717 5.96917 8.86905 5.53181 8.52735C5.09445 8.18564 4.55521 8 4 8C3.44479 8 2.90555 8.18564 2.46819 8.52735C2.03083 8.86905 1.72051 9.34717 1.58667 9.88552L0.706667 9.00631L0 9.71234L1.14667 10.858L1 11.0045V12.0036H0V13.0027H1V13.056C1.051 13.3815 1.14283 13.6993 1.27333 14.0018L0 15.294L0.706667 16L1.80667 14.901C2.06838 15.2346 2.40078 15.5062 2.78001 15.6962C3.15924 15.8862 3.57587 15.99 4 16C4.42413 15.99 4.84076 15.8862 5.21999 15.6962C5.59922 15.5062 5.93162 15.2346 6.19333 14.901L7.29333 16L8 15.294L6.72667 14.0018C6.85879 13.6929 6.95065 13.3683 7 13.036V12.9694H8V12.0036H7V11.0045L6.85333 10.858L8 9.71234L7.29333 9.00631ZM4 9.00631C4.39782 9.00631 4.77936 9.16421 5.06066 9.44526C5.34196 9.72631 5.5 10.1075 5.5 10.505H2.5C2.5 10.1075 2.65804 9.72631 2.93934 9.44526C3.22064 9.16421 3.60218 9.00631 4 9.00631ZM6 13.0027C5.95116 13.5161 5.72476 13.9965 5.35974 14.3612C4.99472 14.7259 4.5139 14.9521 4 15.0009C3.4861 14.9521 3.00528 14.7259 2.64026 14.3612C2.27524 13.9965 2.04884 13.5161 2 13.0027V11.5041H6V13.0027Z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule
        = "evenodd" d =
        "M3.77951 2L2.99951 2.41V7H3.99951V3.35L11.5995 8.42L9 10.1507V11.3497L12.7795 8.83V8L3.77951 2ZM9 13.3497V12.1482L14.5995 8.42006L6.99951 3.35006V2.14673L15.7795 8.00006V8.83006L9 13.3497Z"
        /> < title > { title } < / title > < / svg >
    }
}
