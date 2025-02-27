use std::collections::HashMap;

use thaw_macro::WriteCSSVars;

#[derive(Clone, WriteCSSVars)]
pub struct ColorTheme {
    pub color_scheme: String,

    pub color_neutral_background_static: String,
    pub color_neutral_background_inverted: String,
    pub color_neutral_background_disabled: String,
    pub color_neutral_background_1: String,
    pub color_neutral_background_1_hover: String,
    pub color_neutral_background_1_pressed: String,
    pub color_neutral_background_3: String,
    pub color_neutral_background_3_hover: String,
    pub color_neutral_background_3_pressed: String,
    pub color_neutral_background_4: String,
    pub color_neutral_background_4_hover: String,
    pub color_neutral_background_4_pressed: String,
    pub color_neutral_background_5: String,
    pub color_neutral_background_6: String,

    pub color_neutral_foreground_static_inverted: String,
    pub color_neutral_foreground_disabled: String,
    pub color_neutral_foreground_1: String,
    pub color_neutral_foreground_1_hover: String,
    pub color_neutral_foreground_1_pressed: String,
    pub color_neutral_foreground_2: String,
    pub color_neutral_foreground_2_hover: String,
    pub color_neutral_foreground_2_pressed: String,
    pub color_neutral_foreground_2_brand_hover: String,
    pub color_neutral_foreground_2_brand_pressed: String,
    pub color_neutral_foreground_2_brand_selected: String,
    pub color_neutral_foreground_3: String,
    pub color_neutral_foreground_4: String,
    pub color_neutral_foreground_on_brand: String,
    pub color_neutral_foreground_inverted: String,

    pub color_neutral_stroke_disabled: String,
    pub color_neutral_stroke_1: String,
    pub color_neutral_stroke_1_hover: String,
    pub color_neutral_stroke_1_pressed: String,
    pub color_neutral_stroke_2: String,
    pub color_neutral_stroke_accessible: String,
    pub color_neutral_stroke_accessible_hover: String,
    pub color_neutral_stroke_accessible_pressed: String,

    pub color_neutral_shadow_ambient: String,
    pub color_neutral_shadow_key: String,

    pub color_neutral_stencil_1: String,
    pub color_neutral_stencil_2: String,

    pub color_compound_brand_foreground_1: String,
    pub color_compound_brand_foreground_1_hover: String,
    pub color_compound_brand_foreground_1_pressed: String,
    pub color_compound_brand_background: String,
    pub color_compound_brand_background_hover: String,
    pub color_compound_brand_background_pressed: String,
    pub color_compound_brand_stroke: String,
    pub color_compound_brand_stroke_pressed: String,

    pub color_brand_background: String,
    pub color_brand_background_hover: String,
    pub color_brand_background_pressed: String,
    pub color_brand_background_2: String,
    pub color_brand_foreground_1: String,
    pub color_brand_foreground_2: String,
    pub color_brand_stroke_1: String,
    pub color_brand_stroke_2: String,
    pub color_brand_stroke_2_contrast: String,
    pub color_brand_foreground_link: String,
    pub color_brand_foreground_link_hover: String,
    pub color_brand_foreground_link_pressed: String,

    pub color_stroke_focus_2: String,

    pub color_palette_red_background_1: String,
    pub color_palette_red_background_3: String,
    pub color_palette_red_foreground_1: String,
    pub color_palette_red_foreground_3: String,
    pub color_palette_red_border_1: String,
    pub color_palette_red_border_2: String,
    pub color_palette_green_background_1: String,
    pub color_palette_green_background_3: String,
    pub color_palette_green_foreground_1: String,
    pub color_palette_green_foreground_3: String,
    pub color_palette_green_border_1: String,
    pub color_palette_green_border_2: String,
    pub color_palette_yellow_background_1: String,
    pub color_palette_yellow_background_3: String,
    pub color_palette_yellow_foreground_1: String,
    pub color_palette_yellow_foreground_2: String,
    pub color_palette_yellow_border_1: String,

