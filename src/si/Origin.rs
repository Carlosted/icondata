#[cfg(feature = "SiOrigin")]
use leptos::*;
#[cfg(feature = "SiOrigin")]
///This icon requires the feature `SiOrigin` to be enabled.
#[component]
pub fn Origin(
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
        "M12.588 3.11a8.78 8.78 0 013.417.919 8.775 8.775 0 012.706 2.094 9.113 9.113 0 011.715 2.963 8.65 8.65 0 01.465 3.502 8.224 8.224 0 01-.307 1.848 9.174 9.174 0 01-.674 1.703 19.96 19.96 0 01-1.47 2.412 17.61 17.61 0 01-1.762 2.118 18.61 18.61 0 01-4.286 3.281l-.037.026a.196.196 0 01-.109.023.293.293 0 01-.159-.097.266.266 0 01-.062-.173c0-.029.004-.059.012-.085a.186.186 0 01.037-.062c.277-.393.506-.806.686-1.235a5.42 5.42 0 00.368-1.359.118.118 0 00-.038-.085.11.11 0 00-.085-.038 9.155 9.155 0 01-.795.062 4.926 4.926 0 01-.796-.037 8.818 8.818 0 01-6.123-3.013 9.089 9.089 0 01-1.715-2.963 8.662 8.662 0 01-.465-3.502 8.224 8.224 0 01.306-1.848 8.598 8.598 0 01.675-1.68c.439-.864.93-1.676 1.469-2.436a18.035 18.035 0 011.76-2.119A18.801 18.801 0 0111.609.05l.038-.025a.187.187 0 01.11-.025.295.295 0 01.157.098.255.255 0 01.062.174.244.244 0 01-.012.084.164.164 0 01-.036.061 6.447 6.447 0 00-.687 1.237c-.18.433-.3.885-.366 1.358 0 .033.012.063.036.086a.117.117 0 00.085.037c.262-.033.527-.053.795-.06.272-.01.536.002.798.034zm-.807 12.367a3.32 3.32 0 002.521-.855c.72-.64 1.11-1.438 1.176-2.4a3.357 3.357 0 00-.856-2.535 3.294 3.294 0 00-2.4-1.162 3.36 3.36 0 00-2.534.855 3.3 3.3 0 00-1.164 2.4 3.381 3.381 0 00.846 2.535c.628.725 1.432 1.115 2.411 1.162z"
        /> < title > { title } < / title > < / svg >
    }
}
