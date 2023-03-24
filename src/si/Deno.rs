#[cfg(feature = "SiDeno")]
use leptos::*;
#[cfg(feature = "SiDeno")]
///This icon requires the feature `SiDeno` to be enabled.
#[component]
pub fn Deno(
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
        "M12 0c6.627 0 12 5.373 12 12s-5.373 12-12 12S0 18.627 0 12 5.373 0 12 0Zm-.469 6.793c-3.49 0-6.204 2.196-6.204 4.928 0 2.58 2.498 4.228 6.37 4.145l.118-.003.425-.012-.109.279.013.029c.031.072.06.145.084.22l.01.028.015.045.021.065.014.045.014.047.015.049.021.075.022.079.015.054.023.084.022.088.023.091.023.095.015.065.024.1.023.103.032.143.017.074.024.114.024.117.025.12.035.174.029.142.037.195.02.1.028.155.03.158.039.217.04.225.04.231.041.24.042.246.042.254.042.26.032.201.055.344.022.14.055.36.045.295.034.227.046.308.023.156a10.758 10.758 0 0 0 6.529-3.412l.05-.055-.238-.891-.633-2.37-.395-1.47-.348-1.296-.213-.787-.136-.498-.081-.297-.073-.264-.032-.11-.018-.064-.01-.034-.008-.026a6.042 6.042 0 0 0-2.038-2.97c-1.134-.887-2.573-1.351-4.252-1.351ZM8.467 19.3a.586.586 0 0 0-.714.4l-.004.013-.527 1.953c.328.163.665.309 1.008.437l.08.03.57-2.114.004-.015a.586.586 0 0 0-.417-.704Zm3.264-1.43a.586.586 0 0 0-.715.4l-.004.014-.796 2.953-.004.014a.586.586 0 0 0 1.131.305l.004-.014.797-2.953.003-.014a.585.585 0 0 0 .013-.067l.002-.022-.019-.096-.027-.138-.018-.086a.584.584 0 0 0-.367-.295Zm-5.553-3.04a.59.59 0 0 0-.037.09l-.005.02-.797 2.953-.004.014a.586.586 0 0 0 1.131.306l.004-.014.723-2.678a5.295 5.295 0 0 1-1.015-.692Zm-1.9-3.397a.586.586 0 0 0-.715.4l-.004.013-.797 2.953-.003.015a.586.586 0 0 0 1.13.305l.005-.014.797-2.953.003-.015a.586.586 0 0 0-.416-.704Zm17.868-.67a.586.586 0 0 0-.715.399l-.004.014-.797 2.953-.003.014a.586.586 0 0 0 1.13.305l.005-.014.797-2.953.003-.014a.586.586 0 0 0-.416-.704ZM2.542 6.82a10.707 10.707 0 0 0-1.251 3.926.586.586 0 0 0 1.002-.22l.004-.014.797-2.953.003-.014a.586.586 0 0 0-.555-.725Zm17.585.02a.586.586 0 0 0-.714.4l-.004.014-.797 2.953-.004.014a.586.586 0 0 0 1.131.305l.004-.014.797-2.953.004-.014a.586.586 0 0 0-.417-.704Zm-7.846 1.926a.75.75 0 1 1 0 1.5.75.75 0 0 1 0-1.5Zm-6.27-4.733a.586.586 0 0 0-.715.398l-.004.015-.797 2.953-.004.014a.586.586 0 0 0 1.132.305l.003-.014.797-2.953.004-.014a.586.586 0 0 0-.417-.704Zm10.238.558a.586.586 0 0 0-.714.399l-.004.014-.536 1.984c.347.171.678.373.99.603l.051.038.626-2.32.004-.014a.586.586 0 0 0-.417-.704Zm-5.211-3.33c-.374.033-.746.086-1.115.158l-.078.015-.742 2.753-.004.015a.586.586 0 0 0 1.131.305l.004-.014.797-2.953.004-.015a.583.583 0 0 0 .003-.264Zm7.332 2.04-.156.58-.004.015a.586.586 0 0 0 1.131.305l.004-.014.017-.063a10.838 10.838 0 0 0-.923-.772l-.069-.051Zm-4.636-1.944-.283 1.048-.003.014a.586.586 0 0 0 1.13.305l.005-.014.297-1.102c-.35-.097-.705-.176-1.063-.237l-.083-.014Z"
        /> < title > { title } < / title > < / svg >
    }
}
