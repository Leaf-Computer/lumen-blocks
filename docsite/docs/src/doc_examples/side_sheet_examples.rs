#![allow(non_snake_case)]
pub use basic::BasicSideSheetExample;
pub use positions::SideSheetPositionsExample;

pub mod basic {
    // ANCHOR: basic
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant};
    use laminar_blocks::components::side_sheet::{
        SideSheet, SideSheetTrigger, SideSheetContent, SideSheetCloseButton,
        SideSheetHeader, SideSheetTitle, SideSheetDescription,
        SideSheetBody, SideSheetFooter, SideSheetClose
    };
    
    #[component]
    pub fn BasicSideSheetExample() -> Element {
        rsx! {
            div {
                SideSheet {
                    SideSheetTrigger {
                        Button {
                            variant: ButtonVariant::Primary,
                            "Open Side Sheet"
                        }
                    }
                    
                    SideSheetContent {
                        class: "p-6 flex flex-col h-full",
                        
                        SideSheetCloseButton {}
                        
                        SideSheetHeader {
                            SideSheetTitle {
                                "Side Sheet Title"
                            }
                            SideSheetDescription {
                                "This is a basic side sheet that slides in from the right."
                            }
                        }
                        
                        SideSheetBody {
                            class: "py-6",
                            p {
                                "This is the main content area of the side sheet. You can put any content here, including forms, lists, or other UI elements."
                            }
                        }
                        
                        SideSheetFooter {
                            SideSheetClose {
                                Button {
                                    variant: ButtonVariant::Outline,
                                    "Cancel"
                                }
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                "Save Changes"
                            }
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: basic
}

pub mod positions {
    // ANCHOR: positions
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant};
    use laminar_blocks::components::side_sheet::{
        SideSheet, SideSheetTrigger, SideSheetContent, SideSheetCloseButton,
        SideSheetHeader, SideSheetTitle, SideSheetDescription,
        SideSheetBody, SideSheetSide
    };
    use lucide_dioxus::Menu;
    
    #[component]
    pub fn SideSheetPositionsExample() -> Element {
        rsx! {
            div { class: "flex flex-wrap gap-4",
                // Right side sheet (default)
                div {
                    SideSheet {
                        SideSheetTrigger {
                            Button {
                                variant: ButtonVariant::Primary,
                                "Right Side Sheet"
                            }
                        }
                        
                        SideSheetContent {
                            class: "p-6 flex flex-col h-full",
                            
                            SideSheetCloseButton {}
                            
                            SideSheetHeader {
                                SideSheetTitle {
                                    "Right Side Sheet"
                                }
                                SideSheetDescription {
                                    "This side sheet slides in from the right (default)."
                                }
                            }
                            
                            SideSheetBody {
                                class: "py-6",
                                p {
                                    "Right side sheets are commonly used for detail panels and forms."
                                }
                            }
                        }
                    }
                }
                
                // Left side sheet
                div {
                    SideSheet {
                        side: SideSheetSide::Left,
                        
                        SideSheetTrigger {
                            Button {
                                variant: ButtonVariant::Secondary,
                                Menu { class: "mr-2 h-4 w-4" }
                                "Left Side Sheet"
                            }
                        }
                        
                        SideSheetContent {
                            class: "p-6 flex flex-col h-full",
                            
                            SideSheetCloseButton {}
                            
                            SideSheetHeader {
                                SideSheetTitle {
                                    "Left Side Sheet"
                                }
                                SideSheetDescription {
                                    "This side sheet slides in from the left."
                                }
                            }
                            
                            SideSheetBody {
                                class: "py-6",
                                p {
                                    "Left side sheets are commonly used for navigation menus and filters."
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: positions
}
