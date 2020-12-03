use polyhorn::prelude::*;

#[derive(Default)]
pub struct Root {}

impl Component for Root {
    fn render(&self, _manager: &mut Manager) -> Element {
        let bytes = include_bytes!("../assets/emoji-polyhorn.png");

        poly!(<View style=!{
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
                <Image source={ &bytes[..] } />
            </View>
        </View>)
    }
}

#[derive(Default)]
pub struct App {}

impl Component for App {
    fn render(&self, _manager: &mut Manager) -> Element {
        poly!(<Window><Root /></Window>)
    }
}

polyhorn::render!(<App />);

#[cfg(test)]
mod tests {
    use super::*;

    #[polyhorn::test]
    async fn test_app(automator: &mut polyhorn_test::Automator) {
        automator.wait(std::time::Duration::from_secs(5)).await;
        automator.render(|| poly!(<Root />)).await;
        automator.snapshot("example").await;
    }
}
