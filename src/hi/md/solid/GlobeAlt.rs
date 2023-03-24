#[cfg(feature = "HiMdSolidGlobeAlt")]
use leptos::*;
#[cfg(feature = "HiMdSolidGlobeAlt")]
///This icon requires the feature `HiMdSolidGlobeAlt` to be enabled.
#[component]
pub fn GlobeAlt(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.5546 5.41215C15.6845 4.17138 14.4711 3.18897 13.0522 2.60288C13.8305 3.9731 14.3991 5.47789 14.7147 7.07367C15.4062 6.61183 16.0263 6.05128 16.5546 5.41215Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.3257 7.82478C12.9801 5.69142 12.1346 3.72563 10.9132 2.05155C10.6135 2.0175 10.3088 2 10 2C9.69121 2 9.3865 2.0175 9.08682 2.05155C7.86543 3.72563 7.0199 5.69141 6.67433 7.82478C7.69581 8.25948 8.81982 8.5 10 8.5C11.1802 8.5 12.3042 8.25948 13.3257 7.82478Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.51418 9.37568C7.59957 9.77938 8.77402 10 10 10C11.226 10 12.4004 9.77938 13.4858 9.37568C13.4952 9.58261 13.5 9.79075 13.5 10C13.5 11.079 13.3734 12.1284 13.1343 13.1343C12.1284 13.3734 11.079 13.5 10 13.5C8.92099 13.5 7.87155 13.3734 6.86572 13.1343C6.62659 12.1284 6.5 11.079 6.5 10C6.5 9.79075 6.50476 9.58261 6.51418 9.37568Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.28529 7.07367C5.60086 5.47789 6.16954 3.9731 6.94776 2.60288C5.52894 3.18896 4.3155 4.17138 3.44542 5.41215C3.97374 6.05128 4.59375 6.61183 5.28529 7.07367Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.3336 6.79843C17.7622 7.77878 18 8.86162 18 10C18 10.3088 17.9825 10.6135 17.9484 10.9132C16.9787 11.6207 15.911 12.2021 14.7696 12.6333C14.921 11.7783 15 10.8984 15 10C15 9.5601 14.9811 9.12463 14.944 8.69435C15.8352 8.18645 16.6408 7.54546 17.3336 6.79843Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.66636 6.79843C3.3592 7.54546 4.16477 8.18645 5.05604 8.69435C5.01894 9.12463 5 9.5601 5 10C5 10.8984 5.07898 11.7783 5.23037 12.6333C4.08897 12.2021 3.02132 11.6207 2.05155 10.9132C2.0175 10.6135 2 10.3088 2 10C2 8.86162 2.23777 7.77878 2.66636 6.79843Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 15C10.8984 15 11.7783 14.921 12.6333 14.7696C12.2021 15.911 11.6207 16.9787 10.9132 17.9485C10.6135 17.9825 10.3088 18 10 18C9.69121 18 9.3865 17.9825 9.08682 17.9485C8.37929 16.9787 7.79789 15.911 7.36674 14.7696C8.22167 14.921 9.10161 15 10 15Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.3573 14.3573C14.0334 15.4259 13.5935 16.4441 13.0522 17.3971C15.0158 16.586 16.586 15.0158 17.3971 13.0522C16.4441 13.5935 15.4259 14.0334 14.3573 14.3573Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.94776 17.3971C4.98419 16.586 3.41399 15.0158 2.60288 13.0522C3.55593 13.5935 4.57408 14.0334 5.64268 14.3573C5.96656 15.4259 6.40648 16.4441 6.94776 17.3971Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
