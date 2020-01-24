use ::std::collections::HashMap;
use ::strum::IntoEnumIterator;
use ::strum_macros::EnumIter;

use crate::style::presets::ASCII_FULL;

pub enum ContentArrangement {
    /// Don't do any automatic width calculation.
    /// Table with this mode might overflow and look ugly, if content gets too long.
    /// Constraints on columns are still respected.
    Disabled,
    /// Automatically determine the width of columns in regard to terminal width and content length.
    /// With this mode, the content in cells will wrap automatically and comfy-table tries to determine
    /// the best column layout for the given content.
    /// Constraints on columns are still respected.
    Automatic,
    // /// Same as Automatic, but the full width of the terminal will always be used.
    // /// Use this, if you want tables to use as much space as possible.
    // /// Constraints on columns are still respected.
    // Full,
}

/// All configurable table components.
/// A character can be assigned to each component in the [TableStyle] struct.
/// This is then used to draw character of the respective component to the commandline.
/// Most components should be self-explanatory.
///
/// BorderIntersections are Intersections, where rows/columns lines meet outer borders.
/// E.g.:
/// ```text
///        --------
///        v      |
/// +--+---+---+  |
/// |  |   |   |  |
/// +----------+ <- These "+" chars are border intersection
/// |  |   |   |
/// +--+---+---+
/// ```
#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Component {
    LeftBorder,
    RightBorder,
    TopBorder,
    BottomBorder,
    LeftHeaderIntersection,
    HeaderLines,
    MiddleHeaderIntersections,
    RightHeaderIntersection,
    VerticalLines,
    HorizontalLines,
    MiddleIntersections,
    LeftBorderIntersections,
    RightBorderIntersections,
    TopBorderIntersections,
    BottomBorderIntersections,
    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,
}

/// This struct wraps the various styling options for a table
/// The default style preset when using `::new` is the [ASCII_FULL]
pub struct TableStyle {
    pub(crate) has_header: bool,
    style: HashMap<Component, char>,
}

impl TableStyle {
    /// Create a new TableStyle. The default style is [ASCII_FULL],
    pub fn new() -> Self {
        let mut table_style = TableStyle {
            has_header: false,
            style: HashMap::new(),
        };
        table_style.load_preset(ASCII_FULL);

        table_style
    }

    /// This function creates a TableStyle from a given preset string.
    /// Preset strings can be found in styling::presets::*
    ///
    /// Anyway, you can write your own preset strings and use them with this function.
    /// The function expects a characters for components to be in the same order as in the [Component] enum.
    ///
    /// If the string isn't long enough, the default [ASCII_FULL] style will be used for all remaining components.
    ///
    /// If the string is too long, remaining charaacters will be simply ignored.
    pub fn load_preset(&mut self, preset: &str) {
        let mut components = Component::iter();

        for character in preset.chars() {
            if let Some(component) = components.next() {
                // White spaces mean "don't draw this" in presets
                // If we want to override the default preset, we need to remove
                // this component from the HashMap in case we find a whitespace.
                if character == ' ' {
                    self.style.remove(&component);
                    continue;
                }

                self.style.insert(component, character);
            } else {
                break;
            }
        }
    }

    /// Modify a preset with a modifier string from [modifiers](crate::styling::modifiers).
    /// For instance, the [UTF8_ROUND_CORNERS](crate::styling::modifiers::UTF8_ROUND_CORNERS) modifies all corners to be round UTF8 box corners.
    pub fn apply_modifier(&mut self, modifier: &str) -> &mut Self {
        let mut components = Component::iter();

        for character in modifier.chars() {
            // Skip spaces while applying modifiers.
            if character == ' ' {
                continue;
            }
            if let Some(component) = components.next() {
                self.style.insert(component, character);
            } else {
                break;
            }
        }

        self
    }

    /// Define the char that will be used to draw a specific component
    /// Look at [Component] to see all stylable Components
    ///
    /// If `None` is supplied, the element won't be displayed.
    /// In case of a e.g. *BorderIntersection a whitespace will be used as placeholder,
    /// unless related borders and and corners are set to `None` as well.
    ///
    /// For example, if `TopBorderIntersections` is `None` the first row would look like this:
    /// ```text
    /// +------ ------+
    /// | asdf | ghij |
    /// ```
    ///
    /// If in addition `TopLeftCorner`,`TopBorder` and `TopRightCorner` would be `None` as well,
    /// the first line wouldn't be displayed at all.
    pub fn set_style(&mut self, component: Component, character: Option<char>) -> &mut Self {
        match character {
            Some(character) => {
                self.style.insert(component, character);
            }
            None => (),
        };

        self
    }

    /// Get a copy of the char currently used for drawing a specific component
    pub fn get_style(&mut self, component: Component) -> Option<char> {
        match self.style.get(&component) {
            None => None,
            Some(character) => Some(*character),
        }
    }

    pub fn style_or_default(&self, component: Component) -> String {
        match self.style.get(&component) {
            None => " ".to_string(),
            Some(character) => character.to_string(),
        }
    }

    pub fn style_exists(&self, component: Component) -> bool {
        self.style.get(&component).is_some()
    }
}