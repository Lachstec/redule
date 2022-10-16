use yew::{
    function_component,
    html,
    classes
};

use yew_router::prelude::*;

use crate::components::{
    NavBar,
    AppContainer
};

use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html!{
        <AppContainer>
            <NavBar/>
            <div class="grid grid-cols-2">
                <div class="my-24 text-center lg:text-left pl-24">
                    <h1 class="text-4xl font-bold sm:text-5xl">
                        {"Schedule"}
                        <br/>
                        <span class="whitespace-nowrap text-sky-500 italic">{"Meetups"}</span>
                        <br/>
                        {"the easy way."}
                        <div class=" mt-1 text-xl text-gray-400">{"Quickly find the perfect Day and Time"}</div>
                        <div class="space-x-3">
                            <Link<Route> classes={classes!("rounded", "text-lg", "bg-sky-500", "px-5", "py-3", "font-semibold", "text-white", "shadow-sm")} to={Route::NewPoll}> {"Start Planning"}</Link<Route>>
                            <Link<Route> classes={classes!("rounded", "text-lg", "bg-gray-500", "px-5", "py-3", "font-semibold", "text-white", "shadow-sm")} to={Route::Help}> {"Getting Started"}</Link<Route>>
                        </div>
                    </h1>                    
                </div>
                <div>
                </div>
            </div>
        </AppContainer>
    }
}