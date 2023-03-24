#[cfg(feature = "SiDaisyui")]
use leptos::*;
#[cfg(feature = "SiDaisyui")]
///This icon requires the feature `SiDaisyui` to be enabled.
#[component]
pub fn Daisyui(
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
        "M2.64 10.655v-1.6h1.31v4.92H2.64v-.31c-.09.09-.2.16-.32.22-.18.09-.39.13-.62.13-.34 0-.63-.08-.89-.24s-.46-.38-.6-.67c-.14-.28-.21-.61-.21-.98s.07-.68.21-.96c.14-.28.34-.5.59-.65.25-.15.55-.23.88-.23.23 0 .45.05.64.14.12.06.23.13.33.22l-.01.01Zm-.66 2.3c.2 0 .35-.07.47-.21.12-.14.18-.33.18-.57s-.06-.43-.18-.57c-.12-.14-.28-.21-.47-.21s-.35.07-.48.21c-.12.14-.19.33-.19.57s.06.42.19.57c.12.14.28.21.48.21Zm4.57-1.23c0-.12-.05-.21-.14-.27-.1-.08-.24-.12-.44-.12-.14 0-.29.02-.47.07s-.35.11-.53.2l-.08.04-.38-.93.07-.03c.29-.13.56-.23.83-.29.26-.06.54-.1.82-.1.5 0 .89.12 1.17.35.28.24.43.57.43.99v2.34H6.54v-.26c-.24.21-.56.31-.96.31s-.7-.11-.93-.32c-.23-.22-.34-.5-.34-.85s.13-.63.37-.83.59-.3 1.04-.3h.83Zm0 .98v-.2h-.59c-.28 0-.39.09-.39.27 0 .09.03.17.09.22.07.06.16.09.29.09.15 0 .29-.04.4-.11s.17-.16.2-.26v-.01Zm2.53-2.58c-.2 0-.36-.07-.49-.2s-.19-.3-.19-.49.06-.37.19-.5.3-.19.49-.19.36.07.49.19c.13.13.19.3.19.5s-.06.36-.19.49-.3.2-.49.2Zm.66.21v3.63h-1.3v-3.63h1.3Zm2.01 3.68c-.3 0-.59-.04-.87-.13s-.53-.21-.74-.38l-.05-.04.43-.89.08.06c.19.13.39.23.6.31.21.07.4.11.58.11.1 0 .17-.02.22-.04.04-.02.05-.05.05-.08 0-.05-.03-.09-.1-.12-.09-.04-.24-.09-.44-.15-.25-.07-.45-.15-.61-.22-.17-.08-.32-.19-.44-.34-.13-.16-.2-.36-.2-.61 0-.38.14-.68.43-.89s.64-.31 1.08-.31c.26 0 .52.04.77.11s.49.17.72.31l.07.04-.46.89-.08-.04c-.44-.23-.79-.34-1.06-.34-.08 0-.15 0-.19.04-.03.02-.05.05-.05.09s.03.08.09.11c.09.04.23.09.43.15.25.07.46.15.63.22.18.08.33.19.46.34.13.16.2.37.2.61 0 .38-.15.68-.44.89s-.66.31-1.11.31v-.01Zm3.2-.23-1.47-3.46h1.36l.76 2.08.68-2.08h1.32l-.05.12-1.49 3.8c-.14.34-.32.59-.56.76-.24.17-.53.25-.87.25-.2 0-.39-.03-.57-.09-.18-.06-.35-.16-.51-.29l-.06-.05.56-.94.08.06c.07.06.14.11.21.13.06.03.13.04.2.04.16 0 .26-.07.34-.22l.06-.12h.01v.01ZM20.09 14.055c-.42 0-.8-.08-1.12-.25-.32-.17-.58-.4-.75-.71-.18-.31-.26-.66-.26-1.06v-2.72h1.34v2.72c0 .26.08.46.23.62.15.15.34.23.57.23s.41-.07.55-.23c.14-.15.21-.36.21-.62v-2.72h1.34v2.72c0 .4-.09.76-.26 1.06-.17.31-.42.54-.74.71-.32.16-.69.25-1.11.25Zm3.91-.08h-1.34v-4.66H24v4.66Z"
        /> < title > { title } < / title > < / svg >
    }
}
