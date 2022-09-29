use yew::{
    function_component,
    html,
    Properties,
    Children
};
use yew_router::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct NavBarProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(NavBar)]
pub fn navbar(_props: &NavBarProps) -> Html {
    html! {
        <div>
            <nav class="sticky top-0 z-50">
                <div class="w-screen mx-auto px-4">
                    <div class="flex items-center justify-between h-16">
                        <span class="text-2xl text-gray-900 font-semibold">{"Logo here"}</span>
                        <div class="flex space-x-4 text-gray-900">
                            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                            <Link<Route> to={Route::Help}>{"Help"}</Link<Route>>
                        </div>
                    </div>
                </div>
            </nav>
        </div>
    }
}