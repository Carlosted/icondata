#[cfg(feature = "SiCodeceptjs")]
use leptos::*;
#[cfg(feature = "SiCodeceptjs")]
///This icon requires the feature `SiCodeceptjs` to be enabled.
#[component]
pub fn Codeceptjs(
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
        "M15.3986.597c-1.424 0-1.5473.0048-1.59.0807-.0238.0475-1.0206 2.6485-2.212 5.7811-1.196 3.1326-2.1737 5.7005-2.1832 5.71-.0048.0047-1.0348-1.1534-2.2831-2.5726C5.1937 7.394 4.852 7.0236 4.8283 7.1138c-.0424.1883-.0657.3492-.095.5412l2.7815 3.6262c1.5283 1.9935 2.7956 3.631 2.8146 3.6358.0285.0095 6.474-13.9308 6.588-14.2393.0285-.076-.0664-.0807-1.5188-.0807zM7.6828 2.1692a8.7326 8.7326 0 0 0-.88.0463c-1.2483.1282-2.4871.5696-3.5218 1.2579-.5554.3654-1.5142 1.329-1.9034 1.9033-2.1311 3.161-1.7561 7.3758.8924 9.9816.6028.5933 1.2008 1.0158 2.041 1.4382 1.03.5221 2.0694.8259 3.4174 1.0157 1.0062.1377 2.9238.1092 4.1199-.0616l.1993-.0285v-.6692l-.1756.0284c-1.0727.1709-3.0994.1614-4.0962-.0143-2.601-.4604-4.5091-1.7609-5.577-3.8066-.9256-1.7704-1.049-4.0486-.3228-5.9947.375-1.0157.8781-1.7989 1.647-2.563.7974-.7974 1.5664-1.2769 2.5631-1.6091 1.3148-.4367 2.9475-.4699 4.4901-.095.2658.0618.489.114.4937.114.0284 0 .185-.3797.166-.3987-.0427-.038-1.0015-.2658-1.4903-.356a11.0808 11.0808 0 0 0-2.0628-.1887zm10.6834.792c-.0283-.0042.0274.0476.184.1846 2.1074 1.8321 3.3605 4.0108 3.8019 6.6118.1424.8449.1566 2.3922.0284 3.2275-.3654 2.3495-1.3859 4.3857-3.0471 6.0565-1.8559 1.87-4.101 2.9665-6.8112 3.3225-.8306.1091-2.5345.0712-3.37-.0712-.9777-.1662-1.9697-.4747-3.0139-.9256-.5743-.2515.432.3892 1.2863.8164 2.5298 1.2673 5.4679 1.5568 8.2588.8211.731-.1946 1.3527-.432 2.1311-.8116 3.4602-1.6993 5.7574-4.979 6.1513-8.7762.0665-.655.0333-1.9175-.0759-2.5915-.4746-3.0282-2.2213-5.7337-4.8176-7.4471-.2468-.1661-.5268-.337-.617-.3797-.046-.0223-.0762-.0357-.089-.0376zM16.271 8.1145c-.2641-.0026-.5068.0126-.6967.0482-.8544.1661-1.4382.674-1.609 1.4002-.1804.75.0427 1.4192.5837 1.7657.318.1993.6028.2942 1.462.4888.9018.2041 1.2103.3418 1.3907.6123.1044.1567.128.2421.1233.4794 0 .1709-.0332.3512-.0806.4367-.1092.2135-.413.4556-.6883.5553-.5363.1946-1.5473.185-2.3162-.019-.6598-.1756-.5886-.1993-.5886.1899 0 .2943.0143.3417.1092.4034.3845.2563 1.8986.3893 2.7387.2469.522-.0902 1.03-.3134 1.3242-.5744.1044-.0997.2611-.318.3465-.4889.1472-.2942.1567-.3464.1567-.8163 0-.451-.0143-.5221-.133-.769-.2658-.5458-.7309-.7879-2.0029-1.0584-.8686-.1804-1.234-.375-1.405-.7547-.223-.4936.0048-1.03.5364-1.2625.4841-.2089 1.533-.171 2.4017.0901l.2942.0902V8.438l-.2278-.076c-.4535-.15-1.1382-.2418-1.7192-.2475z"
        /> < title > { title } < / title > < / svg >
    }
}
