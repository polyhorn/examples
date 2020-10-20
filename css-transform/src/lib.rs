use polyhorn::assets::ImageSource;
use polyhorn::prelude::*;
use polyhorn::styles::ViewStyle;

pub struct App {}

impl Component for App {
    fn render(&self, _manager: &mut Manager) -> Element {
        poly!(<Window ...>
            <View style={ style! {
                align-items: center;
                justify-content: center;
                background-color: lightslategrey;
                height: 100%;
            } } ...>
                <View style={ style! {
                    align-items: center;
                    justify-content: center;
                    padding: 32px;
                    background-color: aliceblue;
                    border-radius: 4px;
                    transform: perspective(500px) rotateX(45deg) rotate3d(0.0, 0.0, 1.0, 10deg);
                } } ...>
                    <Image source={ ImageSource::Asset(asset!("emoji-polyhorn")) } ... />
                </View>
            </View>
        </Window>)
    }
}

polyhorn::render!(<App />);
