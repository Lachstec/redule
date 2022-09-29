use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod components;

use components::AppContainer;

#[derive(PartialEq, Clone, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// switch handles the primary Routing for the frontend.
/// 
/// It matches on the Route enum and returns the corresponding Page
fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html!(
                <AppContainer>
                    <p>{"Hello, World!"}</p>
                </AppContainer>
            )
        },
        Route::NotFound => {
            html!(
                <AppContainer>
                </AppContainer>
            )
        }
    }
}

struct App;

impl Component for App {

    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <BrowserRouter>
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        )
    }
}

fn main() {
    yew::start_app::<App>();
}
