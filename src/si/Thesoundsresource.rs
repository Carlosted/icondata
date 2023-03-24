#[cfg(feature = "SiThesoundsresource")]
use leptos::*;
#[cfg(feature = "SiThesoundsresource")]
///This icon requires the feature `SiThesoundsresource` to be enabled.
#[component]
pub fn Thesoundsresource(
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
        "M1.25 0C.557 0 0 .557 0 1.25v4.807c.003-.02.001-.04.006-.06.16-.7 1.27-1.914 3.219-3.589C4.028 1.718 6.149.275 8.348 0H1.25zm8.21 0c1.119.23 2.168 1.177 1.55 2.338-.357.67-1.87 1.872-2.947 2.156l-.708-.098c.16-.56 1.48-1.784 1.473-2.453-.004-.47-.617-.87-1.193-.841-.728.036-2.025.873-3.166 1.845-1.142.973-2.129 2.08-2.112 2.658.028.91 2.086 1.213 4.176 1.182 2.148-.032 2.382-.095 4.164.006 1.596.09 5.601 1.363 5.44 3.535a3.108 3.108 0 01-.362 1.272c.087-.004.153-.021.245-.022.558-.003 1.337.84 1.337.84l-1.955.486s-.207-.247-.332-.242c-.267.01-4.72 3.241-4.53 6.768.111 2.084 2.746 3.566 5.187 3.508 2.584-.062 6.062-1.033 6.171-4.698.057-1.885-1.71-3.145-3.529-2.95-.737.078-2.585.79-2.478 2.165.083 1.077.747 1.45.902 1.416.549-.118 1.453-.43 1.453-.43l.219.297c-.038.391-2.31 1.254-3.207 1.248-1.468-.01-1.821-1.74-1.635-2.728.34-1.808 4.57-3.007 6.322-2.961 3.288.086 4.041 2.53 4.041 2.53l.014.015V1.25C24 .557 23.442 0 22.75 0H9.46zM0 6.324V22.75C0 23.442.557 24 1.25 24h12.037l-.006-.014s-4.661-.659-4.861-3.933c-.168-2.745 2.402-5.515 2.44-5.555-2.062.668-4.418 1.118-5.292 1.104-1.717-.029-5.058-.58-5.054-2.313.002-1.322.912-3.015 2.593-3.103.262-.017.442.017.643.384-.613.607-1.081 1.068-1.045 1.918.052 1.216 1.85 1.551 2.815 1.766.727.163 2.28.508 4.748-.332 2.495-.85 3.544-1.898 3.523-3.3-.009-.558-.476-1.458-2.36-2.087-1.732-.579-2.191-.664-4.267-.633-1.954.03-1.897.019-3.504-.013-1.822-.037-2.51-.402-2.883-.582C.474 7.16.07 6.784 0 6.324zm24 13.123l-.014.01s-.199 1.26-3.314 2.916c-2.064 1.097-5.568 1.625-5.568 1.625l-.002.002h7.648c.692 0 1.25-.558 1.25-1.25v-3.303z"
        /> < title > { title } < / title > < / svg >
    }
}
