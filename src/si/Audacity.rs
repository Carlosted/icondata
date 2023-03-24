#[cfg(feature = "SiAudacity")]
use leptos::*;
#[cfg(feature = "SiAudacity")]
///This icon requires the feature `SiAudacity` to be enabled.
#[component]
pub fn Audacity(
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
        "M12 2.145c-2.487 0-4.755.776-6.428 2.08-1.673 1.303-2.76 3.169-2.76 5.244v.75C1.153 11.06 0 13.268 0 15.856c0 3.312 1.884 6 4.312 6V9.468c0-1.554.805-2.984 2.186-4.06C7.879 4.331 9.829 3.643 12 3.643c2.17 0 4.12.688 5.502 1.764 1.38 1.077 2.186 2.507 2.186 4.06v12.387c2.428 0 4.312-2.687 4.312-6 0-2.587-1.152-4.795-2.813-5.636v-.75c0-2.075-1.086-3.94-2.76-5.244-1.672-1.304-3.94-2.08-6.427-2.08zm0 6.153l-1.125 8.683L9.75 9.105l-.562 6.75-.376-.75-.375-4.5-.187 4.5-.563 1.313-.374-4.5-.376 3.562-.562-.937v2.625l-.563-2.11v-4.64a1.432 1.432 0 0 0-.937-.375v11.812c.375 0 .75-.187.937-.562v-3.375l.188.187.563 1.875.187-2.25.563 2.813v-3.562l.374.937.563 2.625v-3.562l.375.374.563 3.188.562-4.313 1.24 4.86.072-2.985.375-1.124.376 4.687 1.124-4.687.375 3.937.938-4.125.938 4.5.187-3.375.562-1.125.188 4.313.938-4.125.562 1.875.188-1.688.374.75v3.375c.188.375.563.562.938.562V10.043c-.375 0-.75.188-.938.375v4.813l-.374 1-.188-3.188-.375 2.437-.375-.75-.188-2.625-.937 3.563-.188-.75L15 9.293l-.562 4.875-.376 1.5-.75-5.062-.75 4.312-.375 1.125Z"
        /> < title > { title } < / title > < / svg >
    }
}
