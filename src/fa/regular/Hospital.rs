#[cfg(feature = "FaRegularHospital")]
use leptos::*;
#[cfg(feature = "FaRegularHospital")]
///This icon requires the feature `FaRegularHospital` to be enabled.
#[component]
pub fn Hospital(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M232 0c-39.8 0-72 32.2-72 72v8H72C32.2 80 0 112.2 0 152V440c0 39.8 32.2 72 72 72h.2 .2 .2 .2 .2H73h.2 .2 .2 .2 .2 .2 .2 .2 .2 .2H75h.2 .2 .2 .2 .2 .2 .2 .2 .2 .2H77h.2 .2 .2 .2 .2 .2 .2 .2 .2 .2H79h.2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2H82h.2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2H85h.2 .2 .2 .2H86h.2 .2 .2 .2H87h.2 .2 .2 .2H88h.2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2H98h.2 .2 .2 .2H99h.2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2 .2v0H456h8v0H568c39.8 0 72-32.2 72-72V152c0-39.8-32.2-72-72-72H480V72c0-39.8-32.2-72-72-72H232zM480 128h88c13.3 0 24 10.7 24 24v40H536c-13.3 0-24 10.7-24 24s10.7 24 24 24h56v48H536c-13.3 0-24 10.7-24 24s10.7 24 24 24h56V440c0 13.3-10.7 24-24 24H480V336 128zM72 128h88V464h-.1-.2-.2-.2H159h-.2-.2-.2H158h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H154h-.2-.2-.2H153h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H150h-.2-.2-.2H149h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H146h-.2-.2-.2H145h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H142h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H139h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H136h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H133h-.2-.2-.2-.2-.2-.2-.2-.2H131h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H128h-.2-.2-.2-.2-.2-.2-.2-.2H126h-.2-.2-.2-.2-.2-.2-.2-.2H124h-.2-.2-.2-.2-.2-.2-.2-.2H122h-.2-.2-.2-.2-.2-.2-.2-.2H120h-.2-.2-.2-.2-.2-.2-.2-.2H118h-.2-.2-.2-.2-.2-.2-.2-.2H116h-.2-.2-.2-.2-.2-.2-.2-.2H114h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H111h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H108h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H105h-.2-.2-.2-.2H104h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H100h-.2-.2-.2-.2H99h-.2-.2-.2-.2H98h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H88h-.2-.2-.2-.2H87h-.2-.2-.2-.2H86h-.2-.2-.2-.2H85h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H82h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H79h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H77h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H75h-.2-.2-.2-.2-.2-.2-.2-.2-.2-.2H73h-.2-.2-.2-.2-.2H72c-13.2 0-24-10.7-24-24V336h56c13.3 0 24-10.7 24-24s-10.7-24-24-24H48V240h56c13.3 0 24-10.7 24-24s-10.7-24-24-24H48V152c0-13.3 10.7-24 24-24zM208 72c0-13.3 10.7-24 24-24H408c13.3 0 24 10.7 24 24V336 464H368V400c0-26.5-21.5-48-48-48s-48 21.5-48 48v64H208V72zm88 24v24H272c-8.8 0-16 7.2-16 16v16c0 8.8 7.2 16 16 16h24v24c0 8.8 7.2 16 16 16h16c8.8 0 16-7.2 16-16V168h24c8.8 0 16-7.2 16-16V136c0-8.8-7.2-16-16-16H344V96c0-8.8-7.2-16-16-16H312c-8.8 0-16 7.2-16 16z"
        /> < title > { title } < / title > < / svg >
    }
}
