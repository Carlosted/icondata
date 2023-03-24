#[cfg(feature = "SiApachecassandra")]
use leptos::*;
#[cfg(feature = "SiApachecassandra")]
///This icon requires the feature `SiApachecassandra` to be enabled.
#[component]
pub fn Apachecassandra(
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
        "M17.514 5.766c-.002.003-.247.256-.387.408-.366.397-.92.975-1.312 1.12h-.004a2.28 2.28 0 00-.16-.05l.705-1.425-.008.012c-.022.043-.72 1.415-1.666 1.218l-.002.002c-.014-.003-.026.002-.04-.002.455-.58.63-1.113.63-1.113s-.756 1.213-1.69.885a.075.075 0 01-.05-.047.153.153 0 010-.08c.01-.04.04-.092.067-.14.095-.176.256-.368.256-.368s-.156.152-.366.295l-.002.002c-.157.107-.345.208-.52.236a.512.512 0 01-.167 0c-.214-.037-.208-.25-.149-.455v-.002c.06-.204.174-.4.174-.4s-.19.298-.398.588c-.16.223-.311.396-.416.474l-.12.004c-.063-.176.1-.756.1-.76a4.984 4.984 0 01-.314.446 1.73 1.73 0 01-.34.336h-.027c-.082.007-.16.02-.24.031a.208.208 0 01-.063-.125c-.042-.268.123-.756.125-.762-.002.006-.09.26-.21.502V6.6c-.032.067-.07.12-.107.178-.086.136-.176.244-.261.22-.127-.039-.202-.203-.246-.357-.044-.154-.057-.299-.057-.299s-.031.251-.117.485a.8.8 0 01-.146.265c-.024.027-.057.036-.086.055a12.553 12.553 0 00-2.617.764c-.223.082-.446.159-.674.257-.75.32-1.53.726-2.344 1.248a21.335 21.335 0 00-2.56 1.945c-.449.393-.907.82-1.377 1.29C3.887 10.735 5.3 7.908 11.964 7.656c5.045-.19 7.402 2.138 8.18 2.171 1.957.083 3.258-1.4 3.673-1.947.112-.142.179-.237.183-.244-.01.011-.328.374-.781.722l-.006.004c-.46.35-1.05.68-1.577.6h-.002c-.068-.01-.138-.007-.203-.032.744-.434 1.237-1.19 1.483-1.596.076-.125.233-.36.233-.36S21.4 8.762 20.099 8.642l-.002.002c-.051-.005-.105.007-.155-.004.567-.446.985-1.045 1.278-1.55v-.004h.002c.317-.548.482-.985.484-.992-.005.008-.326.498-.766 1.037l-.01.01c-.44.54-.998 1.122-1.464 1.3-.07.026-.14.066-.205.072 1.085-.96 1.54-2.323 1.54-2.323s-1.098 1.595-2.027 1.89c-.056.017-.114.047-.17.054.568-.513.869-.92 1.057-1.24V6.89c.238-.404.281-.65.281-.65l-.004.004a9.381 9.381 0 01-.271.334l-.016.017c-.255.3-.645.733-1.045 1.078l-.005.004a4.148 4.148 0 01-.32.248c-.03.021-.06.046-.09.065a1.7 1.7 0 01-.273.139c-.022.01-.044.022-.065.03-.068-.045-.132-.092-.205-.135-.046.06-.096.117-.142.176h-.002c-.02-.006-.044-.006-.063-.016.859-.935 1.44-2.12 1.44-2.12l-.003.004c-.008.01-1.194 1.61-1.73 1.71-.07-.035-.14-.07-.216-.103a.5.5 0 01.044-.162c.122-.284.465-.717.465-.717s-.093.07-.123.096a4.251 4.251 0 01-.304.238c-.056.04-.114.072-.172.108a1.9 1.9 0 01-.188.105.71.71 0 01-.263.086c-.043 0-.082-.005-.11-.027.453-.277 1.228-1.628 1.233-1.637zm-4.79 2.166a15.085 15.085 0 00-.914.006c-.286.01-.55.033-.817.053l-.002.002a3.41 3.41 0 00-.783.46.427.427 0 01.035.168.431.431 0 01-.047.19l.885.642.022-.014-.36-1.264.623 1.114c.02-.008.042-.014.063-.02l-.02-1.236.397 1.162.476-1.15.018 1.269c.004.002.007.006.012.008l.712-1.006-.363 1.219.008.01 1.041-.692-.758 1.05v.007l.95-.34c.013-.017.03-.03.042-.047a.394.394 0 01.396-.63c.119-.245.209-.504.268-.77a13.153 13.153 0 00-1.884-.19zm-2.508.134l-.02.002c-.309.036-.607.078-.893.125-.287.047-.562.1-.824.16l-.014.002c.115.388.294.756.531 1.08.108-.24.244-.461.402-.662a.436.436 0 01.57-.557c.077-.058.166-.1.248-.15zm4.46.092a3.184 3.184 0 01-.162.826.394.394 0 01-.328.68 3.172 3.172 0 01-.55.63l.66.034-.994.236c-.026.018-.046.042-.072.06a3.195 3.195 0 011.538.69c.32-.574.504-1.235.504-1.94 0-.535-.237-.928-.594-1.216zm-6.319.226c-.296.07-.58.147-.85.23a1.462 1.462 0 00-.212.76 3.99 3.99 0 002.357 3.64l.002-.011a3.198 3.198 0 01-.58-.79.572.572 0 01-.336-1.079 3.18 3.18 0 01.14-1.39 3.16 3.16 0 01-.52-1.36zm7.633.026c.14.41.225.842.225 1.293 0 2.44-2.203 4.419-4.921 4.419-2.718 0-4.921-1.98-4.921-4.42 0-.235.023-.467.063-.693A13.08 13.08 0 004.36 10.12c-.214.388-.318.793-.283 1.2.085.986.951 1.808 2.275 2.365l-.256-.068c-2.291-.646-4.109.503-6.096-.494.46.364.984.717 2.499.722.513.002 2.158-.08 2.461.186.303.265-1.021 1.514-1.021 1.514s2.24-1.897 2.459-1.25c.137.402-.643 1.476-.643 1.476s.784-1.06 1.364-1.287c.4-.156.735-.136 1.022.266.19.265-.91 1.553-.91 1.553s1.477-1.44 1.741-1.364c.266.076 0 1.364 0 1.364s.545-1.307.835-1.403c.398-.133-.909 3.031-.909 3.031s1.596-2.926 1.894-2.992c.51-.114.852 2.044.852 2.044s-.25-1.886.02-2.007c1.694-.767.718 3.257.718 3.257s1.11-2.73.428-3.337c1.799 1.193 1.579 3.337 1.579 3.337s.472-1.02-.855-3.53c.72-.038 1.8 1.864 1.8 1.864s-1.172-2.082-.377-2.12c1.58-.075 1.74 2.765 1.74 2.765s.474-.398-.795-3.181c.808-.518 2.765 2.575 2.765 2.575s-1.805-3.018-1.515-3.22c.29-.202 1.35.96 1.35.96s-.808-1.11-.594-1.223c.216-.114 2.363 2.207 2.363 2.207s-1.895-2.271-1.567-2.55c.328-.277 1.717.784 1.717.784s-2.134-1.4-1.717-1.618c.58-.3 2.235.72 2.235.72s-.996-.82-.832-1.02c.164-.203 2.12 1.287 2.12 1.287s-1.729-1.365-1.818-1.706c-.088-.34 1.362.115 1.362.115s-1.602-.744-1.627-.959c-.026-.214 1.06.24 1.06.24s-1.49-1.225-1.855-.228c-.084.16-.187.3-.283.45.093-.27.135-.543.11-.82a1.87 1.87 0 00-.052-.28c-.683-.367-1.702-.933-3.134-1.336zm-5.831.455a.427.427 0 01-.509.164c-.054.073-.1.152-.148.23l1.278.47c.01-.013.02-.03.032-.042zM9.5 9.26c-.094.155-.18.316-.244.489.12.128.26.235.396.343l.923-.029.002-.004zm4.187.541l-.606.463.053.002c.201-.132.383-.29.553-.465zm-4.535.291c-.002.012-.007.023-.01.035a2.88 2.88 0 00-.048.958.567.567 0 01.451.474l.938-.6c-.085-.03-.17-.06-.251-.097l-1.052-.076.638-.139a3.199 3.199 0 01-.666-.555zm.793.211c.139.086.28.16.43.223l.116-.024c0-.019.005-.037.006-.056zm3.121.68l.916.9-1.198-.466.703 1.079-1.072-.833-.012.006.346 1.28-.596-1.136-.097 1.33-.403-1.326-.47 1.263.113-1.36-.016-.008-.812 1.153.297-1.11a3.299 3.299 0 00-.793 1.19c.095.102.196.198.302.289a3.985 3.985 0 004.353-1.689 3.399 3.399 0 00-1.26-.539zm-2.436.223l-1.079.39c.001.018.01.033.01.051a.57.57 0 01-.184.42c.102.218.228.424.375.616a3.2 3.2 0 01.32-.635l-.295.239zm3.634 2.791c-.186.049-.37.097-.56.137.191-.04.374-.09.56-.136zm-1.089.235c-.114.02-.225.046-.34.063-.202.03-.39.042-.586.062.315-.033.624-.074.926-.125zm-1.35.17c-.11.008-.215.007-.324.012.11-.007.214-.003.324-.012z"
        /> < title > { title } < / title > < / svg >
    }
}
