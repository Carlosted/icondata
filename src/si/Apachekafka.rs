#[cfg(feature = "SiApachekafka")]
use leptos::*;
#[cfg(feature = "SiApachekafka")]
///This icon requires the feature `SiApachekafka` to be enabled.
#[component]
pub fn Apachekafka(
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
        "M9.71 2.136a1.43 1.43 0 0 0-2.047 0h-.007a1.48 1.48 0 0 0-.421 1.042c0 .41.161.777.422 1.039l.007.007c.257.264.616.426 1.019.426.404 0 .766-.162 1.027-.426l.003-.007c.261-.262.421-.629.421-1.039 0-.408-.159-.777-.421-1.042H9.71zM8.683 22.295c.404 0 .766-.167 1.027-.429l.003-.008c.261-.261.421-.631.421-1.036 0-.41-.159-.778-.421-1.044H9.71a1.42 1.42 0 0 0-1.027-.432 1.4 1.4 0 0 0-1.02.432h-.007c-.26.266-.422.634-.422 1.044 0 .406.161.775.422 1.036l.007.008c.258.262.617.429 1.02.429zm7.89-4.462c.359-.096.683-.33.882-.684l.027-.052a1.47 1.47 0 0 0 .114-1.067 1.454 1.454 0 0 0-.675-.896l-.021-.014a1.425 1.425 0 0 0-1.078-.132c-.36.091-.684.335-.881.686-.2.349-.241.75-.146 1.119.099.363.33.691.675.896h.002c.346.203.737.239 1.101.144zm-6.405-7.342a2.083 2.083 0 0 0-1.485-.627c-.58 0-1.103.242-1.482.627-.378.385-.612.916-.612 1.507s.233 1.124.612 1.514a2.08 2.08 0 0 0 2.967 0c.379-.39.612-.923.612-1.514s-.233-1.122-.612-1.507zm-.835-2.51c.843.141 1.6.552 2.178 1.144h.004c.092.093.182.196.265.299l1.446-.851a3.176 3.176 0 0 1-.047-1.808 3.149 3.149 0 0 1 1.456-1.926l.025-.016a3.062 3.062 0 0 1 2.345-.306c.77.21 1.465.721 1.898 1.482v.002c.431.757.518 1.626.313 2.408a3.145 3.145 0 0 1-1.456 1.928l-.198.118h-.02a3.095 3.095 0 0 1-2.154.201 3.127 3.127 0 0 1-1.514-.944l-1.444.848a4.162 4.162 0 0 1 0 2.879l1.444.846c.413-.47.939-.789 1.514-.944a3.041 3.041 0 0 1 2.371.319l.048.023v.002a3.17 3.17 0 0 1 1.408 1.906 3.215 3.215 0 0 1-.313 2.405l-.026.053-.003-.005a3.147 3.147 0 0 1-1.867 1.436 3.096 3.096 0 0 1-2.371-.318v-.006a3.156 3.156 0 0 1-1.456-1.927 3.175 3.175 0 0 1 .047-1.805l-1.446-.848a3.905 3.905 0 0 1-.265.294l-.004.005a3.938 3.938 0 0 1-2.178 1.138v1.699a3.09 3.09 0 0 1 1.56.862l.002.004c.565.572.914 1.368.914 2.243 0 .873-.35 1.664-.914 2.239l-.002.009a3.1 3.1 0 0 1-2.21.931 3.1 3.1 0 0 1-2.206-.93h-.002v-.009a3.186 3.186 0 0 1-.916-2.239c0-.875.35-1.672.916-2.243v-.004h.002a3.1 3.1 0 0 1 1.558-.862v-1.699a3.926 3.926 0 0 1-2.176-1.138l-.006-.005a4.098 4.098 0 0 1-1.173-2.874c0-1.122.452-2.136 1.173-2.872h.006a3.947 3.947 0 0 1 2.176-1.144V6.289a3.137 3.137 0 0 1-1.558-.864h-.002v-.004a3.192 3.192 0 0 1-.916-2.243c0-.871.35-1.669.916-2.243l.002-.002A3.084 3.084 0 0 1 8.683 0c.861 0 1.641.355 2.21.932v.002h.002c.565.574.914 1.372.914 2.243 0 .876-.35 1.667-.914 2.243l-.002.005a3.142 3.142 0 0 1-1.56.864v1.692zm8.121-1.129l-.012-.019a1.452 1.452 0 0 0-.87-.668 1.43 1.43 0 0 0-1.103.146h.002c-.347.2-.58.529-.677.896-.095.365-.054.768.146 1.119l.007.009c.2.347.519.579.874.673.357.103.755.059 1.098-.144l.019-.009a1.47 1.47 0 0 0 .657-.885 1.493 1.493 0 0 0-.141-1.118"
        /> < title > { title } < / title > < / svg >
    }
}