    pub color_palette_dark_orange_background_1: String,
    pub color_palette_dark_orange_background_3: String,
    pub color_palette_dark_orange_foreground_1: String,
    pub color_palette_dark_orange_foreground_3: String,
    pub color_palette_dark_orange_border_1: String,

    pub color_status_success_background_1: String,
    pub color_status_success_foreground_1: String,
    pub color_status_success_border_1: String,
    pub color_status_warning_background_1: String,
    pub color_status_warning_foreground_3: String,
    pub color_status_warning_border_1: String,
    pub color_status_danger_background_1: String,
    pub color_status_danger_foreground_1: String,
    pub color_status_danger_border_1: String,

    pub color_subtle_background: String,
    pub color_subtle_background_hover: String,
    pub color_subtle_background_pressed: String,
    pub color_transparent_background: String,
    pub color_transparent_background_hover: String,
    pub color_transparent_background_pressed: String,
    pub color_transparent_stroke: String,

    pub shadow4: String,
    pub shadow8: String,
    pub shadow16: String,
    pub shadow64: String,
}

impl ColorTheme {

    fn validate_palette(brand_colors: &HashMap<i32, &str>) { 
        for v in 1..=16 {
            let variant = v*10; 
            brand_colors.get(&variant).unwrap_or_else(|| panic!("Missing variant {} in brand color palette", variant));
        }
    }
    pub fn custom_light(brand_colors: &HashMap<i32, &str>) -> Self {
        Self::validate_palette(brand_colors);
        let mut theme = Self::light();
        theme.color_brand_background = brand_colors.get(&80).unwrap().to_string();
        theme.color_brand_background_2 = brand_colors.get(&160).unwrap().to_string();
        theme.color_brand_background_hover = brand_colors.get(&70).unwrap().to_string();
        theme.color_brand_background_pressed = brand_colors.get(&40).unwrap().to_string();
        theme.color_brand_foreground_1 = brand_colors.get(&80).unwrap().to_string();
        theme.color_brand_foreground_2 = brand_colors.get(&70).unwrap().to_string();
        theme.color_brand_foreground_link = brand_colors.get(&70).unwrap().to_string();
        theme.color_brand_foreground_link_hover = brand_colors.get(&60).unwrap().to_string();
        theme.color_brand_foreground_link_pressed = brand_colors.get(&40).unwrap().to_string();
        theme.color_brand_stroke_1 = brand_colors.get(&80).unwrap().to_string();
        theme.color_brand_stroke_2 = brand_colors.get(&140).unwrap().to_string();
        theme.color_brand_stroke_2_contrast = brand_colors.get(&140).unwrap().to_string();
        theme.color_compound_brand_background = brand_colors.get(&80).unwrap().to_string();
        theme.color_compound_brand_background_hover = brand_colors.get(&70).unwrap().to_string();
        theme.color_compound_brand_background_pressed = brand_colors.get(&60).unwrap().to_string();
        theme.color_compound_brand_foreground_1 = brand_colors.get(&80).unwrap().to_string();
        theme.color_compound_brand_foreground_1_hover = brand_colors.get(&70).unwrap().to_string();
        theme.color_compound_brand_foreground_1_pressed = brand_colors.get(&60).unwrap().to_string();
        theme.color_compound_brand_stroke = brand_colors.get(&80).unwrap().to_string();
        theme.color_compound_brand_stroke_pressed = brand_colors.get(&60).unwrap().to_string();
        theme.color_neutral_foreground_2_brand_hover = brand_colors.get(&80).unwrap().to_string();
        theme.color_neutral_foreground_2_brand_pressed = brand_colors.get(&70).unwrap().to_string();
        theme.color_neutral_foreground_2_brand_selected = brand_colors.get(&80).unwrap().to_string();
        theme
    }

