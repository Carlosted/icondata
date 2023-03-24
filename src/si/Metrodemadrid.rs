#[cfg(feature = "SiMetrodemadrid")]
use leptos::*;
#[cfg(feature = "SiMetrodemadrid")]
///This icon requires the feature `SiMetrodemadrid` to be enabled.
#[component]
pub fn Metrodemadrid(
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
        "M12 4.74L0 12l12 7.26L24 12 12 4.74zm0 2.905l3.45 2.087h-6.9L12 7.645zM7.2 10.64h.786l.606 1.77.564-1.77h.79v2.568h-.524l-.006-1.82-.627 1.82h-.432l-.632-1.805v1.805h-.526V10.64zm5.275.148h.51l.001.559h.371v.343h-.37v.92c0 .174.041.215.212.215a.69.69 0 0 0 .158-.014v.4a1.907 1.907 0 0 1-.298.018c-.313 0-.584-.073-.584-.443V11.69h-.307v-.341h.307v-.559zm5.222.303l1.5.908-1.5.908v-1.816zM6.3 11.094v1.812L4.8 12l1.498-.906zm8.455.203a.345.345 0 0 1 .11.017v.475a.81.81 0 0 0-.18-.018c-.37 0-.5.27-.5.598v.842h-.508v-1.863h.48l.006.345a.653.653 0 0 1 .592-.396zm1.127 0c.58 0 .957.39.957.982s-.377.98-.957.98c-.578 0-.953-.39-.953-.98 0-.593.375-.98.953-.982zm-4.738.002c.628 0 .932.532.896 1.103v.004h-1.334c.015.325.172.473.455.473.204 0 .368-.126.4-.24h.446c-.141.439-.445.625-.863.625-.581 0-.942-.407-.942-.98 0-.56.382-.985.942-.985zm-.012.383c-.322 0-.416.252-.422.396h.824c-.045-.262-.156-.396-.402-.396zm4.75 0c-.343 0-.445.3-.445.597 0 .295.102.596.445.596.349 0 .45-.3.45-.596 0-.3-.104-.597-.45-.597zM8.55 14.268h6.9L12 16.355l-3.451-2.087z"
        /> < title > { title } < / title > < / svg >
    }
}
