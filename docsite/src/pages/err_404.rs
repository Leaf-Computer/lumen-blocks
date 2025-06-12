use dioxus::prelude::*;
use dioxus_router::prelude::*;
use laminar_blocks::components::{
    avatar::{Avatar, AvatarImage, AvatarFallback},
    button::{Button, ButtonVariant, ButtonSize},
    input::{Input, InputSize},
    progress::{Progress, ProgressSize, ProgressVariant},
    switch::Switch,
};
use lucide_dioxus::{Check, Info, X};

use crate::components::Navbar;
use crate::Route;
use crate::LAMINAR_LOGO;
use crate::components::FeatureCard;

#[component]
pub fn Err404(segments: Vec<String>) -> Element {
    rsx! {
        "404 not found"
    }
}
