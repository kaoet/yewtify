use yew::prelude::*;
use yewtify as y;

pub struct YouTubeLayout {}

pub enum Msg {}

impl Component for YouTubeLayout {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <y::App id="inspire">
                <y::NavigationDrawer app=true clipped=true>
                    <y::List dense=true>
                        <y::ListItem link=true>
                            <y::ListItemAction>
                                <y::ListItemIcon icon=y::MdiIcon::TrendingUp />
                            </y::ListItemAction>
                            <y::ListItemContent>
                                <y::ListItemTitle>
                                </y::ListItemTitle>
                            </y::ListItemContent>
                        </y::ListItem>
                        <y::SubHeader>{ "SUBSCRIPTIONS" }</y::SubHeader>
                    </y::List>
                </y::NavigationDrawer>
                <y::AppBar color=y::Color::Red>
                    <y::AppBarNavIcon />
                    <y::Icon large=true icon=y::MdiIcon::Youtube />
                </y::AppBar>
                <div>
                    <y::Btn small=true text=true>{"Small Text"}</y::Btn>
                    <y::Btn text=true>{"Normal Text"}</y::Btn>
                    <y::Btn>{"Default"}</y::Btn>
                    <y::Btn depressed=true>{"Depressed"}</y::Btn>
                    <y::Btn icon=true><y::Icon icon=y::MdiIcon::Heart /></y::Btn>
                    <y::Btn outlined=true fab=true><y::Icon icon=y::MdiIcon::FormatListBulleted /></y::Btn>
                    <y::Btn fixed=true>{"Fixed"}</y::Btn>
                    <y::Btn tile=true>{"Tile"}</y::Btn>
                    <y::Btn block=true>{"Block"}</y::Btn>
                    <y::Btn loading=true>{"Loading"}</y::Btn>
                </div>
            </y::App>
        }
    }
}
