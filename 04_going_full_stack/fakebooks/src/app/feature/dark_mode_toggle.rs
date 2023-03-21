use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::{Meta, MetaProps};
use leptos_router::{ActionForm, ActionFormProps};

cfg_if! {
    if #[cfg(feature="ssr")] {
        fn initial_prefers_dark(cx: Scope) -> bool {
            use_context::<actix_web::HttpRequest>(cx)
                .and_then(|req| {
                    req.cookies().map(|cookies| {
                        cookies
                            .iter()
                            .any(|cookie| cookie.name() == "darkmode" && cookie.value() == "true")
                    }).ok()
                })
                .unwrap_or(false)
        }
    }
    else {
        fn initial_prefers_dark(_cx: Scope) -> bool {
            use wasm_bindgen::JsCast;

            let doc = document().dyn_into::<web_sys::HtmlDocument>().unwrap();
            let cookie = doc.cookie().unwrap_or_default();

            cookie.contains("darkmode=true")
        }
    }
}
#[server(ToggleDarkMode, "/api")]
pub async fn toggle_dark_mode(cx: Scope, prefers_dark: bool) -> Result<bool, ServerFnError> {
    use actix_web::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
    use leptos_actix::{ResponseOptions, ResponseParts};

    let response =
        use_context::<ResponseOptions>(cx).expect("to have leptos_actix::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("darkmode={prefers_dark}; Path=/"))
            .expect("to create header value"),
    );
    response_parts.headers = headers;

    std::thread::sleep(std::time::Duration::from_millis(250));

    response.overwrite(response_parts);
    Ok(prefers_dark)
}

#[component]
pub fn DarkModeToggle(cx: Scope) -> impl IntoView {
    let initial = initial_prefers_dark(cx);

    let toggle_dark_mode_action = create_server_action::<ToggleDarkMode>(cx);
    let input = toggle_dark_mode_action.input();
    let value = toggle_dark_mode_action.value();

    let prefers_dark = move || match (input(), value()) {
        (Some(submission), _) => submission.prefers_dark,
        (_, Some(Ok(value))) => value,
        _ => initial,
    };

    let color_scheme = move || {
        if prefers_dark() {
            "dark".to_string()
        } else {
            "light".to_string()
        }
    };

    view! {
        cx,

        <Meta name="color-scheme" content=color_scheme />

        <ActionForm action=toggle_dark_mode_action>
            <input type="hidden" name="prefers_dark" value=move || (!prefers_dark()).to_string() />
            <button type="submit">"Toggle"</button>
        </ActionForm>
    }
}
