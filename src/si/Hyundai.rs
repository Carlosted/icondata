#[cfg(feature = "SiHyundai")]
use leptos::*;
#[cfg(feature = "SiHyundai")]
///This icon requires the feature `SiHyundai` to be enabled.
#[component]
pub fn Hyundai(
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
        "M11.999 18.145c6.627 0 12.001-2.751 12.001-6.144 0-3.395-5.374-6.146-12.001-6.146C5.374 5.855 0 8.606 0 12.001c0 3.393 5.374 6.144 11.999 6.144m2.26-3.015c-.2.464-.545 1.454-1.336 1.85-.24.119-.537.174-.8.185H12c-2.214 0-4.276-.31-6.002-.834l-.066-.025c-.16-.053-.24-.127-.24-.218 0-.079.043-.14.099-.195l.109-.097c.4-.332 1.598-1.2 3.858-2.067.793-.301 1.786-.679 2.825-.9.608-.126 2.868-.473 1.675 2.301m6.062-6.194c.043-.074.1-.137.203-.142.056-.006.132.007.248.08 1.409.867 2.245 1.952 2.245 3.125 0 2.118-2.724 3.94-6.62 4.735-.248.05-.416.048-.471-.015-.04-.038-.05-.106 0-.19a.815.815 0 01.104-.145c2.12-2.5 3.736-6.189 4.195-7.253.035-.074.068-.147.096-.195M9.777 8.857c.2-.463.545-1.454 1.335-1.846.24-.12.537-.178.8-.185.061-.002.104 0 .12 0 2.217 0 4.276.306 6.004.833.013.006.053.02.066.025.16.054.24.127.24.218 0 .079-.042.137-.098.193a1.89 1.89 0 01-.11.096c-.397.335-1.598 1.201-3.858 2.068-.795.304-1.786.679-2.822.899-.61.13-2.87.474-1.677-2.3M7.6 7.264c.25-.048.415-.048.476.015.035.04.045.106-.002.19a.89.89 0 01-.104.142c-2.12 2.503-3.737 6.189-4.198 7.256a2.313 2.313 0 01-.096.195c-.04.073-.099.136-.2.142-.056.005-.135-.011-.251-.081C1.817 14.256.98 13.172.98 11.999c0-2.118 2.724-3.94 6.62-4.735Z"
        /> < title > { title } < / title > < / svg >
    }
}
