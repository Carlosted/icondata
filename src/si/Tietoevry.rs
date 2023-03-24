#[cfg(feature = "SiTietoevry")]
use leptos::*;
#[cfg(feature = "SiTietoevry")]
///This icon requires the feature `SiTietoevry` to be enabled.
#[component]
pub fn Tietoevry(
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
        "M1 1.239v1.966H0v1.282h1v2.55c0 1.641.434 2.131 2.121 2.131.36 0 .72-.047 1.084-.059V7.654c-.174 0-.351.024-.514.024-.43 0-.709-.105-.709-.582v-2.61h1.223v-1.28H2.982L2.98 1.238zm3.744 0v1.427h1.98V1.24zm9.66 0v1.966h-1v1.282h1v2.55c0 1.641.434 2.131 2.12 2.131.36 0 .72-.047 1.085-.059V7.654c-.174 0-.352.024-.514.024-.43 0-.709-.105-.709-.582v-2.61h1.223v-1.28h-1.223l-.002-1.967zm-4.201 1.814a3.129 3.129 0 00-3.111 3.088c0 1.967 1.338 3.12 3.306 3.12 1.351 0 2.642-.588 3.004-1.944H11.55a1.183 1.183 0 01-1.128.644 1.27 1.27 0 01-1.35-1.334h4.435c.094-1.989-.99-3.572-3.156-3.572a3.129 3.129 0 00-.148-.002zm10.761 0a3.044 3.044 0 00-.158.002 3.104 3.104 0 103.193 3.11 3.044 3.044 0 00-3.035-3.112zm-16.22.152V9.11h1.98V3.205zm5.588 1.15a1.287 1.287 0 01.02 0 1.206 1.206 0 011.175 1.198H9.072a1.287 1.287 0 011.26-1.197zm10.474.17c.919 0 1.21.893 1.21 1.64 0 .745-.29 1.626-1.21 1.626-.919 0-1.197-.88-1.197-1.627 0-.746.278-1.638 1.197-1.638zM4.599 10.54a2.365 2.365 0 00-1.376.428 1.494 1.494 0 00-.327.701l-.683 2.361a.98.98 0 01-.334.656c-.469.282-1.426.301-1.34 1.336a5.366 5.366 0 00.428 1.547 1.165 1.165 0 01.068.69c-.03.201-.065.401-.105.601-.034.174-.114.55.004.692.117.142.34.13.607.097a15.92 15.92 0 003.322-.994 2.974 2.974 0 00.61-.344 1.624 1.624 0 00.488-1.002.393.393 0 00-.16-.414c-.124-.07-.293-.026-.55.045-.422.115-1.469.519-1.831.63-.147.044-.437.113-.549-.028-.186-.234.08-.967.24-1.375a1.512 1.512 0 011.178-.957c.282-.081.745-.229.933-.293a.849.849 0 00.594-.672c.132-.532-.272-.614-.64-.717-.528-.146-.858-.163-.627-.675a.525.525 0 01.388-.36c.21-.055 1.983-.338 2.688-.467.395-.072.53-.285.695-.627.359-.743-.089-.79-.451-.757-.817.075-2.13.314-2.691.396-.3.045-.101-.47-.45-.494a2.365 2.365 0 00-.129-.004zm11.067 1.149a5.006 5.006 0 00-1.707 1.136c-.045-.303-.052-.637-.36-.672a1.275 1.275 0 00-.68.18 1.102 1.102 0 00-.49.47l-.718 1.659a.13.13 0 01-.108.08.72.72 0 00-.539.45.623.623 0 00.123.657c-.142.384-.52 1.28-.52 1.28-.469 1.097-.438 1.478-.27 1.71a1.305 1.305 0 00.743.35c.272.045.48-.118.707-.48.336-.535 1.01-1.948 1.01-1.948a10.646 10.646 0 004.03 2.436s-1.423 1.967-1.628 3.054c0 0-.105.714.852.711a1.787 1.787 0 00.865-.22.467.467 0 00.219-.282.221.221 0 00-.022-.152.466.466 0 00-.097-.103c-.05-.05-.043-.113-.016-.186.527-1.54 2.67-4.155 3.125-4.722 1.206-1.503 3.433-3.843 3.592-4.03.288-.371.344-.593-.127-1.056-.423-.416-.705-.17-.912.045-.035.041-.994.996-2.004 2.162-.786.907-1.467 1.752-1.467 1.752a8.91 8.91 0 01.06-1.284c.049-.406.18-1.196.221-1.447.2-1.27-.58-1.321-.87-1.34-.306-.018-.451.146-.542.534a14.45 14.45 0 00-.357 4.31c.032.362.102.734.11.758l-.184.256a7.312 7.312 0 01-2.881-1.657A5.466 5.466 0 0016 15.557a2.539 2.539 0 001.332-2.346 1.695 1.695 0 00-1.667-1.523zm-4.625.205a.538.538 0 00-.514.337c-.254.436-.93 1.679-1.541 2.641a10.35 10.35 0 01-1.07 1.52s0-2.442-.031-2.653c-.1-.7-.467-.63-.74-.582-.708.12-.7.388-.733.615a33.16 33.16 0 00.066 3.41c.026.314-.024 1.42.616 1.51.481.068.94-.375 1.289-.76a19.636 19.636 0 001.564-2.01 37.079 37.079 0 001.787-2.763c.231-.408.218-.695-.31-1.092a.538.538 0 00-.383-.173zm4.513 1.306a.353.353 0 01.24.022c.295.16-.044.85-.61 1.271a2.985 2.985 0 01-1.638.678 6.203 6.203 0 01.995-1.295c.344-.345.732-.619 1.013-.676Z"
        /> < title > { title } < / title > < / svg >
    }
}
