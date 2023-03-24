#[cfg(feature = "SiDacia")]
use leptos::*;
#[cfg(feature = "SiDacia")]
///This icon requires the feature `SiDacia` to be enabled.
#[component]
pub fn Dacia(
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
        "M17.48 10.112v3.884h-1.11v-3.884h1.11zm-1.922-.04v.789h-2.232c-.068.002-1.06.047-1.06 1.173 0 .928.697 1.174 1.065 1.174h2.226v.788h-2.42c-.143 0-.646-.019-1.12-.268-.608-.319-.916-.877-.916-1.66 0-.826.341-1.412 1.015-1.743.489-.24.982-.254 1.047-.255h2.395zm-12.63-.001c.143 0 .646.019 1.12.268.608.319.916.877.916 1.66 0 .826-.341 1.412-1.015 1.743-.489.24-.982.254-1.046.255H0v-3.923h2.928zm18.191-.067c.059 0 .118.002.176.007.506.047.749.234.784.259.198.15.386.47.494.676.175.358.348.717.519 1.077.255.537.586 1.243.908 1.974h-1.198l-.13-.274-.029-.062-.213-.453h-2.648l-.213.453-.029.062-.13.274h-1.2c.322-.731.653-1.437.908-1.974.168-.354.335-.699.519-1.077.108-.205.296-.525.494-.676.035-.024.278-.212.784-.259.058-.005.117-.007.176-.007h.028zm-13.094 0c.059 0 .118.002.176.007.506.047.749.234.784.259.198.15.386.47.494.676.175.358.348.717.519 1.077.255.537.586 1.243.908 1.974H9.707l-.13-.274-.029-.062-.213-.453H6.686l-.213.453-.029.062-.13.274H5.116c.322-.731.653-1.437.908-1.974.168-.354.335-.699.519-1.077.107-.206.296-.526.493-.676.035-.024.278-.212.784-.259.058-.005.117-.007.176-.007h.029zm-5.291.856H1.08v2.347h1.659c.068-.002 1.06-.048 1.06-1.173 0-.928-.697-1.174-1.065-1.174zm18.371.027c-.206 0-.28.096-.327.154a2.852 2.852 0 0 0-.145.296l-.194.433-.03.067-.015.034-.03.067-.199.439-.015.033-.024.054h1.96s-.366-.81-.507-1.126c0 0-.111-.252-.145-.296-.048-.059-.123-.155-.329-.155zm-13.095 0c-.206 0-.28.096-.327.154a2.852 2.852 0 0 0-.145.296l-.194.433-.03.067-.016.033-.03.067-.199.439-.015.033-.024.054h1.96l-.238-.527-.03-.067-.24-.533s-.111-.252-.145-.296c-.046-.057-.121-.153-.327-.153z"
        /> < title > { title } < / title > < / svg >
    }
}
