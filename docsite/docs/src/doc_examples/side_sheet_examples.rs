#![allow(non_snake_case)]

use dioxus::prelude::*;
use laminar_blocks::components::button::{Button, ButtonVariant};
use laminar_blocks::components::side_sheet::{
    SideSheet, SideSheetTrigger, SideSheetContent, SideSheetCloseButton,
    SideSheetHeader, SideSheetTitle, SideSheetDescription,
    SideSheetBody, SideSheetFooter, SideSheetClose, SideSheetSide
};
use lucide_dioxus::{Menu, Settings, Info, X, CircleHelp};

pub use basic::BasicSideSheetExample;
pub use positions::SideSheetPositionsExample;
pub use settings::SettingsSideSheetExample;
pub use help::HelpSideSheetExample;

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
                            variant: use_signal(|| ButtonVariant::Primary),
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
                                    variant: use_signal(|| ButtonVariant::Outline),
                                    "Cancel"
                                }
                            }
                            Button {
                                variant: use_signal(|| ButtonVariant::Primary),
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
                                variant: use_signal(|| ButtonVariant::Primary),
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
                                variant: use_signal(|| ButtonVariant::Secondary),
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

pub mod settings {
    // ANCHOR: settings
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant};
    use laminar_blocks::components::side_sheet::{
        SideSheet, SideSheetTrigger, SideSheetContent, SideSheetCloseButton,
        SideSheetHeader, SideSheetTitle, SideSheetDescription,
        SideSheetBody, SideSheetFooter, SideSheetClose
    };
    use lucide_dioxus::Settings;
    
    #[component]
    pub fn SettingsSideSheetExample() -> Element {
        rsx! {
            div {
                SideSheet {
                    SideSheetTrigger {
                        Button {
                            variant: use_signal(|| ButtonVariant::Outline),
                            Settings { class: "mr-2 h-4 w-4" }
                            "Settings"
                        }
                    }
                    
                    SideSheetContent {
                        class: "p-6 flex flex-col h-full",
                        
                        SideSheetCloseButton {}
                        
                        SideSheetHeader {
                            SideSheetTitle {
                                "Application Settings"
                            }
                            SideSheetDescription {
                                "Manage your preferences and application settings."
                            }
                        }
                        
                        SideSheetBody {
                            class: "py-6 space-y-6",
                            
                            div {
                                h3 { class: "text-md font-medium mb-3", "User Preferences" }
                                div {
                                    class: "space-y-2",
                                    div {
                                        class: "flex items-center justify-between",
                                        span { "Dark Mode" }
                                        // Placeholder for a toggle switch
                                        div { class: "w-10 h-5 bg-muted rounded-full" }
                                    }
                                    div {
                                        class: "flex items-center justify-between",
                                        span { "Notifications" }
                                        // Placeholder for a toggle switch
                                        div { class: "w-10 h-5 bg-primary rounded-full" }
                                    }
                                }
                            }
                        }
                        
                        SideSheetFooter {
                            SideSheetClose {
                                Button {
                                    variant: use_signal(|| ButtonVariant::Outline),
                                    "Cancel"
                                }
                            }
                            Button {
                                variant: use_signal(|| ButtonVariant::Primary),
                                "Save Settings"
                            }
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: settings
}

pub mod help {
    // ANCHOR: help
    use dioxus::prelude::*;
    use laminar_blocks::components::button::{Button, ButtonVariant};
    use laminar_blocks::components::side_sheet::{
        SideSheet, SideSheetTrigger, SideSheetContent, SideSheetCloseButton,
        SideSheetHeader, SideSheetTitle, SideSheetDescription,
        SideSheetBody, SideSheetFooter, SideSheetClose
    };
    use lucide_dioxus::{CircleHelp, Info};
    
    #[component]
    pub fn HelpSideSheetExample() -> Element {
        rsx! {
            div {
                SideSheet {
                    SideSheetTrigger {
                        Button {
                            variant: use_signal(|| ButtonVariant::Ghost),
                            CircleHelp { class: "mr-2 h-4 w-4" }
                            "Help"
                        }
                    }
                    
                    SideSheetContent {
                        class: "p-6 flex flex-col h-full",
                        
                        SideSheetCloseButton {}
                        
                        SideSheetHeader {
                            SideSheetTitle {
                                "Need Help?"
                            }
                            SideSheetDescription {
                                "Find answers to frequently asked questions."
                            }
                        }
                        
                        SideSheetBody {
                            class: "py-6",
                            
                            div {
                                class: "space-y-4",
                                div {
                                    h3 { class: "text-md font-medium", "Frequently Asked Questions" }
                                    div {
                                        class: "mt-2 space-y-3",
                                        details {
                                            class: "group",
                                            summary { class: "cursor-pointer font-medium", "How do I use this component?" }
                                            p { class: "mt-2 text-sm", "The Side Sheet component provides a sliding panel for showing additional information or controls without navigating away from the current page." }
                                        }
                                        details {
                                            class: "group",
                                            summary { class: "cursor-pointer font-medium", "Can I customize the width?" }
                                            p { class: "mt-2 text-sm", "Yes, you can apply custom classes to the SideSheetContent component to control width, background color, and other styling properties." }
                                        }
                                    }
                                }
                                
                                div {
                                    class: "mt-4",
                                    Info { class: "inline-block mb-1 h-4 w-4 text-primary" }
                                    span { class: "ml-1 text-sm", "For more detailed help, please refer to the documentation." }
                                }
                            }
                        }
                        
                        SideSheetFooter {
                            SideSheetClose {
                                Button {
                                    variant: use_signal(|| ButtonVariant::Primary),
                                    "Close Help"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // ANCHOR_END: help
}
