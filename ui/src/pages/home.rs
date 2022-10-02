use yew::{
    function_component,
    html
};
use crate::components::{
    NavBar,
    AppContainer
};

#[function_component(Home)]
pub fn home() -> Html {
    html!{
        <AppContainer>
            <NavBar/>
            <div class="grid grid-cols-2">
                <div>
                </div>
                <div>
                </div>
            </div>
        </AppContainer>
    }
}