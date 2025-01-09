use crate::router::Route;
use weblog::console_error;
use yew::prelude::*;

use crate::theme::{self, get_theme};

use crate::utils::{get_state, svg_asset};

use crate::{
    components::{lyrics::Lyrics, now_playing::NowPlaying},
    utils::{local::remove_local, reload_page},
};

#[derive(PartialEq, Properties)]
pub struct NavBarProps {}

#[function_component(NavBar)]
pub fn nav_bar(props: &NavBarProps) -> Html {
    let NavBarProps {} = props;

    // Route Handling
    let navigator = yew_router::prelude::use_navigator().unwrap();
    let create_route_callback = |route| {
        let navigator = navigator.clone();
        move |_| navigator.push(route)
    };

    let onclick_home = create_route_callback(&Route::Home);
    let onclick_clear_auth = {
        move |_| {
            remove_local("spotify-token");
            remove_local("spotify-state");
            get_state();
            reload_page();
        }
    };

    // Small screen dropdown handling
    let show_dropdown = use_state(|| true);
    let toggle_dropdown = {
        let show_dropdown = show_dropdown.clone();
        move |_| show_dropdown.set(!*show_dropdown)
    };

    // Theme handling
    let is_dark_mode = use_state(|| false);
    let toggle_theme = {
        let is_dark_mode = is_dark_mode.clone();
        move |_| {
            let toggle_result = theme::toggle_theme();
            if toggle_result.is_ok() {
                is_dark_mode.set(!*is_dark_mode);
            }
        }
    };

    use_effect_with((), {
        let is_dark_mode = is_dark_mode.clone();
        let t = get_theme();

        move |_| {
            if let Ok(t) = t {
                is_dark_mode.set(theme::Theme::Dark == t);
            } else {
                console_error!(t.err());
            }
        }
    });

    macro_rules! get_btn_class {
        ($route:expr) => {
            match navigator.basename() {
                Some(n) => match n {
                    $route => "nav-btn-active",
                    _ => "nav-btn",
                },
                None => "nav-btn",
            }
        };
    }

    html! {
        <nav class="bg-white border-gray-200 dark:bg-gray-900">
            <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
                <a href="/" class="flex items-center space-x-3 rtl:space-x-reverse">
                    <img src="/assets/favicon-32x32.png" class="h-8" alt="typify logo" />
                    <span
                        class="self-center text-2xl font-semibold whitespace-nowrap text-gray-800 dark:text-gray-100"
                    >
                        { "typify" }
                    </span>
                    <span
                        class="self-center text-2xl font-semibold whitespace-nowrap text-red-800 dark:text-red-100"
                    >
                        { "⚠️under development ⚠️" }
                    </span>
                </a>
                <button
                    data-collapse-toggle="navbar-default"
                    type="button"
                    onclick={toggle_dropdown}
                    class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-600 bg-gray-100 rounded-lg md:hidden hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-300 dark:text-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 dark:focus:ring-gray-500"
                    aria-controls="navbar-default"
                    aria-expanded="false"
                >
                    <span class="sr-only">{ "Open main menu" }</span>
                    <svg
                        class="w-5 h-5"
                        aria-hidden="true"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 17 14"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M1 1h15M1 7h15M1 13h15"
                        />
                    </svg>
                </button>
                <div
                    class={(if *show_dropdown { "hidden " } else { "" }).to_owned() +  "w-full md:block md:w-auto"}
                >
                    <ul
                        class="font-medium flex flex-col p-4 pt-5 md:p-0 mt-5 border border-gray-200 rounded-lg bg-gray-50 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700"
                    >
                        <li class="mt-2">
                            <a
                                href="#"
                                onclick={onclick_home}
                                class="text-gray-700 hover:text-blue-600 dark:text-gray-300 dark:hover:text-blue-400"
                                aria-current="page"
                            >
                                { "Home" }
                            </a>
                        </li>
                        <li class="bg-red-500 hover:bg-red-600 text-white rounded p-2">
                            <a href="#" onclick={onclick_clear_auth}>{ "Clear Auth" }</a>
                        </li>
                        <li>
                            <a href="https://github.com/PandeCode/" class="nav-btn">
                                <img
                                    src={svg_asset("github")}
                                    alt="Github"
                                    class="w-7 h-7 mb-4 rounded-full shadow-lg transform transition-transform duration-200 hover:scale-110"
                                />
                            </a>
                        </li>
                        <li>
                            <button
                                onclick={toggle_theme}
                                type="button"
                                class="text-gray-600 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 focus:outline-none focus:ring-4 focus:ring-gray-300 dark:focus:ring-gray-500 rounded-lg text-sm p-2"
                            >
                                <svg
                                    class={if *is_dark_mode {"hidden w-5 h-5"} else {"w-5 h-5"}}
                                    fill="currentColor"
                                    viewBox="0 0 20 20"
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <path
                                        d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"
                                    />
                                </svg>
                                <svg
                                    class={if *is_dark_mode {"w-5 h-5"} else {"hidden w-5 h-5"}}
                                    fill="currentColor"
                                    viewBox="0 0 20 20"
                                    xmlns="http://www.w3.org/2000/svg"
                                >
                                    <path
                                        d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z"
                                        fill-rule="evenodd"
                                        clip-rule="evenodd"
                                    />
                                </svg>
                            </button>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
