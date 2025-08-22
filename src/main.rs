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
    use_effect(move || {
        document::eval(
            r#"
            document.body.classList.add('loaded');
            "#
        );
    });

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

                div {
                    class: "flex items-center justify-center mb-4",
                    h3 {
                        class: "text-2xl sm:text-3xl md:text-4xl font-semibold text-gray-800 mr-3",
                        "Explore Our Work"
                    }
                    a {
                        href: "https://github.com/palegg-works",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "text-gray-600 hover:text-gray-900 transition-colors duration-200",
                        svg {
                            class: "w-8 h-8 sm:w-9 sm:h-9 md:w-10 md:h-10",
                            fill: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.082-.74.08-.725.08-.725 1.196.084 1.834 1.218 1.834 1.218 1.066 1.835 2.793 1.305 3.476.998.108-.77.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.046.138 3.003.404 2.293-1.552 3.3-.322 3.3-.322.651 1.652.24 2.873.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.922.43.369.812 1.102.812 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z"
                            }
                        }
                    }
                }

                a {
                    href: "https://palegg-works.github.io/StayAhead/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    //class: "inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition duration-150 ease-in-out",
                    class: "inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-teal-500 hover:bg-teal-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-teal-500 transition duration-150 ease-in-out",
                    "Stay Ahead - Habit Builder"
                    span { class: "ml-2", "→" }
                }
            }
        }
    }
}
