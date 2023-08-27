use super::theme;
use leptos::*;
use styled::style;
use stylist::Style as Styles;

#[derive(PartialEq)]
pub enum Variant {
    PRIMARY,
    SECONDARY,
    ALERT,
    DISABLED,
}

impl Variant {
    pub fn is(&self, variant: &Variant) -> bool {
        self == variant
    }
}

struct ButtonColors {
    text: String,
    background: String,
    border: String,
}

fn get_colors(variant: &Variant) -> ButtonColors {
    let theme = theme::get_theme().unwrap();
    match variant {
        Variant::PRIMARY => ButtonColors {
            text: theme.white(),
            background: theme.purple(),
            border: theme.transparent(),
        },
        Variant::SECONDARY => ButtonColors {
            text: theme.black(),
            background: theme.teal(),
            border: theme.gray.lightest(),
        },
        Variant::ALERT => ButtonColors {
            text: theme.white(),
            background: theme.red(),
            border: theme.transparent(),
        },
        Variant::DISABLED => ButtonColors {
            text: theme.white(),
            background: theme.red(),
            border: theme.transparent(),
        },
    }
}

#[component]
pub fn Button(cx: Scope, variant: Variant) -> impl IntoView {
    let disabled = variant.is(&Variant::DISABLED);

    let styles = styles(&variant);

    styled::view! {
        cx,
        styles,
        <button disabled=disabled>"Button"</button>
    }
}

#[allow(non_upper_case_globals)]
fn styles<'a>(variant: &Variant) -> stylist::Result<Styles> {
    let colors = get_colors(variant);

    style!(
            button {
                color: ${colors.text};
                background-color: ${colors.background};
                border: 1px solid ${colors.border};
                outline: none;
                height: 48px;
                min-width: 154px;
                font-size: 14px;
                font-weight: 700;
                text-align: center;
                box-shadow: rgba(0, 0, 0, 0.05) 0px 1px 2px 0px;
                position: relative;
                box-sizing: border-box;
                vertical-align: middle;
                text-align: center;
                text-overflow: ellipsis;
                text-transform: uppercase;
                overflow: hidden;
                cursor: pointer;
                transition: box-shadow 0.2s;
                margin: 10px;
            }

            & button:active {
                transform: scale(0.99);
            }


            & button::-moz-focus-inner {
                border: none;
            }

            & button::before {
                content: "";
                position: absolute;
                top: 0;
                bottom: 0;
                left: 0;
                right: 0;
                background-color: rgb(255, 255, 255);
                opacity: 0;
                transition: opacity 0.2s;
            }

            & button::after {
                content: "";
                position: absolute;
                left: 50%;
                top: 50%;
                border-radius: 50%;
                padding: 50%;
                background-color: ${colors.text};
                opacity: 0;
                transform: translate(-50%, -50%) scale(1);
                transition: opacity 1s, transform 0.5s;
            }

            & button:hover,
            & button:focus {
                box-shadow: 0 2px 4px -1px rgba(0, 0, 0, 0.2), 0 4px 5px 0 rgba(0, 0, 0, 0.14), 0 1px 10px 0 rgba(0, 0, 0, 0.12);
            }

            & button:hover::before {
                opacity: 0.08;
            }

            & button:hover:focus::before {
                opacity: 0.3;
            }

            & button:active {
                box-shadow: 0 5px 5px -3px rgba(0, 0, 0, 0.2), 0 8px 10px 1px rgba(0, 0, 0, 0.14), 0 3px 14px 2px rgba(0, 0, 0, 0.12);
            }

            & button:active::after {
                opacity: 0.32;
                transform: translate(-50%, -50%) scale(0);
                transition: transform 0s;
            }

            & button:disabled {
                color: rgba(0, 0, 0, 0.28);
                background-color: rgba(0, 0, 0, 0.12);
                box-shadow: none;
                cursor: initial;
            }

            & button:disabled::before {
                opacity: 0;
            }

            & button:disabled::after {
                opacity: 0;
            }

    )
}