    pub fn custom_dark(brand_colors: &HashMap<i32, &str>) -> Self {
        Self::validate_palette(brand_colors);
        let mut theme = Self::dark();
        theme.color_brand_background = brand_colors.get(&70).unwrap().to_string();
        theme.color_brand_background_2 = brand_colors.get(&20).unwrap().to_string();
        theme.color_brand_background_hover = brand_colors.get(&80).unwrap().to_string();
        theme.color_brand_background_pressed = brand_colors.get(&40).unwrap().to_string();
        theme.color_brand_foreground_1 = brand_colors.get(&110).unwrap().to_string();
        theme.color_brand_foreground_2 = brand_colors.get(&120).unwrap().to_string();
        theme.color_brand_foreground_link = brand_colors.get(&100).unwrap().to_string();
        theme.color_brand_foreground_link_hover = brand_colors.get(&110).unwrap().to_string();
        theme.color_brand_foreground_link_pressed = brand_colors.get(&90).unwrap().to_string();
        theme.color_brand_stroke_1 = brand_colors.get(&100).unwrap().to_string();
        theme.color_brand_stroke_2 = brand_colors.get(&50).unwrap().to_string();
        theme.color_brand_stroke_2_contrast = brand_colors.get(&50).unwrap().to_string();
        theme.color_compound_brand_background = brand_colors.get(&100).unwrap().to_string();
        theme.color_compound_brand_background_hover = brand_colors.get(&110).unwrap().to_string();
        theme.color_compound_brand_background_pressed = brand_colors.get(&90).unwrap().to_string();
        theme.color_compound_brand_foreground_1 = brand_colors.get(&100).unwrap().to_string();
        theme.color_compound_brand_foreground_1_hover = brand_colors.get(&110).unwrap().to_string();
        theme.color_compound_brand_foreground_1_pressed = brand_colors.get(&90).unwrap().to_string();
        theme.color_compound_brand_stroke = brand_colors.get(&100).unwrap().to_string();
        theme.color_compound_brand_stroke_pressed = brand_colors.get(&90).unwrap().to_string();
        theme.color_neutral_foreground_2_brand_hover = brand_colors.get(&100).unwrap().to_string();
        theme.color_neutral_foreground_2_brand_pressed = brand_colors.get(&90).unwrap().to_string();
        theme.color_neutral_foreground_2_brand_selected = brand_colors.get(&100).unwrap().to_string();
        theme
    }

