#[cfg(feature = "SiWireguard")]
use leptos::*;
#[cfg(feature = "SiWireguard")]
///This icon requires the feature `SiWireguard` to be enabled.
#[component]
pub fn Wireguard(
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
        "M23.98 11.645S24.533 0 11.735 0C.418 0 .064 11.17.064 11.17S-1.6 24 11.997 24C25.04 24 23.98 11.645 23.98 11.645zM8.155 7.576c2.4-1.47 5.469-.571 6.618 1.638.218.419.246 1.063.108 1.503-.477 1.516-1.601 2.366-3.145 2.728.455-.39.817-.832.933-1.442a2.112 2.112 0 0 0-.364-1.677 2.14 2.14 0 0 0-2.465-.75c-.95.36-1.47 1.228-1.377 2.294.087.99.839 1.632 2.245 1.876-.21.111-.372.193-.53.281a5.113 5.113 0 0 0-1.644 1.43c-.143.192-.24.208-.458.075-2.827-1.729-3.009-6.067.078-7.956zM6.04 18.258c-.455.116-.895.286-1.359.438.227-1.532 2.021-2.943 3.539-2.782a3.91 3.91 0 0 0-.74 2.072c-.504.093-.98.155-1.44.272zM15.703 3.3c.448.017.898.01 1.347.02a2.324 2.324 0 0 1 .334.047 3.249 3.249 0 0 1-.34.434c-.16.15-.341.296-.573.069-.055-.055-.187-.042-.283-.044-.447-.005-.894-.02-1.34-.003a8.323 8.323 0 0 0-1.154.118c-.072.013-.178.25-.146.338.078.207.191.435.359.567.619.49 1.277.928 1.9 1.413.604.472 1.167.99 1.51 1.7.446.928.46 1.9.267 2.877-.322 1.63-1.147 2.98-2.483 3.962-.538.395-1.205.62-1.821.903-.543.25-1.1.465-1.644.712-.98.446-1.53 1.51-1.369 2.615.149 1.015 1.04 1.862 2.059 2.037 1.223.21 2.486-.586 2.785-1.83.336-1.397-.423-2.646-1.845-3.024l-.256-.066c.38-.17.708-.291 1.012-.458q.793-.437 1.558-.925c.15-.096.231-.096.36.014.977.846 1.56 1.898 1.724 3.187.27 2.135-.74 4.096-2.646 5.101-2.948 1.555-6.557-.215-7.208-3.484-.558-2.8 1.418-5.34 3.797-5.83 1.023-.211 1.958-.637 2.685-1.425.47-.508.697-.944.775-1.141a3.165 3.165 0 0 0 .217-1.158 2.71 2.71 0 0 0-.237-.992c-.248-.566-1.2-1.466-1.435-1.656l-2.24-1.754c-.079-.065-.168-.06-.36-.047-.23.016-.815.048-1.067-.018.204-.155.76-.38 1-.56-.726-.49-1.554-.314-2.315-.46.176-.328 1.046-.831 1.541-.888a7.323 7.323 0 0 0-.135-.822c-.03-.111-.154-.22-.263-.283-.262-.154-.541-.281-.843-.434a1.755 1.755 0 0 1 .906-.28 3.385 3.385 0 0 1 .908.088c.54.123.97.042 1.399-.324-.338-.136-.676-.26-1.003-.407a9.843 9.843 0 0 1-.942-.493c.85.118 1.671.437 2.54.32l.022-.118-2.018-.47c1.203-.11 2.323-.128 3.384.388.299.146.61.266.897.432.14.08.233.24.348.365.09.098.164.23.276.29.424.225.89.234 1.366.223l.01-.16c.479.15 1.017.702 1.017 1.105-.776 0-1.55-.003-2.325.004-.083 0-.165.061-.247.094.078.046.155.128.235.131z M14.703 2.153a.118.118 0 0 0-.016.19.179.179 0 0 0 .246.065c.075-.038.148-.078.238-.125-.072-.062-.13-.114-.19-.163-.106-.087-.193-.032-.278.033z"
        /> < title > { title } < / title > < / svg >
    }
}
