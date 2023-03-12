use uuid::Uuid;
use yew::prelude::*;

#[cfg(feature = "ssr")]
async fn loader() -> Uuid {
    Uuid::default()
}

#[function_component]
pub fn Index() -> HtmlResult {
    let uuid = use_prepared_state!(async move |_| -> Uuid { loader().await }, ())?.unwrap();

    Ok(html! {
        <div>{"Some pretty html and stuff"}{uuid}</div>
    })
}