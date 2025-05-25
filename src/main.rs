use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const LOGO: Asset = asset!("/assets/icons/logo_tp.png");
const FAVICON: Asset = asset!("/assets/icons/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/css/tailwind_output.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        // Full screen container to center content both horizontally and vertically
        div {
            class: "min-h-screen bg-gray-50 flex flex-col items-center justify-center p-4 sm:p-6 lg:p-8",

            // Logo Container
            div {
                class: "mb-8",
                img {
                    src: LOGO,
                    alt: "Palegg Works Logo",
                    class: "h-32 sm:h-40 md:h-48 lg:h-56 xl:h-64 object-contain",
                }
            }

            // Branding Message Container
            div {
                class: "text-center max-w-2xl mb-12",

                h2 {
                    class: "text-3xl sm:text-4xl md:text-5xl font-extrabold text-gray-900 leading-tight tracking-tight",
                    "Palegg Works"
                }
                p {
                    class: "mt-4 text-xl sm:text-2xl md:text-3xl font-light text-gray-700 italic",
                    "Food for Simple Thoughts;"
                }
                p {
                    class: "mt-2 text-xl sm:text-2xl md:text-3xl font-light text-gray-700 italic",
                    "Apps for Mindful Lives"
                }
            }

            // Our Work Section
            div {
                class: "text-center mt-8",
                h3 {
                    class: "text-2xl sm:text-3xl md:text-4xl font-semibold text-gray-800 mb-4",
                    "Explore Our Work"
                }
                a {
                    href: "https://palegg-works.github.io/StayAhead/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition duration-150 ease-in-out",
                    "Stay Ahead - Habit Builder"
                    span { class: "ml-2", "→" }
                }
            }
        }
    }
}
