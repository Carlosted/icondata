#[cfg(feature = "SiBoehringeringelheim")]
use leptos::*;
#[cfg(feature = "SiBoehringeringelheim")]
///This icon requires the feature `SiBoehringeringelheim` to be enabled.
#[component]
pub fn Boehringeringelheim(
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
        "M12.53029 7.32626v16.64847C12.32849 24 12.22723 24 12.00009 24c-.22738 0-.3539 0-.55609-.02527V7.32626h1.08629zm-2.39964.00006v16.57275c-.42958-.076-.85921-.2022-1.08628-.27786V7.32632h1.08628zm4.79978-.00006v16.26937c-.55577.17692-.78316.25265-1.08628.30344V7.32626h1.08628zm-7.1998 7.73063v8.15995c-.22739-.07574-.32864-.1262-.55577-.2274-.17693-.07572-.30312-.12619-.53051-.25264v-7.67991h1.08628zm9.62502 0v7.67984c-.42932.20219-.73237.3539-1.11155.48004V15.0569h1.11155zM12.00015 0c6.64404 0 11.99985 5.38108 11.99985 12.05063 0 2.65267-.8589 5.22935-2.42528 7.225-.7075.93488-1.3894 1.54112-2.9306 2.70314v-6.92188h1.08629v4.90097c2.27388-2.6022 3.25891-4.9767 3.25891-7.9325C22.98932 5.86112 18.08835.8841 12.0001.8841 5.91152.8841.98529 5.88632.98529 12.07583c0 2.82934.90936 5.02717 3.25891 7.88203v-4.90097h1.11155v6.92188c-1.23794-.73238-2.39996-1.91998-3.41057-3.41032C.6567 16.62328 0 14.45066 0 12.1263 0 5.38108 5.33061 0 12.00016 0zm.00001 2.65267l3.63777 2.85454-.63151.73276-3.00626-2.34955-3.00658 2.34955-.63144-.7075 3.63802-2.8798z"
        /> < title > { title } < / title > < / svg >
    }
}
