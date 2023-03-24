#[cfg(feature = "BiLogosJquery")]
use leptos::*;
#[cfg(feature = "BiLogosJquery")]
///This icon requires the feature `BiLogosJquery` to be enabled.
#[component]
pub fn Jquery(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M3.345 6.935c-1.756 2.523-1.538 5.806-.196 8.487l.098.19c.021.04.041.082.063.121.012.024.026.047.039.069a7.842 7.842 0 0 0 .198.344l.073.12a9.14 9.14 0 0 0 .211.33 10.179 10.179 0 0 0 .221.319l.036.049c.063.088.129.175.196.261l.074.094.182.223.069.082c.082.097.167.194.252.289l.005.005c.004.004.007.006.01.011.083.091.17.181.256.271l.083.083.205.201.084.08c.092.087.186.172.281.256l.004.004.049.041c.083.073.169.145.255.215l.105.084a11.03 11.03 0 0 0 .565.424c.029.021.057.042.087.062l.024.017c.076.053.154.103.231.153.033.022.066.045.101.067a10.975 10.975 0 0 0 .886.509c.065.034.129.068.195.101l.045.022.08.039c.102.048.205.097.308.143l.066.029c.119.052.239.102.36.149l.088.035a13.895 13.895 0 0 0 .382.142c.125.044.252.085.38.124l.092.028c.128.039.256.085.39.11 8.492 1.548 10.958-5.102 10.958-5.102-2.072 2.698-5.748 3.41-9.232 2.618-.132-.03-.261-.071-.39-.109l-.097-.029a10.929 10.929 0 0 1-.755-.264l-.093-.036c-.12-.047-.239-.097-.356-.148l-.071-.031a11.932 11.932 0 0 1-.301-.14l-.087-.042c-.078-.038-.155-.079-.232-.118-.051-.027-.104-.053-.155-.082a8.294 8.294 0 0 1-.278-.156l-.094-.052a11.4 11.4 0 0 1-.363-.223l-.098-.065a10.557 10.557 0 0 1-.259-.172l-.083-.059c-.082-.058-.164-.116-.244-.176a11.921 11.921 0 0 1-.328-.255l-.099-.079c-.092-.076-.184-.152-.274-.231a12.01 12.01 0 0 1-.319-.288c-.028-.026-.055-.051-.081-.078a7.985 7.985 0 0 1-.208-.203l-.081-.081a10.76 10.76 0 0 1-.254-.269l-.012-.012a11.75 11.75 0 0 1-.258-.293l-.067-.081a10.313 10.313 0 0 1-.254-.313 11.855 11.855 0 0 1-.215-.286C3.864 11.825 3.17 8.186 4.715 5.198"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.794 4.809c-1.27 1.827-1.2 4.273-.21 6.205.166.324.353.639.561.938.191.273.401.597.654.816.092.101.187.199.284.295l.076.074c.094.092.19.182.291.271l.011.01.003.002c.111.097.224.19.34.281l.078.06a8.281 8.281 0 0 0 .366.268c.053.037.108.072.161.107.026.017.051.035.078.051a7.14 7.14 0 0 0 .301.184c.076.044.155.087.233.13.026.015.055.029.082.043.055.028.108.057.163.083a9.645 9.645 0 0 0 .365.171l.074.031c.093.039.186.077.281.113l.117.044c.086.032.171.06.256.089l.117.037c.121.038.243.085.37.107 6.556 1.086 8.068-3.961 8.068-3.961-1.364 1.964-4.006 2.902-6.825 2.17a7.866 7.866 0 0 1-.743-.232l-.118-.043a7.629 7.629 0 0 1-.353-.145 8.79 8.79 0 0 1-.344-.159c-.057-.028-.113-.058-.17-.087l-.099-.051a9.352 9.352 0 0 1-.533-.313l-.08-.053a7.65 7.65 0 0 1-.602-.435c-1.234-.974-2.213-2.306-2.678-3.815-.488-1.566-.382-3.323.462-4.75"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.379 3.221c-.749 1.102-.823 2.469-.304 3.686.548 1.292 1.671 2.304 2.981 2.785a3.85 3.85 0 0 0 .234.079c.077.024.152.053.233.067 3.62.699 4.601-1.857 4.862-2.234-.86 1.239-2.306 1.536-4.078 1.105a5.403 5.403 0 0 1-.939-.344 5.39 5.39 0 0 1-.895-.545c-1.585-1.204-2.573-3.501-1.536-5.372"
        /> < title > { title } < / title > < / svg >
    }
}
