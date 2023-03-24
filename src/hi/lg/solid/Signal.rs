#[cfg(feature = "HiLgSolidSignal")]
use leptos::*;
#[cfg(feature = "HiLgSolidSignal")]
///This icon requires the feature `HiLgSolidSignal` to be enabled.
#[component]
pub fn Signal(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M5.63604 4.57514C5.92893 4.86803 5.92893 5.34291 5.63604 5.6358C2.12132 9.15052 2.12132 14.849 5.63604 18.3637C5.92893 18.6566 5.92893 19.1315 5.63604 19.4244C5.34315 19.7173 4.86827 19.7173 4.57538 19.4244C0.474874 15.3239 0.474873 8.67564 4.57538 4.57514C4.86827 4.28225 5.34315 4.28225 5.63604 4.57514ZM18.364 4.57514C18.6569 4.28225 19.1317 4.28225 19.4246 4.57514C23.5251 8.67564 23.5251 15.3239 19.4246 19.4244C19.1317 19.7173 18.6569 19.7173 18.364 19.4244C18.0711 19.1315 18.0711 18.6566 18.364 18.3637C21.8787 14.849 21.8787 9.15052 18.364 5.6358C18.0711 5.34291 18.0711 4.86803 18.364 4.57514ZM7.75736 6.69646C8.05025 6.98935 8.05025 7.46423 7.75736 7.75712C5.41421 10.1003 5.41421 13.8993 7.75736 16.2424C8.05025 16.5353 8.05025 17.0102 7.75736 17.3031C7.46447 17.596 6.98959 17.596 6.6967 17.3031C3.76777 14.3741 3.76777 9.62539 6.6967 6.69646C6.98959 6.40357 7.46447 6.40357 7.75736 6.69646ZM16.2426 6.69646C16.5355 6.40357 17.0104 6.40357 17.3033 6.69646C20.2322 9.62539 20.2322 14.3741 17.3033 17.3031C17.0104 17.596 16.5355 17.596 16.2426 17.3031C15.9497 17.0102 15.9497 16.5353 16.2426 16.2424C18.5858 13.8993 18.5858 10.1003 16.2426 7.75712C15.9497 7.46423 15.9497 6.98935 16.2426 6.69646ZM9.87868 8.81778C10.1716 9.11067 10.1716 9.58555 9.87868 9.87844C8.70711 11.05 8.70711 12.9495 9.87868 14.1211C10.1716 14.414 10.1716 14.8888 9.87868 15.1817C9.58579 15.4746 9.11091 15.4746 8.81802 15.1817C7.06066 13.4244 7.06066 10.5751 8.81802 8.81778C9.11091 8.52489 9.58579 8.52489 9.87868 8.81778ZM14.1213 8.81778C14.4142 8.52489 14.8891 8.52489 15.182 8.81778C16.9393 10.5751 16.9393 13.4244 15.182 15.1817C14.8891 15.4746 14.4142 15.4746 14.1213 15.1817C13.8284 14.8888 13.8284 14.414 14.1213 14.1211C15.2929 12.9495 15.2929 11.05 14.1213 9.87844C13.8284 9.58555 13.8284 9.11067 14.1213 8.81778ZM10.875 11.9998C10.875 11.3784 11.3787 10.8748 12 10.8748C12.6213 10.8748 13.125 11.3784 13.125 11.9998C13.125 12.6211 12.6213 13.1248 12 13.1248C11.3787 13.1248 10.875 12.6211 10.875 11.9998Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