    pub fn light() -> Self {
        Self {
            color_scheme: "light".into(),

            color_neutral_background_static: "#333333".into(),
            color_neutral_background_inverted: "#292929".into(),
            color_neutral_background_disabled: "#f0f0f0".into(),
            color_neutral_background_1: "#ffffff".into(),
            color_neutral_background_1_hover: "#f5f5f5".into(),
            color_neutral_background_1_pressed: "#e0e0e0".into(),
            color_neutral_background_3: "#f5f5f5".into(),
            color_neutral_background_3_hover: "#ebebeb".into(),
            color_neutral_background_3_pressed: "#d6d6d6".into(),
            color_neutral_background_4: "#f0f0f0".into(),
            color_neutral_background_4_hover: "#fafafa".into(),
            color_neutral_background_4_pressed: "#f5f5f5".into(),
            color_neutral_background_5: "#ebebeb".into(),
            color_neutral_background_6: "#e6e6e6".into(),

            color_neutral_foreground_static_inverted: "#ffffff".into(),
            color_neutral_foreground_disabled: "#bdbdbd".into(),
            color_neutral_foreground_1: "#242424".into(),
            color_neutral_foreground_1_hover: "#242424".into(),
            color_neutral_foreground_1_pressed: "#242424".into(),
            color_neutral_foreground_2: "#424242".into(),
            color_neutral_foreground_2_hover: "#242424".into(),
            color_neutral_foreground_2_pressed: "#242424".into(),
            color_neutral_foreground_2_brand_hover: "#0f6cbd".into(),
            color_neutral_foreground_2_brand_pressed: "#115ea3".into(),
            color_neutral_foreground_2_brand_selected: "#0f6cbd".into(),
            color_neutral_foreground_3: "#616161".into(),
            color_neutral_foreground_4: "#707070".into(),
            color_neutral_foreground_on_brand: "#fff".into(),
            color_neutral_foreground_inverted: "#fff".into(),

            color_neutral_stroke_disabled: "#e0e0e0".into(),
            color_neutral_stroke_1: "#d1d1d1".into(),
            color_neutral_stroke_1_hover: "#c7c7c7".into(),
            color_neutral_stroke_1_pressed: "#b3b3b3".into(),
            color_neutral_stroke_2: "#e0e0e0".into(),
            color_neutral_stroke_accessible: "#616161".into(),
            color_neutral_stroke_accessible_hover: "#575757".into(),
            color_neutral_stroke_accessible_pressed: "#4d4d4d".into(),

            color_neutral_shadow_ambient: "rgba(0,0,0,0.12)".into(),
            color_neutral_shadow_key: "rgba(0,0,0,0.14)".into(),

            color_neutral_stencil_1: "#e6e6e6".into(),
            color_neutral_stencil_2: "#fafafa".into(),

            color_compound_brand_foreground_1: "#0f6cbd".into(),
            color_compound_brand_foreground_1_hover: "#115ea3".into(),
            color_compound_brand_foreground_1_pressed: "#0f548c".into(),

            color_compound_brand_background: "#0f6cbd".into(),
            color_compound_brand_background_hover: "#115ea3".into(),
            color_compound_brand_background_pressed: "#0f548c".into(),
            color_compound_brand_stroke: "#0f6cbd".into(),
            color_compound_brand_stroke_pressed: "#0f548c".into(),

            color_brand_background: "#0f6cbd".into(),
            color_brand_background_hover: "#115ea3".into(),
            color_brand_background_pressed: "#0c3b5e".into(),
            color_brand_background_2: "#ebf3fc".into(),
            color_brand_foreground_1: "#0f6cbd".into(),
            color_brand_foreground_2: "#115ea3".into(),
            color_brand_stroke_1: "#0f6cbd".into(),
            color_brand_stroke_2: "#b4d6fa".into(),
            color_brand_stroke_2_contrast: "#b4d6fa".into(),
            color_brand_foreground_link: "#115ea3".into(),
            color_brand_foreground_link_hover: "#0f548c".into(),
            color_brand_foreground_link_pressed: "#0c3b5e".into(),

            color_stroke_focus_2: "#000000".into(),

            color_palette_red_background_1: "#fdf6f6".into(),
            color_palette_red_background_3: "#d13438".into(),
            color_palette_red_foreground_1: "#bc2f32".into(),
            color_palette_red_foreground_3: "#d13438".into(),
            color_palette_red_border_1: "#f1bbbc".into(),
            color_palette_red_border_2: "#d13438".into(),
            color_palette_green_background_1: "#f1faf1".into(),
            color_palette_green_background_3: "#107c10".into(),
            color_palette_green_foreground_1: "#0e700e".into(),
            color_palette_green_foreground_3: "#107c10".into(),
            color_palette_green_border_1: "#9fd89f".into(),
            color_palette_green_border_2: "#107c10".into(),
            color_palette_yellow_background_1: "#fffef5".into(),
            color_palette_yellow_background_3: "#fde300".into(),
            color_palette_yellow_foreground_1: "#817400".into(),
            color_palette_yellow_foreground_2: "#817400".into(),
            color_palette_yellow_border_1: "#fef7b2".into(),

            color_palette_dark_orange_background_1: "#fdf6f3".into(),
            color_palette_dark_orange_background_3: "#da3b01".into(),
            color_palette_dark_orange_foreground_1: "#c43501".into(),
            color_palette_dark_orange_foreground_3: "#da3b01".into(),
            color_palette_dark_orange_border_1: "#f4bfab".into(),

            color_status_success_background_1: "#f1faf1".into(),
            color_status_success_foreground_1: "#0e700e".into(),
            color_status_success_border_1: "#9fd89f".into(),
            color_status_warning_background_1: "#fff9f5".into(),
            color_status_warning_foreground_3: "#bc4b09".into(),
            color_status_warning_border_1: "#fdcfb4".into(),
            color_status_danger_background_1: "#fdf3f4".into(),
            color_status_danger_foreground_1: "#b10e1c".into(),
            color_status_danger_border_1: "#eeacb2".into(),

            color_subtle_background: "transparent".into(),
            color_subtle_background_hover: "#f5f5f5".into(),
            color_subtle_background_pressed: "#e0e0e0".into(),
            color_transparent_background: "transparent".into(),
            color_transparent_background_hover: "transparent".into(),
            color_transparent_background_pressed: "transparent".into(),
            color_transparent_stroke: "transparent".into(),

            shadow4: "0 0 2px rgba(0,0,0,0.12), 0 2px 4px rgba(0,0,0,0.14)".into(),
            shadow8: "0 0 2px rgba(0,0,0,0.12), 0 4px 8px rgba(0,0,0,0.14)".into(),
            shadow16: "0 0 2px rgba(0,0,0,0.12), 0 8px 16px rgba(0,0,0,0.14)".into(),
            shadow64: "0 0 8px rgba(0,0,0,0.12), 0 32px 64px rgba(0,0,0,0.14)".into(),
        }
    }

