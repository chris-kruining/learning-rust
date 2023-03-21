pub mod from_id;

use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    DueInDays(u8),
    DueToday,
    OverDue,
    Paid,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    Eur,
    Usd,
    Jpy,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Price {
    value: isize,
    currency: Currency,
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{}", self.currency, self.value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Invoice {
    pub id: u32,
    pub location: String,
    pub year: u16,
    pub state: State,
    pub price: Price,
}

#[server(GetInvoices, "/api")]
async fn loader() -> Result<Vec<Invoice>, ServerFnError> {
    Ok(vec![
        Invoice {
            id: 1,
            location: "Santa Monica".to_string(),
            year: 1995,
            state: State::OverDue,
            price: Price {
                value: 10800,
                currency: Currency::Usd,
            },
        },
        Invoice {
            id: 2,
            location: "Stankonia".to_string(),
            year: 2000,
            state: State::OverDue,
            price: Price {
                value: 10800,
                currency: Currency::Usd,
            },
        },
        Invoice {
            id: 3,
            location: "Ocean avenue".to_string(),
            year: 2003,
            state: State::Paid,
            price: Price {
                value: 8000,
                currency: Currency::Eur,
            },
        },
        Invoice {
            id: 4,
            location: "Tubthumper".to_string(),
            year: 1997,
            state: State::DueInDays(10),
            price: Price {
                value: 14000,
                currency: Currency::Jpy,
            },
        },
        Invoice {
            id: 5,
            location: "Wide open space".to_string(),
            year: 1998,
            state: State::DueInDays(8),
            price: Price {
                value: 4600,
                currency: Currency::Usd,
            },
        },
    ])
}

#[component]
pub fn Invoices(cx: Scope) -> impl IntoView {
    let invoices = create_resource(cx, move || (), move |_| loader());

    view! {
        cx,
        <h1>"Invoices!"</h1>

        <main class="invoices">
            <nav class="horizontal">
                <Suspense fallback=move || view! { cx, <p>"Loading..."</p> }>
                    <ErrorBoundary fallback=|cx, errors| view!{ cx, <p>"Errors"</p> }>{move ||
                        invoices.with(cx, |invoices| {
                            invoices.clone().map(|invoices| {
                                log!("invoices: {invoices:#?}");

                                view! {
                                    cx,

                                    <For
                                        each=move || invoices.clone().into_iter()
                                        key=|i| i.id
                                        view=move |cx, invoice: Invoice| {
                                            view! {
                                                cx,

                                                <InvoiceNavItem invoice={invoice} />
                                           }
                                        }
                                    />
                                }
                            })
                        })}
                    </ErrorBoundary>
                </Suspense>
            </nav>

            <Outlet />
        </main>
    }
}

#[component]
fn InvoiceNavItem(cx: Scope, invoice: Invoice) -> impl IntoView {
    let href = invoice.id.to_string();

    view! {
        cx,

        <A href={href}>
            <strong>{invoice.location}</strong>
            <strong>{invoice.price.to_string()}</strong>
            <p>{invoice.year.to_string()}</p>
            <p>{ move || format!("{:?}", invoice.state) }</p>
        </A>
    }
}
