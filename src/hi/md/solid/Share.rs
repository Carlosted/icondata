#[cfg(feature = "HiMdSolidShare")]
use leptos::*;
#[cfg(feature = "HiMdSolidShare")]
///This icon requires the feature `HiMdSolidShare` to be enabled.
#[component]
pub fn Share(
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
        "M13 4.5C13 3.11929 14.1193 2 15.5 2C16.8807 2 18 3.11929 18 4.5C18 5.88071 16.8807 7 15.5 7C14.7943 7 14.1569 6.70762 13.7024 6.23739L6.96884 9.60415C6.98935 9.73308 7 9.8653 7 10C7 10.1347 6.98934 10.267 6.96882 10.3959L13.7023 13.7627C14.1569 13.2924 14.7943 13 15.5 13C16.8807 13 18 14.1193 18 15.5C18 16.8807 16.8807 18 15.5 18C14.1193 18 13 16.8807 13 15.5C13 15.3653 13.0107 15.2331 13.0312 15.1041L6.29764 11.7374C5.84307 12.2076 5.20568 12.5 4.5 12.5C3.11929 12.5 2 11.3807 2 10C2 8.61929 3.11929 7.5 4.5 7.5C5.20571 7.5 5.84312 7.79241 6.29769 8.26267L13.0312 4.89593C13.0107 4.76697 13 4.63473 13 4.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
