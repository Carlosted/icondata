#[cfg(feature = "IoCogSharp")]
use leptos::*;
#[cfg(feature = "IoCogSharp")]
///This icon requires the feature `IoCogSharp` to be enabled.
#[component]
pub fn CogSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M464,249.93a10.58,10.58,0,0,0-9.36-9.94L429,235.84a5.42,5.42,0,0,1-4.5-4.67c-.49-3.15-1-6.42-1.7-9.52a5.52,5.52,0,0,1,2.63-5.85l22.78-12.65a10.35,10.35,0,0,0,5-12.83l-3.95-10.9a10.32,10.32,0,0,0-12.13-6.51l-25.55,5a5.5,5.5,0,0,1-5.82-2.81c-1.49-2.79-3.11-5.63-4.8-8.42a5.6,5.6,0,0,1,.44-6.5l17-19.63a10.44,10.44,0,0,0,.39-13.77l-7.42-8.91a10.24,10.24,0,0,0-13.58-2l-22.37,13.43a5.39,5.39,0,0,1-6.39-.63c-2.47-2.17-4.95-4.26-7.37-6.19a5.45,5.45,0,0,1-1.72-6.21l9.26-24.4a10.35,10.35,0,0,0-4.31-13.07l-10.08-5.85a10.31,10.31,0,0,0-13.46,2.83L325,96.28A4.58,4.58,0,0,1,319.4,98c-.62-.25-5.77-2.36-9.78-3.7a5.42,5.42,0,0,1-3.74-5.23L306.27,63a10.48,10.48,0,0,0-8.57-10.88l-11.45-2a10.45,10.45,0,0,0-11.75,7.17L266,82.1a5.42,5.42,0,0,1-5.36,3.65h-9.75a5.53,5.53,0,0,1-5.3-3.67l-8.46-24.67a10.46,10.46,0,0,0-11.77-7.25l-11.46,2a10.46,10.46,0,0,0-8.57,10.79l.4,26.16a5.45,5.45,0,0,1-3.86,5.25c-2.28.89-7.26,2.78-9.51,3.63-2,.72-4.19-.07-6-2.1l-16.26-20A10.3,10.3,0,0,0,156.69,73l-10.06,5.83A10.36,10.36,0,0,0,142.31,92l9.25,24.34a5.54,5.54,0,0,1-1.7,6.23c-2.43,2-4.92,4-7.4,6.22a5.38,5.38,0,0,1-6.35.64L114,115.74a10.39,10.39,0,0,0-13.61,2l-7.4,8.9a10.32,10.32,0,0,0,.37,13.76L110.45,160a5.42,5.42,0,0,1,.45,6.45c-1.71,2.72-3.34,5.58-4.82,8.44a5.53,5.53,0,0,1-5.86,2.82l-25.51-4.93a10.34,10.34,0,0,0-12.14,6.51l-4,10.88a10.37,10.37,0,0,0,5,12.85l22.78,12.65A5.39,5.39,0,0,1,89,221.59l-.23,1.24c-.53,2.8-1,5.45-1.47,8.27a5.48,5.48,0,0,1-4.46,4.64l-25.7,4.15A10.42,10.42,0,0,0,48,250.16v11.58A10.26,10.26,0,0,0,57.16,272l25.68,4.14a5.41,5.41,0,0,1,4.5,4.67c.49,3.16,1,6.42,1.7,9.52a5.52,5.52,0,0,1-2.63,5.85L63.64,308.85a10.35,10.35,0,0,0-5,12.83l4,10.9a10.33,10.33,0,0,0,12.13,6.51l25.55-4.95a5.5,5.5,0,0,1,5.82,2.81c1.5,2.8,3.12,5.64,4.8,8.42a5.58,5.58,0,0,1-.44,6.5l-17,19.64A10.41,10.41,0,0,0,93,385.27l7.41,8.91a10.24,10.24,0,0,0,13.58,2l22.37-13.43a5.39,5.39,0,0,1,6.39.63c2.48,2.17,5,4.26,7.37,6.19a5.45,5.45,0,0,1,1.72,6.21l-9.26,24.4a10.35,10.35,0,0,0,4.31,13.07L157,439.09a10.3,10.3,0,0,0,13.45-2.82L187,415.92c1.39-1.73,3.6-2.5,5.24-1.84,3.47,1.44,5.8,2.25,9.93,3.63a5.44,5.44,0,0,1,3.75,5.23l-.4,26.05a10.5,10.5,0,0,0,8.57,10.88l11.45,2a10.44,10.44,0,0,0,11.75-7.17l8.5-24.77a5.48,5.48,0,0,1,5.36-3.65h9.75a5.52,5.52,0,0,1,5.3,3.67l8.47,24.67a10.48,10.48,0,0,0,10,7.41,9.74,9.74,0,0,0,1.78-.16l11.47-2a10.46,10.46,0,0,0,8.56-10.79l-.4-26.16a5.43,5.43,0,0,1,3.75-5.2c3.84-1.29,6.53-2.33,8.91-3.24l.6-.24c3.06-1.06,4.53.14,5.47,1.31l16.75,20.63A10.3,10.3,0,0,0,355,439l10.07-5.83a10.35,10.35,0,0,0,4.31-13.1l-9.24-24.34a5.52,5.52,0,0,1,1.69-6.23c2.43-2,4.92-4,7.4-6.22a5.39,5.39,0,0,1,6.38-.62L398,396.06a10.39,10.39,0,0,0,13.61-2l7.4-8.9a10.31,10.31,0,0,0-.37-13.75l-17.06-19.67a5.42,5.42,0,0,1-.45-6.45c1.71-2.71,3.34-5.57,4.82-8.44a5.56,5.56,0,0,1,5.86-2.82L437.29,339a10.34,10.34,0,0,0,12.14-6.51l3.95-10.88a10.36,10.36,0,0,0-5-12.84L425.58,296.1a5.4,5.4,0,0,1-2.61-5.89l.23-1.25c.53-2.8,1-5.44,1.47-8.26a5.48,5.48,0,0,1,4.46-4.64l25.7-4.14A10.43,10.43,0,0,0,464,261.64V249.93ZM171.59,361.27a135.12,135.12,0,0,1,.5-210.94l60,105.61ZM256,391.11a133.75,133.75,0,0,1-48.49-9.05L268,276.79H389.22C379.21,341.45,323.29,391.11,256,391.11Zm12.06-155.9-59.95-105.5A133.87,133.87,0,0,1,256,120.89c67.29,0,123.21,49.66,133.22,114.32Z"
        /> < title > { title } < / title > < / svg >
    }
}
