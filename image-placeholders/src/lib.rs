use polyhorn::assets::ImageSource;
use polyhorn::prelude::*;
use std::time::Duration;
use tokio::time::delay_for;

#[derive(Default)]
pub struct App {}

impl Component for App {
    fn render(&self, manager: &mut Manager) -> Element {
        let source = use_reference!(manager, ImageSource::placeholder(96.0, 96.0));

        let weak_source = source.weak(manager);
        use_async!(manager, async move {
            delay_for(Duration::from_millis(1000)).await;
            weak_source.replace(asset!("emoji-polyhorn"));
            weak_source.queue_rerender();
        });

        poly!(<Window>
            <View style=!{
                align-items: center;
                justify-content: center;
                background-color: lightslategrey;
                height: 100%;
            }>
                <View style=!{
                    align-items: center;
                    justify-content: center;
                    padding: 32px;
                    background-color: aliceblue;
                    border-radius: 4px;
                }>
                    <Image source={ source.cloned(manager) } />
                </View>
            </View>
        </Window>)
    }
}

polyhorn::render!(<App />);
