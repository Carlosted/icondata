#[cfg(feature = "SiBandrautomation")]
use leptos::*;
#[cfg(feature = "SiBandrautomation")]
///This icon requires the feature `SiBandrautomation` to be enabled.
#[component]
pub fn Bandrautomation(
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
        "M22.637 5.186H1.363C.613 5.186 0 5.799 0 6.548v10.903c0 .75.614 1.363 1.363 1.363h21.274c.75 0 1.363-.613 1.363-1.363V6.548c0-.75-.614-1.362-1.363-1.362zm1.039 12.265a1.04 1.04 0 0 1-1.039 1.04H1.363a1.04 1.04 0 0 1-1.039-1.04V6.548A1.04 1.04 0 0 1 1.363 5.51h21.274a1.04 1.04 0 0 1 1.039 1.038v10.903zm-.685-5.491c-.116-.267-.418-.332-.682-.307A1.51 1.51 0 0 0 23 10.644c.052-.285.044-.577.039-.865-.011-1.021.004-2.116.004-3.07a.576.576 0 0 0-.568-.568c-2.257-.017-4.51.04-6.767-.02-.351-.022-.747.153-.77.55-.008 3.418.009 7.184-.004 10.557-.099.744.966.797.917.055v-4.11a1.56 1.56 0 0 1 .693-1.005c.963-.638 2.305.011 2.305 1.188 0 1.273.006 3.934.006 4.028 0 .223.169.425.39.452.31.003 2.496 0 3.257 0 .309 0 .541-.273.541-.568v-2.062l-.001-2.635c0-.199.033-.424-.052-.611zm-.873-2.041c0 .845-.733 1.484-1.579 1.49-.843.006-1.577-.547-1.577-1.49V8.57c0-.939.678-1.577 1.577-1.577.901 0 1.579.71 1.579 1.577v1.35zM8.939 15.403V13.13c.003-.59-.334-1.172-.821-1.485.495-.31.85-.859.821-1.457-.006-1.166.011-2.33-.006-3.496a.566.566 0 0 0-.57-.55c-2.283.012-4.566 0-6.851 0a.556.556 0 0 0-.55.53C.943 10.206.95 13.75.958 17.286c.002.288.249.559.554.552 2.175 0 4.675-.014 6.84-.002a.573.573 0 0 0 .585-.581c.01-.616-.003-1.236.002-1.853zm-.893.124c0 .832-.66 1.463-1.48 1.468-.82.006-1.59-.52-1.59-1.45v-1.98c0-1.053.717-1.573 1.59-1.573.876 0 1.48.698 1.48 1.554v1.981zm.028-5.74c-.028.853-.728 1.428-1.511 1.473-.856.05-1.583-.586-1.583-1.478V8.511c0-.929.707-1.554 1.583-1.554s1.511.701 1.511 1.56v1.27zm6.154 4.312c-.103-.052-.228-.036-.342-.035.002-.174-.004-.345.001-.518.007-.158.027-.355-.127-.454a.315.315 0 0 0-.159-.048c-.133-.004-.266.082-.293.22v.828a81.76 81.76 0 0 0-1.043-.007c-.319-.004-.414-.256-.419-.533.013-.341-.049-.702.044-1.036.064-.198.237-.266.433-.266h1.073c.16 0 .29-.132.29-.292a.295.295 0 0 0-.303-.289h-3.137c-.157 0-.321-.02-.443.1-.078.08-.097.18-.103.286-.004.688 0 1.38.005 2.067.02.135.136.238.275.238a.284.284 0 0 0-.27.211c-.023.918.003 1.844-.006 2.763 0 .142-.009.296.099.404.104.102.246.101.38.101 1.062.002 2.12-.019 3.181.013.231 0 .522-.067.522-.354v-2.853h.204c.294.003.397-.411.138-.546zm-.896 2.722c0 .18-.147.326-.326.326h-.74c-.256 0-.42-.232-.42-.416v-1.407c0-.292.097-.668.464-.668h1.022v2.165z"
        /> < title > { title } < / title > < / svg >
    }
}
