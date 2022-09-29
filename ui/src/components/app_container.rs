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
        <div class="grid grid-cols-1">
            <div class="bg-[url('../../assets/wiggle.svg')] opacity-10 h-screen relative z-0"></div>
            <div class="z-10 absolute">
                {for props.children.iter() }
            </div>
        </div>
    }
}

