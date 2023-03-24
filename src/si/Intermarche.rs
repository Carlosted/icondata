#[cfg(feature = "SiIntermarche")]
use leptos::*;
#[cfg(feature = "SiIntermarche")]
///This icon requires the feature `SiIntermarche` to be enabled.
#[component]
pub fn Intermarche(
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
        "m9.948 13.05.677 1.18h12.782l.15-1.18zm1.961-2.556a3.209 3.209 0 0 0-.694.071 1.72 1.72 0 0 0-.537-.07 2.699 2.699 0 0 0-1.007.168l-.215 1.53.317.552h.384l.254-1.834a.648.648 0 0 1 .2-.03c.17 0 .229.086.23.192l-.234 1.672h.782l.215-1.525a1 1 0 0 0 .008-.093l.03-.214a.646.646 0 0 1 .201-.029c.197 0 .246.115.227.25l-.22 1.608h.778l.215-1.524c.062-.442-.185-.72-.93-.72M8.88 11.88a.725.725 0 0 0 .438-.593l.012-.078h-.004c.062-.442-.185-.72-.93-.72a2.699 2.699 0 0 0-1.007.17v.003L7.1 12.741h.778l.104-.74h.2l1.466 2c.097.13.219.23.375.23h.208l-1.355-2.352zm-.32-.755-.01.084-.025.165a.27.27 0 0 1-.3.247h-.19l.01-.078.09-.638a.64.64 0 0 1 .202-.03c.197 0 .247.114.227.25zm-3.84 1.503c.014-.102-.023-.174-.145-.174h-.09c-.197 0-.247-.114-.228-.25l.165-1.178v-.015l.01-.078h.21a.058.058 0 0 1 .02 0c.113 0 .207-.115.231-.289H4.47l.122-.871c-.558.061-.815.32-.871.705l-.066.434-.006.042-.004.03-.14 1.04c-.061.432.17.707.876.72h.322l.016-.114m1.416-2.14a2.699 2.699 0 0 0-1.007.17l-.119.846-.068.509c-.06.43.175.705.877.718h.853l.015-.102c.015-.103-.021-.186-.143-.186H5.92c-.197 0-.246-.114-.227-.25l.025-.186h.244c.714-.01 1.03-.283 1.092-.713l.012-.079h-.004c.06-.44-.187-.719-.931-.719zm.166.642-.011.085-.027.155a.27.27 0 0 1-.3.247h-.191l.01-.078.09-.638a.642.642 0 0 1 .202-.029c.196 0 .245.115.226.25zm16.764-.642a2.696 2.696 0 0 0-1.005.17l-.12.846-.068.509c-.06.43.175.705.876.718h.854l.015-.103c.015-.103-.022-.185-.144-.185h-.625c-.196 0-.246-.114-.226-.25l.025-.186h.244c.713-.01 1.03-.283 1.092-.713l.012-.079h-.004c.061-.44-.186-.719-.93-.719zm.163.642-.01.085-.025.164a.271.271 0 0 1-.3.247h-.191l.01-.077.09-.639a.643.643 0 0 1 .202-.028c.196-.01.246.105.226.24zm-9.025-.632h-.665c-.116 0-.208.115-.233.29h.794c.197 0 .246.121.227.253l-.034.247h-.258c-.703.013-1.017.284-1.078.713l-.012.078h.007c-.06.44.187.719.933.719.342.008.684-.05 1.005-.17l.12-.847.075-.564c.06-.43-.175-.713-.876-.726zm.024 1.245-.09.645a.642.642 0 0 1-.201.03c-.196 0-.245-.115-.227-.25l.01-.085.026-.165a.27.27 0 0 1 .298-.248h.192l-.01.077zm3.16-.456.012-.079h.005c.06-.44-.187-.718-.93-.718a2.698 2.698 0 0 0-1.007.17v.003l-.293 2.08h.778l.104-.74h.198l.437.739h.778l-.513-.862a.725.725 0 0 0 .438-.593zm-.747-.165-.01.085-.039.163a.27.27 0 0 1-.3.247h-.19l.01-.078.09-.638a.64.64 0 0 1 .201-.029c.201.001.25.115.232.25zm2.604 1.33.018-.132a3.28 3.28 0 0 1-.647.073h-.06c-.196 0-.245-.115-.227-.254l.065-.479.018-.136.07-.477a.27.27 0 0 1 .296-.254h.06c.21-.002.42.02.624.067l.018-.137c.016-.114.027-.207-.127-.236a3.265 3.265 0 0 0-.537-.037c-.744 0-1.07.277-1.13.718l-.117.835c-.06.44.188.719.932.719.194 0 .388-.015.58-.048.085-.02.14-.058.165-.236m1.486-1.946c-.072 0-.143.002-.21.006l.1-.73c-.558.061-.815.32-.87.706l-.071.507-.247 1.76h.778l.254-1.834a.646.646 0 0 1 .2-.029c.197 0 .246.114.227.25l-.227 1.613h.778l.216-1.524c.062-.442-.186-.72-.932-.72M.865 11.51l.007-.055.243-1.684h-.78l-.27 1.843v.007l-.056.424c-.054.385.128.646.67.706l.18-1.239m22.771-1.53h-1.07l-.042.29h.878c.115 0 .208-.116.233-.29M2.274 12.741l.227-1.613c.019-.135-.03-.25-.227-.25a.64.64 0 0 0-.2.03l-.254 1.833h-.78l.293-2.079a2.69 2.69 0 0 1 1.006-.17c.745 0 .992.28.93.72l-.214 1.525h-.778"
        /> < title > { title } < / title > < / svg >
    }
}
