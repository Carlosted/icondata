#[cfg(feature = "CgDice6")]
use leptos::*;
#[cfg(feature = "CgDice6")]
///This icon requires the feature `CgDice6` to be enabled.
#[component]
pub fn Dice6(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.9451 5.05518C15.8405 5.05518 14.9451 5.95061 14.9451 7.05518C14.9451 8.15975 15.8405 9.05518 16.9451 9.05518C18.0496 9.05518 18.9451 8.15975 18.9451 7.05518C18.9451 5.95061 18.0496 5.05518 16.9451 5.05518Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.9451 16.8921C14.9451 15.7875 15.8405 14.8921 16.9451 14.8921C18.0496 14.8921 18.9451 15.7875 18.9451 16.8921C18.9451 17.9967 18.0496 18.8921 16.9451 18.8921C15.8405 18.8921 14.9451 17.9967 14.9451 16.8921Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.05518 14.8921C5.95061 14.8921 5.05518 15.7875 5.05518 16.8921C5.05518 17.9967 5.95061 18.8921 7.05518 18.8921C8.15975 18.8921 9.05518 17.9967 9.05518 16.8921C9.05518 15.7875 8.15975 14.8921 7.05518 14.8921Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.05518 7.05518C5.05518 5.95061 5.95061 5.05518 7.05518 5.05518C8.15975 5.05518 9.05518 5.95061 9.05518 7.05518C9.05518 8.15975 8.15975 9.05518 7.05518 9.05518C5.95061 9.05518 5.05518 8.15975 5.05518 7.05518Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.9451 9.97363C15.8405 9.97363 14.9451 10.8691 14.9451 11.9736C14.9451 13.0782 15.8405 13.9736 16.9451 13.9736C18.0496 13.9736 18.9451 13.0782 18.9451 11.9736C18.9451 10.8691 18.0496 9.97363 16.9451 9.97363Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.05518 11.9736C5.05518 10.8691 5.95061 9.97363 7.05518 9.97363C8.15975 9.97363 9.05518 10.8691 9.05518 11.9736C9.05518 13.0782 8.15975 13.9736 7.05518 13.9736C5.95061 13.9736 5.05518 13.0782 5.05518 11.9736Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M4 1C2.34315 1 1 2.34315 1 4V20C1 21.6569 2.34315 23 4 23H20C21.6569 23 23 21.6569 23 20V4C23 2.34315 21.6569 1 20 1H4ZM20 3H4C3.44772 3 3 3.44772 3 4V20C3 20.5523 3.44772 21 4 21H20C20.5523 21 21 20.5523 21 20V4C21 3.44772 20.5523 3 20 3Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