    pub fn dark() -> Self {
        Self {
            color_scheme: "dark".into(),

            color_neutral_background_static: "#3d3d3d".into(),
            color_neutral_background_inverted: "#ffffff".into(),
            color_neutral_background_disabled: "#141414".into(),
            color_neutral_background_1: "#292929".into(),
            color_neutral_background_1_hover: "#3d3d3d".into(),
            color_neutral_background_1_pressed: "#1f1f1f".into(),
            color_neutral_background_3: "#141414".into(),
            color_neutral_background_3_hover: "#292929".into(),
            color_neutral_background_3_pressed: "#0a0a0a".into(),
            color_neutral_background_4: "#0a0a0a".into(),
            color_neutral_background_4_hover: "#1f1f1f".into(),
            color_neutral_background_4_pressed: "#000000".into(),
            color_neutral_background_5: "#000000".into(),
            color_neutral_background_6: "#333333".into(),

            color_neutral_foreground_static_inverted: "#ffffff".into(),
            color_neutral_foreground_disabled: "#5c5c5c".into(),
            color_neutral_foreground_1: "#fff".into(),
            color_neutral_foreground_1_hover: "#fff".into(),
            color_neutral_foreground_1_pressed: "#fff".into(),
            color_neutral_foreground_2: "#d6d6d6".into(),
            color_neutral_foreground_2_hover: "#fff".into(),
            color_neutral_foreground_2_pressed: "#fff".into(),
            color_neutral_foreground_2_brand_hover: "#479ef5".into(),
            color_neutral_foreground_2_brand_pressed: "#2886de".into(),
            color_neutral_foreground_2_brand_selected: "#479ef5".into(),
            color_neutral_foreground_3: "#adadad".into(),
            color_neutral_foreground_4: "#999999".into(),
            color_neutral_foreground_on_brand: "#fff".into(),
            color_neutral_foreground_inverted: "#242424".into(),

            color_neutral_stroke_disabled: "#424242".into(),
            color_neutral_stroke_1: "#666666".into(),
            color_neutral_stroke_1_hover: "#757575".into(),
            color_neutral_stroke_1_pressed: "#6b6b6b".into(),
            color_neutral_stroke_2: "#525252".into(),
            color_neutral_stroke_accessible: "#adadad".into(),
            color_neutral_stroke_accessible_hover: "#bdbdbd".into(),
            color_neutral_stroke_accessible_pressed: "#b3b3b3".into(),

            color_neutral_shadow_ambient: "rgba(0,0,0,0.24)".into(),
            color_neutral_shadow_key: "rgba(0,0,0,0.28)".into(),

            color_neutral_stencil_1: "#575757".into(),
            color_neutral_stencil_2: "#333333".into(),

            color_compound_brand_foreground_1: "#479ef5".into(),
            color_compound_brand_foreground_1_hover: "#62abf5".into(),
            color_compound_brand_foreground_1_pressed: "#2886de".into(),

            color_compound_brand_background: "#479ef5".into(),
            color_compound_brand_background_hover: "#62abf5".into(),
            color_compound_brand_background_pressed: "#2886de".into(),
            color_compound_brand_stroke: "#479ef5".into(),
            color_compound_brand_stroke_pressed: "#2886de".into(),

            color_brand_background: "#115ea3".into(),
            color_brand_background_hover: "#0f6cbd".into(),
            color_brand_background_pressed: "#0c3b5e".into(),
            color_brand_background_2: "#082338".into(),
            color_brand_foreground_1: "#479ef5".into(),
            color_brand_foreground_2: "#62abf5".into(),
            color_brand_stroke_1: "#479ef5".into(),
            color_brand_stroke_2: "#0e4775".into(),
            color_brand_stroke_2_contrast: "#0e4775".into(),
            color_brand_foreground_link: "#479ef5".into(),
            color_brand_foreground_link_hover: "#62abf5".into(),
            color_brand_foreground_link_pressed: "#2886de".into(),

            color_stroke_focus_2: "#ffffff".into(),

            color_palette_red_background_1: "#3f1011".into(),
            color_palette_red_background_3: "#d13438".into(),
            color_palette_red_foreground_1: "#e37d80".into(),
            color_palette_red_foreground_3: "#e37d80".into(),
            color_palette_red_border_1: "#d13438".into(),
            color_palette_red_border_2: "#e37d80".into(),
            color_palette_green_background_1: "#052505".into(),
            color_palette_green_background_3: "#107c10".into(),
            color_palette_green_foreground_1: "#54b054".into(),
            color_palette_green_foreground_3: "#9fd89f".into(),
            color_palette_green_border_1: "#107c10".into(),
            color_palette_green_border_2: "#9fd89f".into(),
            color_palette_yellow_background_1: "#4c4400".into(),
            color_palette_yellow_background_3: "#fde300".into(),
            color_palette_yellow_foreground_1: "#feee66".into(),
            color_palette_yellow_foreground_2: "#fef7b2".into(),
            color_palette_yellow_border_1: "#fde300".into(),

            color_palette_dark_orange_background_1: "#411200".into(),
            color_palette_dark_orange_background_3: "#da3b01".into(),
            color_palette_dark_orange_foreground_1: "#e9835e".into(),
            color_palette_dark_orange_foreground_3: "#e9835e".into(),
            color_palette_dark_orange_border_1: "#da3b01".into(),

            color_status_success_background_1: "#052505".into(),
            color_status_success_foreground_1: "#54b054".into(),
            color_status_success_border_1: "#107c10".into(),
            color_status_warning_background_1: "#4a1e04".into(),
            color_status_warning_foreground_3: "#f98845".into(),
            color_status_warning_border_1: "#f7630c".into(),
            color_status_danger_background_1: "#3b0509".into(),
            color_status_danger_foreground_1: "#dc626d".into(),
            color_status_danger_border_1: "#c50f1f".into(),

            color_subtle_background: "transparent".into(),
            color_subtle_background_hover: "#383838".into(),
            color_subtle_background_pressed: "#2e2e2e".into(),
            color_transparent_background: "transparent".into(),
            color_transparent_background_hover: "transparent".into(),
            color_transparent_background_pressed: "transparent".into(),
            color_transparent_stroke: "transparent".into(),

            shadow4: "0 0 2px rgba(0,0,0,0.24), 0 2px 4px rgba(0,0,0,0.28)".into(),
            shadow8: "0 0 2px rgba(0,0,0,0.24), 0 4px 8px rgba(0,0,0,0.28)".into(),
            shadow16: "0 0 2px rgba(0,0,0,0.24), 0 8px 16px rgba(0,0,0,0.28)".into(),
            shadow64: "0 0 8px rgba(0,0,0,0.24), 0 32px 64px rgba(0,0,0,0.28)".into(),
        }
    }
}
