use polyhorn::assets::ImageSource;
use polyhorn::prelude::*;

#[derive(Default)]
pub struct App {}

impl Component for App {
    fn render(&self, manager: &mut Manager) -> Element {
        let source = use_reference!(manager, ImageSource::placeholder(96.0, 96.0));

        let weak_source = source.weak(manager);
        use_async!(manager, async move {
            let bytes = reqwest::get("https://polyhorn.com/img/emoji-polyhorn.png")
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap();

            weak_source.replace(bytes);
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
