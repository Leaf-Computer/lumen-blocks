pub mod layout;
pub mod cards;
pub mod showcase;
pub mod toast_demo;
pub mod progress_demo;
pub mod form_demo;
pub mod project_detail_card;
pub mod team_member_card;
pub mod main_layout;
pub mod navbar;
pub mod feature_card;

// Layouts and core components
pub use layout::*;
pub use showcase::*;
pub use toast_demo::*;
pub use progress_demo::*;
pub use form_demo::*;
pub use main_layout::*;
pub use navbar::Navbar;

// Cards - avoid glob imports for modules with conflicting names
pub use cards::{StatsCard, ProjectCard}; // Import specific items from cards

// Detail components
pub use project_detail_card::ProjectDetailCard;
pub use team_member_card::TeamMemberCard;
pub use feature_card::FeatureCard;