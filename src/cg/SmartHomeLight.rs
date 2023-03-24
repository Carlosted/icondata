#[cfg(feature = "CgSmartHomeLight")]
use leptos::*;
#[cfg(feature = "CgSmartHomeLight")]
///This icon requires the feature `CgSmartHomeLight` to be enabled.
#[component]
pub fn SmartHomeLight(
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
        "M7.03435 6.5C7.03435 3.73858 9.27293 1.5 12.0344 1.5C14.7958 1.5 17.0344 3.73858 17.0344 6.5V10.5C17.0344 13.2614 14.7958 15.5 12.0344 15.5C9.27293 15.5 7.03435 13.2614 7.03435 10.5V6.5ZM15.0344 6.5V10.5C15.0344 12.1569 13.6912 13.5 12.0344 13.5C10.3775 13.5 9.03435 12.1569 9.03435 10.5V6.5C9.03435 4.84315 10.3775 3.5 12.0344 3.5C13.6912 3.5 15.0344 4.84315 15.0344 6.5Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.0344 16.5C11.4821 16.5 11.0344 16.9477 11.0344 17.5V21.5C11.0344 22.0523 11.4821 22.5 12.0344 22.5C12.5866 22.5 13.0344 22.0523 13.0344 21.5V17.5C13.0344 16.9477 12.5866 16.5 12.0344 16.5Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.74433 16.4397C7.93323 15.9207 8.50707 15.6531 9.02605 15.842C9.54502 16.0309 9.81261 16.6048 9.62372 17.1237L8.25564 20.8825C8.06675 21.4015 7.4929 21.6691 6.97393 21.4802C6.45495 21.2913 6.18736 20.7174 6.37625 20.1985L7.74433 16.4397Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.974 15.8421C14.4551 16.031 14.1875 16.6048 14.3764 17.1238L15.7445 20.8825C15.9333 21.4015 16.5072 21.6691 17.0262 21.4802C17.5451 21.2913 17.8127 20.7175 17.6238 20.1985L16.2558 16.4397C16.0669 15.9208 15.493 15.6532 14.974 15.8421Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
