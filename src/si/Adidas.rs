#[cfg(feature = "SiAdidas")]
use leptos::*;
#[cfg(feature = "SiAdidas")]
///This icon requires the feature `SiAdidas` to be enabled.
#[component]
pub fn Adidas(
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
        "M11.936 17.952c0-.644.517-1.16 1.162-1.16.644 0 1.16.516 1.16 1.16a1.157 1.157 0 01-1.16 1.161 1.157 1.157 0 01-1.162-1.16m4.724 0c0-.645.517-1.162 1.161-1.162s1.161.517 1.161 1.161-.517 1.161-1.16 1.161a1.157 1.157 0 01-1.162-1.16m-10.95 0c0-.645.517-1.162 1.161-1.162s1.16.517 1.16 1.161-.516 1.161-1.16 1.161a1.157 1.157 0 01-1.161-1.16m-4.724 0c0-.645.517-1.162 1.161-1.162s1.161.517 1.161 1.161a1.157 1.157 0 01-1.161 1.161 1.157 1.157 0 01-1.16-1.16m9.55-2.052h-1.01v4.063h1.01v-4.063zM3.3 19.964h1.01v-4.063H3.3v.326a2.087 2.087 0 00-1.2-.374c-1.162 0-2.1.938-2.1 2.1 0 1.168.938 2.099 2.1 2.099.445 0 .858-.135 1.2-.374v.286zm15.674 0h1.01v-4.063h-1.01v.326a2.087 2.087 0 00-1.2-.374c-1.162 0-2.1.938-2.1 2.1a2.092 2.092 0 002.1 2.099c.445 0 .858-.135 1.2-.374v.286zm1.384-1.32c.032.82.732 1.4 1.9 1.4.955 0 1.742-.414 1.742-1.328 0-.636-.358-1.01-1.185-1.17l-.644-.126c-.414-.08-.7-.16-.7-.406 0-.27.278-.39.628-.39.51 0 .716.255.732.557h1.018c-.056-.795-.692-1.328-1.718-1.328-1.057 0-1.686.58-1.686 1.336 0 .922.748 1.073 1.392 1.193l.533.095c.382.072.549.183.549.406 0 .199-.191.397-.645.397-.66 0-.874-.342-.882-.636h-1.034zM8.024 14.517v1.71a2.087 2.087 0 00-1.2-.374c-1.162 0-2.1.938-2.1 2.1 0 1.168.938 2.099 2.1 2.099.444 0 .858-.135 1.2-.374v.286h1.01v-5.447h-1.01zm6.226 0v1.71a2.087 2.087 0 00-1.2-.374c-1.161 0-2.1.938-2.1 2.1a2.092 2.092 0 002.1 2.099c.445 0 .858-.135 1.2-.374v.286h1.01v-5.447h-1.01zm-11.626-1.2l.684 1.2h4.716l-1.869-3.229-3.53 2.028zm7.913 2.21v-1.01h3.713l-3.96-6.855L6.751 9.69l2.776 4.827v1.01h1.01zm5.217-1.01h4.723L14.37 3.948l-3.531 2.036 4.915 8.533z"
        /> < title > { title } < / title > < / svg >
    }
}
