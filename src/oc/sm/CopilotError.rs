#[cfg(feature = "OcSmCopilotError")]
use leptos::*;
#[cfg(feature = "OcSmCopilotError")]
///This icon requires the feature `OcSmCopilotError` to be enabled.
#[component]
pub fn CopilotError(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M.865 2.759v.001l14.82 10.722a.755.755 0 0 0 .188.1.751.751 0 0 1-1.063 1.025l-1.415-1.024c-.274.147-.613.315-1.008.482C11.296 14.528 9.756 15 8 15c-1.756 0-3.296-.473-4.387-.934a11.947 11.947 0 0 1-1.654-.859l-.098-.065-.028-.018-.006-.004-.015-.01a10.19 10.19 0 0 1-.792-.597 5.145 5.145 0 0 1-.605-.58 2.185 2.185 0 0 1-.259-.366A1.193 1.193 0 0 1 0 11V9.736a2.75 2.75 0 0 1 1.52-2.46l.067-.033.167-.838c-.175-.442-.238-.936-.251-1.434L.31 4.107a.75.75 0 0 1 .555-1.348ZM7.86 1.77c.05.053.097.107.14.164.043-.057.09-.111.14-.164.681-.731 1.737-.9 2.943-.765 1.23.136 2.145.527 2.724 1.26.566.716.693 1.614.693 2.485 0 .572-.053 1.147-.254 1.655l.168.838.066.033A2.75 2.75 0 0 1 16 9.736V11c0 .24-.086.438-.156.567a1.59 1.59 0 0 1-.075.125L13 9.688V7.824l-.023-.115c-.49.21-1.075.291-1.727.291-.22 0-.43-.012-.633-.036L6.824 5.22c.082-.233.143-.503.182-.813.117-.936-.038-1.396-.242-1.614-.193-.207-.637-.414-1.681-.298-.707.079-1.144.243-1.424.434l-1.251-.905c.58-.579 1.422-.899 2.51-1.02 1.205-.133 2.26.035 2.943.766ZM4.75 8c-.652 0-1.237-.081-1.727-.291L3 7.825v4.26c.387.225.788.426 1.2.6.97.412 2.306.815 3.8.815 1.494 0 2.829-.403 3.801-.815.076-.033.15-.065.22-.097L5.594 7.934A5.158 5.158 0 0 1 4.75 8Zm4.486-5.207c-.204.218-.359.678-.242 1.614.091.726.303 1.23.618 1.553.299.304.784.54 1.638.54.922 0 1.28-.199 1.442-.38.179-.2.308-.578.308-1.37 0-.765-.123-1.242-.37-1.555-.233-.296-.693-.586-1.713-.7-1.044-.116-1.488.091-1.681.298Z"
        /> < title > { title } < / title > < / svg >
    }
}
