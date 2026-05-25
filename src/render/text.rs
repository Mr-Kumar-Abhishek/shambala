#[derive(Debug, Clone)]
pub struct TextRenderer {
    pub font_atlas_id: String,
    pub glyph_size: (u32, u32),
    pub characters_per_row: u32,
}

impl TextRenderer {
    pub fn new(font_atlas_id: &str, glyph_size: (u32, u32), characters_per_row: u32) -> Self {
        Self {
            font_atlas_id: font_atlas_id.to_string(),
            glyph_size,
            characters_per_row,
        }
    }

    pub fn get_glyph_uv(&self, char_code: u8) -> (f32, f32, f32, f32) {
        let row = (char_code as u32) / self.characters_per_row;
        let col = (char_code as u32) % self.characters_per_row;

        let gx = col as f32 * self.glyph_size.0 as f32;
        let gy = row as f32 * self.glyph_size.1 as f32;
        let gw = self.glyph_size.0 as f32;
        let gh = self.glyph_size.1 as f32;

        (gx, gy, gw, gh)
    }

    pub fn text_dimensions(&self, text: &str, scale: f32) -> (f32, f32) {
        let width = text.len() as f32 * self.glyph_size.0 as f32 * scale;
        let height = self.glyph_size.1 as f32 * scale;
        (width, height)
    }

    pub fn wrap_text<'a>(&self, text: &'a str, max_width: f32, scale: f32) -> Vec<&'a str> {
        let char_width = self.glyph_size.0 as f32 * scale;
        let max_chars = (max_width / char_width) as usize;

        let mut lines = Vec::new();
        let mut start = 0;

        while start < text.len() {
            let end = (start + max_chars).min(text.len());
            // Try to break at word boundary
            if end < text.len() {
                if let Some(space_pos) = text[start..end].rfind(' ') {
                    lines.push(&text[start..start + space_pos]);
                    start = start + space_pos + 1;
                    continue;
                }
            }
            lines.push(&text[start..end]);
            start = end;
        }

        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glyph_uv() {
        let renderer = TextRenderer::new("font", (16, 16), 16);
        let (gx, gy, gw, gh) = renderer.get_glyph_uv(b'A');
        assert_eq!(gx, 16.0 * 1.0); // 'A' is ASCII 65, col 1
        assert_eq!(gy, 16.0 * 4.0); // row 4
        assert_eq!(gw, 16.0);
        assert_eq!(gh, 16.0);
    }

    #[test]
    fn test_text_dimensions() {
        let renderer = TextRenderer::new("font", (16, 16), 16);
        let (w, h) = renderer.text_dimensions("Hello", 1.0);
        assert_eq!(w, 80.0); // 5 chars * 16px
        assert_eq!(h, 16.0);
    }

    #[test]
    fn test_text_wrapping() {
        let renderer = TextRenderer::new("font", (16, 16), 16);
        let lines = renderer.wrap_text("Hello World", 80.0, 1.0);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_empty_text() {
        let renderer = TextRenderer::new("font", (16, 16), 16);
        let (w, h) = renderer.text_dimensions("", 1.0);
        assert_eq!(w, 0.0);
        assert_eq!(h, 16.0);
    }
}
