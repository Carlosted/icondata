#[cfg(feature = "SiCreativetechnology")]
use leptos::*;
#[cfg(feature = "SiCreativetechnology")]
///This icon requires the feature `SiCreativetechnology` to be enabled.
#[component]
pub fn Creativetechnology(
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
        "M13.48 13.248h-.593l-1.028-2.213-1.004 2.213h-.514l1.543-3.422 1.595 3.422M2.804 11.19s-.18-.103-.283-.155c-.206-.077-.489-.18-.798-.18-.617 0-1.183.283-1.183 1.004 0 .72.54 1.003 1.21 1.003.334 0 .668-.103.848-.257.103-.078.257-.232.257-.232l.283.437-.231.155c-.232.154-.54.36-1.21.36C.85 13.325 0 12.887 0 11.859c0-.927.797-1.467 1.775-1.467.437 0 .772.103 1.003.232.129 0 .309.128.309.128l-.283.438m7.537 2.546h3.138v.438h-3.138v-.438m12.116-2.135h.746v.54h-.746v-.54m-1.107-.051v-1.106H24v.437h-2.11v.669h-.54m.54.566v.694H24v.438h-2.65v-1.132h.54M4.786 11.6h.668c.283 0 .515-.103.515-.36s-.232-.386-.515-.386H4.27v2.367h-.54v-2.778h1.775c.669 0 1.029.36 1.029.797 0 .437-.283.772-.772.798l.746 1.029c.026.077.103.18.103.18h-.695L4.785 11.6m10.186 1.647h-.54V10.88h-1.106v-.437h2.804v.437h-1.106l-.052 2.367m2.238 0h-.54v-2.804h.54v2.804m3.138-2.804-.952 2.11-.951-2.11h-.618l1.312 2.804h.54l1.287-2.83h-.618M8.386 11.55h.746v.54h-.746v-.54m-1.106 0v-1.106h2.65v.437H7.82v.669h-.54m.566.566v.694h2.109v.438H7.28v-1.132h.566"
        /> < title > { title } < / title > < / svg >
    }
}
