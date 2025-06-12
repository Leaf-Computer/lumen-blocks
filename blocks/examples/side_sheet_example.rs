use dioxus::prelude::*;
use laminar_blocks::components::button::{Button, ButtonVariant};
use laminar_blocks::components::side_sheet::*;
use lucide_dioxus::{Menu, Info, Settings, X};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-background p-8",
            
            div {
                class: "max-w-4xl mx-auto space-y-8",
                
                h1 {
                    class: "text-3xl font-bold text-foreground mb-8",
                    "Side Sheet Examples"
                }
                
                SideSheetExamples {}
            }
        }
    }
}

#[component]
pub fn SideSheetExamples() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 gap-6",
            
            // Basic Side Sheet - Right Side
            div {
                class: "space-y-4 p-6 border rounded-md",
                
                h2 {
                    class: "text-xl font-semibold text-foreground mb-4",
                    "Basic Side Sheet (Right)"
                }
                
                SideSheet {
                    // Right side is the default
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
                            
                            div {
                                class: "mt-4",
                                ul {
                                    class: "space-y-2",
                                    li {
                                        class: "flex items-center",
                                        span { class: "mr-2 text-primary", "•" }
                                        "Item 1"
                                    }
                                    li {
                                        class: "flex items-center",
                                        span { class: "mr-2 text-primary", "•" }
                                        "Item 2"
                                    }
                                    li {
                                        class: "flex items-center",
                                        span { class: "mr-2 text-primary", "•" }
                                        "Item 3"
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
                                "Save Changes"
                            }
                        }
                    }
                }
                
                p {
                    class: "text-sm text-muted-foreground mt-4",
                    "Click the button to open a side sheet from the right side."
                }
            }
            
            // Left Side Sheet
            div {
                class: "space-y-4 p-6 border rounded-md",
                
                h2 {
                    class: "text-xl font-semibold text-foreground mb-4",
                    "Left Side Sheet"
                }
                
                SideSheet {
                    side: SideSheetSide::Left,
                    
                    SideSheetTrigger {
                        Button {
                            variant: use_signal(|| ButtonVariant::Secondary),
                            Menu { class: "mr-2 h-4 w-4" }
                            "Open Left Menu"
                        }
                    }
                    
                    SideSheetContent {
                        class: "p-6 flex flex-col h-full",
                        
                        SideSheetCloseButton {}
                        
                        SideSheetHeader {
                            SideSheetTitle {
                                "Navigation"
                            }
                            SideSheetDescription {
                                "Browse through available sections."
                            }
                        }
                        
                        SideSheetBody {
                            class: "py-6",
                            nav {
                                class: "space-y-2",
                                a {
                                    class: "block p-2 hover:bg-muted rounded-md",
                                    href: "#",
                                    "Home"
                                }
                                a {
                                    class: "block p-2 hover:bg-muted rounded-md",
                                    href: "#",
                                    "About"
                                }
                                a {
                                    class: "block p-2 hover:bg-muted rounded-md",
                                    href: "#",
                                    "Services"
                                }
                                a {
                                    class: "block p-2 hover:bg-muted rounded-md",
                                    href: "#",
                                    "Contact"
                                }
                            }
                        }
                    }
                }
                
                p {
                    class: "text-sm text-muted-foreground mt-4",
                    "Click the button to open a side sheet from the left side."
                }
            }
            
            // Settings Side Sheet
            div {
                class: "space-y-4 p-6 border rounded-md",
                
                h2 {
                    class: "text-xl font-semibold text-foreground mb-4",
                    "Settings Panel"
                }
                
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
                            
                            div {
                                h3 { class: "text-md font-medium mb-3", "Account Settings" }
                                div {
                                    class: "space-y-2",
                                    div {
                                        label { class: "block text-sm mb-1", "Display Name" }
                                        input {
                                            class: "w-full p-2 border rounded-md",
                                            r#type: "text",
                                            value: "User Name"
                                        }
                                    }
                                    div {
                                        label { class: "block text-sm mb-1", "Email" }
                                        input {
                                            class: "w-full p-2 border rounded-md",
                                            r#type: "email",
                                            value: "user@example.com"
                                        }
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
                
                p {
                    class: "text-sm text-muted-foreground mt-4",
                    "Click the button to open a settings panel."
                }
            }
            
            // Help Side Sheet
            div {
                class: "space-y-4 p-6 border rounded-md",
                
                h2 {
                    class: "text-xl font-semibold text-foreground mb-4",
                    "Help Panel"
                }
                
                SideSheet {
                    SideSheetTrigger {
                        Button {
                            variant: use_signal(|| ButtonVariant::Ghost),
                            Info { class: "mr-2 h-4 w-4" }
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
                                "Find answers to frequently asked questions or contact support."
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
                                            summary { class: "cursor-pointer font-medium", "How do I create an account?" }
                                            p { class: "mt-2 text-sm", "To create an account, click on the 'Sign Up' button in the top right corner of the page." }
                                        }
                                        details {
                                            class: "group",
                                            summary { class: "cursor-pointer font-medium", "How do I reset my password?" }
                                            p { class: "mt-2 text-sm", "You can reset your password by clicking on the 'Forgot Password' link on the login page." }
                                        }
                                        details {
                                            class: "group",
                                            summary { class: "cursor-pointer font-medium", "Is there a mobile app available?" }
                                            p { class: "mt-2 text-sm", "Yes, our mobile app is available on both iOS and Android app stores." }
                                        }
                                    }
                                }
                                
                                div {
                                    h3 { class: "text-md font-medium", "Contact Support" }
                                    p { class: "mt-1 text-sm", "If you can't find the answer you're looking for, please contact our support team." }
                                    
                                    div {
                                        class: "mt-3",
                                        Button {
                                            variant: use_signal(|| ButtonVariant::Secondary),
                                            full_width: use_signal(|| true),
                                            "Contact Support"
                                        }
                                    }
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
                
                p {
                    class: "text-sm text-muted-foreground mt-4",
                    "Click the button to open the help panel."
                }
            }
        }
        
        div {
            class: "mt-8 p-4 bg-muted rounded-lg",
            h3 {
                class: "text-lg font-semibold mb-2",
                "Implementation Notes"
            }
            ul {
                class: "space-y-1 text-sm text-muted-foreground",
                li { "• Side sheets can slide in from either left or right" }
                li { "• They are implemented with proper WAI-ARIA attributes for accessibility" }
                li { "• Clicking the overlay or pressing Escape will close the side sheet" }
                li { "• Use them for secondary navigation, settings panels, or contextual information" }
                li { "• The content can be structured using Header, Body, and Footer components" }
            }
        }
    }
}