#[cfg(feature = "OcLgBellSlash")]
use leptos::*;
#[cfg(feature = "OcLgBellSlash")]
///This icon requires the feature `OcLgBellSlash` to be enabled.
#[component]
pub fn BellSlash(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M1.22 1.22a.75.75 0 0 1 1.06 0l20.5 20.5a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L17.94 19H15.5a3.5 3.5 0 1 1-7 0H3.518a1.516 1.516 0 0 1-1.263-2.36l2.2-3.298A3.249 3.249 0 0 0 5 11.539V7c0-.294.025-.583.073-.866L1.22 2.28a.75.75 0 0 1 0-1.06ZM6.5 7.56h-.001v3.979a4.75 4.75 0 0 1-.797 2.635l-2.2 3.298-.003.01.001.007.004.006.006.004.007.001H16.44ZM10 19a2 2 0 1 0 4 0Zm2-16.5c-1.463 0-2.8.485-3.788 1.257l-.04.032a.75.75 0 1 1-.935-1.173l.05-.04C8.548 1.59 10.212 1 12 1c3.681 0 7 2.565 7 6v4.539c0 .642.19 1.269.546 1.803l1.328 1.992a.75.75 0 1 1-1.248.832l-1.328-1.992a4.75 4.75 0 0 1-.798-2.635V7c0-2.364-2.383-4.5-5.5-4.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
