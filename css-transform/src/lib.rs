use polyhorn::assets::ImageSource;
use polyhorn::prelude::*;
use std::time::Duration;
use tokio::time::sleep;
use yoyo::components::View as YoyoView;
use yoyo::yoyo;

pub struct App {}

yoyo!(States, {
    transition-transform: spring(100, 10, 1, false, false);

    :initial {
        transform: rotateX(45deg) rotate3d(0.0, 0.0, 1.0, 10deg);
    }

    .flip {
        transform: none;
    }
});

impl Component for App {
    fn render(&self, manager: &mut Manager) -> Element {
        let flip = use_state!(manager, false);

        let state = match &*flip.get(manager) {
            false => States::Initial,
            true => States::Flip,
        };

        // Turn the flip into a weak RC so we can use it asynchronously.
        let weak_flip = flip.weak(manager);
        use_async!(manager, async move {
            let mut value = false;

            loop {
                sleep(Duration::from_millis(1000)).await;
                value = !value;
                weak_flip.replace(value);
            }
        });

        poly!(<Window ...>
            <View style={ style! {
                align-items: center;
                justify-content: center;
                background-color: lightslategrey;
                height: 100%;
            } } ...>
                <YoyoView::<States> variant=state style={ style! {
                    align-items: center;
                    justify-content: center;
                    padding: 32px;
                    background-color: aliceblue;
                    border-radius: 4px;
                } } ...>
                    <Image source={ ImageSource::Asset(asset!("emoji-polyhorn")) } ... />
                </YoyoView::<States>>
            </View>
        </Window>)
    }
}

polyhorn::render!(<App />);
