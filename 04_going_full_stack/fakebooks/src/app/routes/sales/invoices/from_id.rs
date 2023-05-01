use crate::app::routes::sales::invoices::Invoice as InvoiceModel;
use leptos::*;
use leptos_router::*;
use serde::{Serialize, Deserialize};
use http::status::StatusCode;
use thiserror::Error;

#[server(GetInvoice, "/api")]
pub async fn get_invoice(id: u32) -> Result<Option<InvoiceModel>, ServerFnError> {
    if id > 5 {
        return Ok(None);
    }

    Ok(Some(InvoiceModel { id,   }))
            location: "Santa Monica".to_string(),
            year: 1995,
            state: State::OverDue,
            price: Price {
                value: 10800,
                currency: Currency::Usd,
            },
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum InvoiceError {
    #[error("Invalid invoice id")]
    InvalidId,
    #[error("Invoice not found")]
    NotFound,
    #[error("Server error.")]
    ServerError,
}

impl InvoiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            InvoiceError::InvalidId => StatusCode::BAD_REQUEST,
            InvoiceError::NotFound => StatusCode::NOT_FOUND,
            InvoiceError::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Params, PartialEq, Eq, Copy, Clone, Debug)]
pub struct InvoiceParams {
    id: u32,
}

#[component]
pub fn Invoice(cx: Scope) -> impl IntoView {
    let params = use_params::<InvoiceParams>(cx);
    let id = move || params.with(|q| {
        q.as_ref().map(|q| q.id).map_err(|_| InvoiceError::InvalidId)
    });

    let invoice = create_resource(cx, id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_invoice(id)
                .await
                .map(|i| i.ok_or(InvoiceError::NotFound))
                .map_err(|_| InvoiceError::ServerError)
                .flatten(),
        }
    });

    let invoice_view = move || {
        invoice.with(cx, |invoice| {
            invoice.clone().map(|invoice| {
                view! {
                    cx,

                    <p>{invoice.id.to_string()}</p>
                }
            })
        })
    };

    view! {
        cx,

        <div class="invoice">
            <Suspense fallback=move || view! { cx, <p>"Loading"</p> }>
                <ErrorBoundary fallback=|cx, errors| view!{ cx, <InvoiceErrorTemplate errors=errors /> }>
                    {invoice_view}
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}

#[component]
fn InvoiceErrorTemplate(cx: Scope, errors: RwSignal<Errors>) -> impl IntoView {
    let errors: Vec<InvoiceError> = errors()
        .into_iter()
        .filter_map(|(_, error)| error.downcast_ref::<InvoiceError>().cloned())
        .collect();

    view! {
        cx,

        <h1>{ if errors.len() > 1 {"Errors"} else {"Error"} }</h1>
        <For
            each=move || {errors.clone().into_iter().enumerate()}
            key=|(index, _)| *index
            view=move |cx, error| {
                let code = error.1.status_code().to_string();
                let error = error.1.to_string();
                view! {
                    cx,

                    <div class="error">{code} ": " {error}</div>
                }
            }
        />
    }
}
