#[cfg(feature = "BsFiletypeTsx")]
use leptos::*;
#[cfg(feature = "BsFiletypeTsx")]
///This icon requires the feature `BsFiletypeTsx` to be enabled.
#[component]
pub fn FiletypeTsx(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-filetype-tsx" viewBox = "0 0 16 16" width = size.clone() height =
        size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" d =
        "M14 4.5V14a2 2 0 0 1-2 2h-1v-1h1a1 1 0 0 0 1-1V4.5h-2A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v9H2V2a2 2 0 0 1 2-2h5.5L14 4.5ZM3.172 14.841a1.13 1.13 0 0 0 .401.823c.129.108.288.192.478.252.189.061.41.091.665.091.338 0 .624-.053.858-.158.236-.105.416-.252.54-.44a1.17 1.17 0 0 0 .187-.656c0-.224-.045-.41-.135-.56a1.001 1.001 0 0 0-.375-.357 2.027 2.027 0 0 0-.566-.21l-.62-.144a.97.97 0 0 1-.405-.176.37.37 0 0 1-.144-.299c0-.156.062-.284.185-.384.125-.101.296-.152.513-.152.142 0 .265.023.369.068a.624.624 0 0 1 .246.181.56.56 0 0 1 .12.258h.75a1.092 1.092 0 0 0-.2-.566 1.21 1.21 0 0 0-.5-.41 1.813 1.813 0 0 0-.78-.152c-.292 0-.551.05-.776.15-.224.099-.4.24-.527.421-.127.182-.19.395-.19.639 0 .201.04.376.122.524.083.149.2.27.352.367.152.095.332.167.54.213l.617.144c.207.049.362.113.463.193a.387.387 0 0 1 .152.326.511.511 0 0 1-.084.29.559.559 0 0 1-.255.193 1.07 1.07 0 0 1-.413.07c-.118 0-.224-.013-.32-.04a.837.837 0 0 1-.249-.115.578.578 0 0 1-.255-.384h-.764Zm-1.244 1.09v-3.337h1.136v-.662H0v.662h1.134v3.337h.794Zm7.076-3.999h.893l-1.274 2.007 1.254 1.992h-.909l-.85-1.415h-.034l-.853 1.415H6.37l1.239-2.016-1.228-1.983h.932l.832 1.438h.035l.824-1.438Z"
        /> < title > { title } < / title > < / svg >
    }
}
