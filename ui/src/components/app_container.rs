use yew::{
    function_component,
    html,
    Properties,
    Children
};

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AppContainer)]
pub fn app_container(props: &ContainerProps) -> Html {
    html! {
        <div class="grid grid-cols-1 h-max">
            <div class="z-10 absolute">
                {for props.children.iter() }
            </div>
        </div>
    }
}

