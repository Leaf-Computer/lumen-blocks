use ::docs::docs::{
    dropdown_examples::ComplexDropdownExample, menubar_examples::MenubarWithIconsExample,
    progress_examples::InteractiveProgressExample,
};
use dioxus::prelude::*;
use docs::accordion_examples::BasicAccordionExample;
use docs::avatar_examples::AvatarGroupExample;
use docs::button_examples::ButtonVariantsExample;
use docs::docs;
use docs::form_examples::CompleteFormExample;
use docs::hover_card_examples::HoverCardProfileExample;
use docs::side_sheet_examples::BasicSideSheetExample;
use docs::switch_examples::SwitchWithTextExample;
use docs::toast_examples::ToastWithDescriptionsExample;
use lucide_dioxus::{Check, PersonStanding, Wind};
use lumen_blocks::components::{
    button::{Button, ButtonSize, ButtonVariant},
    toast::ToastProvider,
};

use crate::components::{ComponentCard, FeatureCard};
use crate::Route;
use crate::LUMEN_LOGO;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "min-h-screen bg-background relative",

            div { class: "max-w-6xl mx-auto px-6 py-12",
                div { class: "text-center mb-12",
                    img { class: "w-48 h-48 mx-auto mb-4 dark:invert", src: LUMEN_LOGO, alt: "Lumen Logo" }
                    h1 { class: "text-4xl font-bold text-foreground mb-4", "Lumen Blocks" }
                    p { class: "text-xl text-muted-foreground mb-8",
                        "Styled, opinionated UI components for building Dioxus applications"
                    }

                    div { class: "flex justify-center gap-4",
                        Link { to: Route::Docs01 { child: docs::router_01::BookRoute::Index { section: Default::default() } },
                            Button {
                                variant: ButtonVariant::Primary,
                                size: ButtonSize::Large,
                                "View Docs"
                            }
                        }
                    }
                }

                // Feature Cards
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 mb-24",
                    FeatureCard {
                        title: "Rich Components".to_string(),
                        description: "Comprehensive set of UI components built for modern web applications".to_string(),
                        icon: rsx! { Check { class: "w-8 h-8 text-primary" } }
                    }
                    FeatureCard {
                        title: "Tailwind Styled".to_string(),
                        description: "Beautifully designed with Tailwind CSS and dark mode support".to_string(),
                        icon: rsx! { Wind { class: "w-8 h-8 text-primary" } }
                    }
                    FeatureCard {
                        title: "Accessible".to_string(),
                        description: "Built with accessibility in mind, following good ARIA practices".to_string(),
                        icon: rsx! { PersonStanding { class: "w-8 h-8 text-primary" } }
                    }
                }

                // Interactive Component Showcase - Bento Grid
                ToastProvider {
                    div { class: "rounded-lg",
                        h2 { class: "text-3xl font-semibold text-foreground mb-6", "Component Showcase" }

                        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                            // Buttons
                            ComponentCard {
                                title: "Button".to_string(),
                                doc_route: Route::Docs01 { child: docs::router_01::BookRoute::ButtonIndex { section: Default::default() } },
                                col_span: Some("md:col-span-2".to_string()),
                                ButtonVariantsExample {  }
                            }

                            // Hover Card
                            ComponentCard {
                                title: "Hover Card".to_string(),
                                doc_route: Route::Docs01 { child: docs::router_01::BookRoute::HoverCardIndex { section: Default::default() } },
                                content_class: Some("px-4".to_string()),
                                HoverCardProfileExample {}
                            }

                            // Complete form example
                            ComponentCard {
                                title: "Form components".to_string(),
                                doc_route: Route::Docs01 { child: docs::router_01::BookRoute::FormIndex { section: Default::default() } },
                                col_span: Some("md:col-span-1".to_string()),
                                row_span: Some("md:row-span-2".to_string()),
                                CompleteFormExample {}
                            }

                            // Switch (small screen)
                            ComponentCard {
                                title: "Switch".to_string(),
                                doc_route: Route::Docs01 { child: docs::router_01::BookRoute::SwitchIndex { section: Default::default() } },
                                col_span: Some("col-span-1 block lg:hidden".to_string()),
                                SwitchWithTextExample {}
                            }

                            // Accordion
                            ComponentCard {
                                title: "Accordion".to_string(),
                                doc_route: Route::Docs01 { child: docs::router_01::BookRoute::AccordionIndex { section: Default::default() } },
                                col_span: Some("md:col-span-2".to_string()),
                                BasicAccordionExample {}
                            }

                            // Switch (large screen)
                            ComponentCard {
                                title: "Switch".to_string(),
                                doc_route: Route::Docs01 { child: docs::router_01::BookRoute::SwitchIndex { section: Default::default() } },
                                col_span: Some("col-span-1 hidden lg:block".to_string()),
                                SwitchWithTextExample {}
                            }

                            // Side Sheet
                            ComponentCard {
                                title: "Side Sheet".to_string(),
                                doc_route: Route::Docs01 { child: docs::router_01::BookRoute::SideSheetIndex { section: Default::default() } },
                                col_span: Some("md:col-span-1".to_string()),
                                BasicSideSheetExample {}
                            }

                            // Toast
                            ComponentCard {
                                title: "Toast".to_string(),
                                doc_route: Route::Docs01 { child: docs::router_01::BookRoute::ToastIndex { section: Default::default() } },
                                ToastWithDescriptionsExample {}
                            }

                            // Menu bar
                            ComponentCard {
                                title: "Menubar".to_string(),
                                doc_route: Route::Docs01 { child: docs::router_01::BookRoute::ProgressIndex { section: Default::default() } },
                                col_span: Some("md:col-span-2".to_string()),
                                MenubarWithIconsExample {  }
                            }

                            // Dropdown
                            ComponentCard {
                                title: "Dropdown".to_string(),
                                doc_route: Route::Docs01 { child: docs::router_01::BookRoute::DropdownIndex { section: Default::default() } },
                                ComplexDropdownExample {  }
                            }

                            // Progress
                            ComponentCard {
                                title: "Progress".to_string(),
                                doc_route: Route::Docs01 { child: docs::router_01::BookRoute::ProgressIndex { section: Default::default() } },
                                col_span: Some("md:col-span-1".to_string()),
                                InteractiveProgressExample {  }
                            }

                            // Avatar
                            ComponentCard {
                                title: "Avatar".to_string(),
                                doc_route: Route::Docs01 { child: docs::router_01::BookRoute::AvatarIndex { section: Default::default() } },
                                col_span: Some("col-span-1 md:col-span-2 lg:col-span-1".to_string()),
                                AvatarGroupExample {  }
                            }
                        }
                    }
                }
            }
        }
    }
}
