#[cfg(feature = "SiTurbosquid")]
use leptos::*;
#[cfg(feature = "SiTurbosquid")]
///This icon requires the feature `SiTurbosquid` to be enabled.
#[component]
pub fn Turbosquid(
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
        "M11.909.01C9.832.153 7.89 1.617 7.89 1.617s4.235-1.383 5.42.752c.861 1.552-.133 3.989-1.67 6.6-1.418-1.212-5.21-4.035-8.73-3.038C-1.528 7.187.43 13.176.43 13.176S.45 8.632 2.803 8.14c1.71-.358 3.673 1.364 5.63 3.664-1.567 1.005-5.368 3.815-5.526 7.545C2.705 24.048 8.883 24 8.883 24s-4.224-1.424-3.955-3.863c.196-1.773 2.403-3.148 5.149-4.338.451 1.833 1.894 6.393 5.316 7.701 4.313 1.648 6.176-4.37 6.176-4.37s-2.627 3.662-4.816 2.647c-1.59-.737-2.189-3.308-2.448-6.343 1.845.127 6.536.137 8.811-2.785 2.867-3.681-2.158-7.353-2.158-7.353s2.597 3.687.976 5.5c-1.178 1.317-3.755 1.103-6.66.417.689-1.753 2.146-6.31.129-9.423C14.45.32 13.155-.074 11.909.01zm-1.266 2.487c-.547 0-1.097.072-1.557.162.78.292 1.413.754 1.862 1.361a3.578 3.578 0 01.646 1.524c.234-.536.407-1.024.524-1.475a1.254 1.254 0 00-.186-1.039 1.234 1.233 0 00-.93-.523c-.119-.008-.24-.01-.359-.01zm9.713 4.283c-.032.849-.263 1.606-.691 2.228a3.492 3.491 0 01-1.219 1.098 10.375 10.374 0 001.115.065c.142 0 .283-.003.418-.01a1.203 1.203 0 00.912-.502 1.283 1.283 0 00.198-1.065c-.164-.657-.459-1.302-.733-1.814zM3.362 9.626a1.23 1.23 0 00-1.143.802c-.245.629-.378 1.33-.451 1.91.785-1.018 1.825-1.582 2.967-1.582a3.438 3.438 0 01.494.036 9.569 9.569 0 00-1.211-.965 1.185 1.185 0 00-.656-.201zm12.953 8.546c.12.576.26 1.076.424 1.512a1.21 1.21 0 001.135.795 1.247 1.247 0 00.666-.202c.559-.362 1.07-.848 1.463-1.273a4.358 4.358 0 01-1.211.178 3.472 3.472 0 01-2.477-1.01zm-8.168.428a9.79 9.79 0 00-1.272.877 1.24 1.24 0 00-.449.95 1.269 1.269 0 00.451.981c.51.431 1.122.78 1.64 1.03-.465-.707-.711-1.46-.724-2.221a3.605 3.604 0 01.354-1.617z"
        /> < title > { title } < / title > < / svg >
    }
}
