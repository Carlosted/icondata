#[cfg(feature = "HiLgOutlineSignalSlash")]
use leptos::*;
#[cfg(feature = "HiLgOutlineSignalSlash")]
///This icon requires the feature `HiLgOutlineSignalSlash` to be enabled.
#[component]
pub fn SignalSlash(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 3L11.7348 11.7348M11.7348 11.7348C11.8027 11.667 11.8964 11.625 12 11.625C12.2071 11.625 12.375 11.7929 12.375 12C12.375 12.1036 12.333 12.1973 12.2652 12.2652M11.7348 11.7348L12.2652 12.2652M12.2652 12.2652L21 21M14.6517 9.34835C16.1161 10.8128 16.1161 13.1872 14.6517 14.6516M16.773 7.22703C19.409 9.86307 19.409 14.1369 16.773 16.773M18.8943 5.10571C22.7019 8.91332 22.7019 15.0867 18.8943 18.8943M9.34835 14.6516C8.75129 14.0546 8.39765 13.3063 8.28743 12.5301M7.22703 16.773C5.35299 14.8989 4.81126 12.1971 5.60184 9.84448M5.10571 18.8943C2.03824 15.8268 1.44197 11.2239 3.3169 7.55955M12 12H12.0075V12.0075H12V12Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
