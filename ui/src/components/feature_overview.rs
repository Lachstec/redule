use yew::{
    function_component,
    html,
};

#[function_component(FeatureOverview)]
pub fn feature_overview() -> Html {
    html! {
        <div>
            <h2 class="text-sky-500 text-2xl">{ "Features" }</h2>
            <p class="text-4xl">{" Meetup Scheduling made simple "}</p>
            <div class="grid grid-cols-2 gap-12 pt-12">
                <div class="col-span-2 md:col-span-1">
                    <h3 class="heading-sm flex items-center font-bold text-xl pb-2">
                        { "Scheduling" }
                    </h3>
                    <p class="opacity-50">
                        {" Pick possible Dates for a Meetup and share a link to the poll. Your participants can then chose, on which Dates they are available, telling you the ideal Date for your Meetup."}
                    </p>
                </div>

                <div class="col-span-2 md:col-span-1">
                    <h3 class="heading-sm flex items-center font-bold text-xl pb-2">
                        { "Comments" }
                    </h3>
                    <p class="opacity-50">
                        {" Let people comment on your Poll, giving you valuable Feedback on your proposed Dates."}
                    </p>
                </div>

                <div class="col-span-2 md:col-span-1">
                    <h3 class="heading-sm flex items-center font-bold text-xl pb-2">
                        { "Notifications" }
                    </h3>
                    <p class="opacity-50">
                        {" Get notified via E-Mail when someone votes on a Date or leaves you a comment on your poll."}
                    </p>
                </div>

                <div class="col-span-2 md:col-span-1">
                    <h3 class="heading-sm flex items-center font-bold text-xl pb-2">
                        { "No Login required" }
                    </h3>
                    <p class="opacity-50">
                        {" Create Polls without the need to create an Account in the first place, making Redule easy to use from start to end."}
                    </p>
                </div>

            </div>
        </div>
    }
}