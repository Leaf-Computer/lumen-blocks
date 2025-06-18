# Laminar Blocks Sample App

A comprehensive demonstration application showcasing the capabilities of the `laminar-blocks` component library for Dioxus. This sample app demonstrates real-world usage patterns and provides a complete example of building modern web applications with the blocks library.

## 🏗️ Project Structure

The sample app is organized into a clean, modular structure:

```
src/
├── main.rs              # App entry point and routing
├── components/          # Reusable UI components
│   ├── mod.rs          # Component exports
│   ├── layout.rs       # Layout components (Sidebar, Header, AppLayout)
│   ├── cards.rs        # Card components (StatsCard, ProjectCard, etc.)
│   └── showcase.rs     # Component demonstration showcase
└── pages/              # Application pages
    ├── mod.rs          # Page exports
    ├── dashboard.rs    # Main dashboard page
    ├── projects.rs     # Projects management page
    ├── team.rs         # Team members page
    └── settings.rs     # User settings page
```

## 🎯 What This Demo Showcases

### Core Components Demonstrated

- **Buttons**: All variants (Primary, Secondary, Outline, Ghost, Link, Destructive), sizes, states, and icon combinations
- **Inputs**: Different sizes, variants, with icons, form validation states
- **Progress Bars**: Multiple variants (Default, Success, Warning, Destructive) and sizes
- **Switches**: Interactive toggles with different sizes and real functionality
- **Avatars**: Image avatars with fallbacks, different sizes, and group layouts
- **Cards**: Complex layouts with multiple components working together

### Real-World Application Patterns

1. **Dashboard Layout**: Complete dashboard with sidebar navigation, header, and content areas
2. **Data Visualization**: Stats cards, progress indicators, and interactive elements
3. **Form Handling**: Settings page with complex forms, validation, and state management
4. **List Management**: Team and project management interfaces
5. **Component Library Showcase**: Interactive examples of all available components

## ✨ Key Features

### 🎨 Modern UI Design
- Clean, professional design using Tailwind CSS
- Dark mode support (toggleable in settings)
- Responsive layout that works on all screen sizes
- Consistent design system throughout

### 🔧 Interactive Elements
- **Live Progress Demo**: Adjustable slider that updates progress bars in real-time
- **Working Switches**: Toggle settings that actually change the UI state
- **Navigation**: Multi-page routing with active state management
- **Form Controls**: Fully functional form inputs with state management

### 📱 Application Sections

#### Dashboard (`/`)
- Overview statistics with trend indicators
- Recent projects list with progress tracking
- Interactive progress demonstration
- Component showcase with live examples

#### Projects (`/projects`)
- Project management interface
- Detailed project cards with team info, progress, and actions
- Statistics overview
- Action buttons (Filter, Export, New Project)

#### Team (`/team`)
- Team member management
- Status indicators (Online, Away, Offline)
- Member statistics and activity tracking
- Profile cards with contact actions

#### Settings (`/settings`)
- Complete user profile management
- Notification preferences
- Security settings with 2FA toggle
- Password change functionality
- Account deletion (danger zone)

## 🚀 Getting Started

### Prerequisites
- Rust (latest stable)
- Dioxus CLI: `cargo install dioxus-cli`

### Running the App

```bash
# Navigate to the sample app directory
cd dioxus-blocks/sample-app

# Install dependencies (if needed)
cargo check

# Run in development mode with hot reload
dx serve --hot-reload

# Or run with cargo
cargo run --features web
```

### Building for Production

```bash
# Build optimized release
dx build --release

# The built files will be in the dist/ directory
```

## 🎓 Learning from This Sample

### Component Usage Patterns

This sample demonstrates proper usage of the blocks library:

```rust
// Button with variant and size
Button {
    variant: Signal::new(ButtonVariant::Primary).into(),
    size: Signal::new(ButtonSize::Small).into(),
    icon_left: rsx! { Check { class: "w-4 h-4" } },
    "Save Changes"
}

// Input with validation state
Input {
    placeholder: Signal::new("Enter email".to_string()).into(),
    variant: Signal::new(InputVariant::Error).into(),
    icon_left: rsx! { Mail { class: "w-4 h-4" } },
}

// Progress with dynamic value
Progress {
    value: Some(Signal::new(progress_value).into()),
    variant: ProgressVariant::Success,
    show_percentage: true,
}
```

### State Management

The app shows how to manage component state effectively:

```rust
// Local state for form inputs
let mut email = use_signal(|| "user@example.com".to_string());

// Shared state across components
let mut theme_mode = use_signal(|| false);

// Reactive updates
Switch {
    checked: theme_mode,
    on_checked_change: Some(EventHandler::new(move |checked| {
        theme_mode.set(checked);
    })),
}
```

### Layout Composition

Learn how to build complex layouts by composing simpler components:

```rust
AppLayout {
    current_section,
    // Page content goes here
    OverviewSection { project_progress }
}
```

## 🎨 Styling and Theming

The app uses Tailwind CSS for styling with a consistent design system:

- **Colors**: Semantic color tokens (primary, secondary, muted, etc.)
- **Spacing**: Consistent spacing scale
- **Typography**: Readable font hierarchy
- **Components**: Styled with the blocks library's design tokens

## 🔧 Customization

This sample can serve as a starting point for your own applications:

1. **Modify the color scheme** by updating Tailwind configuration
2. **Add new pages** by following the existing page structure
3. **Customize components** by extending the blocks library
4. **Add real functionality** by connecting to APIs or databases

## 📚 Next Steps

- Explore the [laminar-blocks documentation](../blocks/) for detailed component APIs
- Check out individual component examples in the showcase section
- Use this sample as a template for your own Dioxus applications
- Contribute improvements or additional examples

## 🤝 Contributing

This sample app is part of the laminar-blocks project. Contributions are welcome:

- Report bugs or issues
- Suggest new component demonstrations
- Improve documentation
- Add new example patterns

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
