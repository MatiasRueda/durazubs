use regex::Regex;
use std::result::Result;

use crate::model::format::ass::{
    line_ass::{ASS_FIELDS_COUNT, AssField, AssLine},
    parser::parser_error::ParserError,
};

pub struct Parser {
    style_re: Regex,
    tags_re: Regex,
    additional_scene_re: Regex,
}

impl Parser {
    const SCRIPT_INFO_PREFIX: &str = "[Script Info]";
    const STYLES_PREFIX: &str = "[V4+ Styles]";
    const STYLE_PREFIX: &str = "Style:";
    const PLAYRESX_PREFIX: &str = "PlayResX:";
    const PLAYRESY_PREFIX: &str = "PlayResY:";
    const SCALED_PREFIX: &str = "ScaledBorderAndShadow:";
    const DIALOGUE_PREFIX: &str = "Dialogue: ";
    const EVENTS_PREFIX: &str = "[Events]";
    const FORMAT_PREFIX: &str = "Format:";

    const SPECIAL_STYLES_RE: &str = r"(?i)Opening|Ending|OP|ED";
    const TAGS_EXPRESSION_RE: &str = r"\{.*?\}";
    const ADDITIONAL_SCENE_RE: &str = r"(?i)(additional scene|extra scene|bonus scene|special)";

    const STYLES_FORMAT: &str = "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding";

    const FX_KEYWORD: &str = "fx";
    const TEMPLATE_KEYWORD: &str = "template";
    const CODE_KEYWORD: &str = "code";
    const EXCESSIVE_TAGS_THRESHOLD: usize = 50;

    pub fn new() -> Self {
        Self {
            style_re: Regex::new(Self::SPECIAL_STYLES_RE).unwrap(),
            tags_re: Regex::new(Self::TAGS_EXPRESSION_RE).unwrap(),
            additional_scene_re: Regex::new(Self::ADDITIONAL_SCENE_RE).unwrap(),
        }
    }

    fn strip_dialogue(&self, line: &str) -> Result<String, ParserError> {
        match line.strip_prefix(Self::DIALOGUE_PREFIX) {
            Some(stripped) => Ok(stripped.to_string()),
            None => Err(ParserError::DialoguePrefix),
        }
    }

    fn split_line(&self, line: &str) -> Result<Vec<String>, ParserError> {
        let parts: Vec<String> = line
            .splitn(ASS_FIELDS_COUNT, ',')
            .map(|s| s.to_string())
            .collect();

        if parts.len() >= ASS_FIELDS_COUNT {
            Ok(parts)
        } else {
            Err(ParserError::MissingFields { found: parts.len() })
        }
    }

    fn build_line_key(&self, start: &str, end: &str, style: &str) -> String {
        format!("{}|{}|{}", start, end, style)
    }

    fn is_additional_scene(&self, line: &str) -> Result<bool, ParserError> {
        Ok(self
            .additional_scene_re
            .is_match(&self.parse_ass_line(line)?.name))
    }

    fn ass_line_to_string(&self, l: &AssLine) -> String {
        format!(
            "Dialogue: 10,{},{},{},{},0,0,0,{},{}",
            self.seconds_to_hms(l.start),
            self.seconds_to_hms(l.end),
            l.style,
            l.name,
            l.effect,
            l.text
        )
    }

    fn parse_time(&self, t: &str) -> f64 {
        let parts: Vec<&str> = t.split(':').collect();
        let h: f64 = parts[0].parse().unwrap();
        let m: f64 = parts[1].parse().unwrap();

        let (ss, cs) = {
            let s = parts[2];
            let s_parts: Vec<&str> = s.split('.').collect();
            (
                s_parts[0].parse::<f64>().unwrap(),
                s_parts[1].parse::<f64>().unwrap(),
            )
        };

        h * 3600.0 + m * 60.0 + ss + cs / 100.0
    }

    fn seconds_to_hms(&self, seg: f64) -> String {
        let h = (seg / 3600.0).floor() as i32;
        let m = ((seg % 3600.0) / 60.0).floor() as i32;
        let s = (seg % 60.0).floor() as i32;
        let cs = ((seg - seg.floor()) * 100.0).round() as i32;
        format!("{h}:{m:02}:{s:02}.{cs:02}")
    }

    pub fn parse_ass_line(&self, line: &str) -> Result<AssLine, ParserError> {
        let stripped = self.strip_dialogue(line)?;
        let parts = self.split_line(&stripped)?;
        Ok(AssLine {
            start: self.parse_time(&parts[AssField::Start.index()]),
            end: self.parse_time(&parts[AssField::End.index()]),
            style: parts[AssField::Style.index()].trim().to_string(),
            name: parts[AssField::Name.index()].trim().to_string(),
            effect: parts[AssField::Effect.index()].trim().to_string(),
            text: parts[AssField::Text.index()].trim().to_string(),
        })
    }

    pub fn get_styles_tag(&self) -> String {
        Self::STYLES_PREFIX.to_string()
    }

    pub fn get_styles_format(&self) -> String {
        Self::STYLES_FORMAT.to_string()
    }

    pub fn get_new_offset(
        &self,
        next_line: &str,
        next_block_line: &str,
    ) -> Result<f64, ParserError> {
        let next_ass = self.parse_ass_line(next_line)?;
        let next_block_ass = self.parse_ass_line(next_block_line)?;
        Ok(next_block_ass.start - next_ass.start)
    }

    pub fn get_style(&self, line: &str) -> Result<String, ParserError> {
        Ok(self.parse_ass_line(line)?.style)
    }

    pub fn get_text(&self, line: &str) -> Result<String, ParserError> {
        Ok(self.parse_ass_line(line)?.text)
    }

    pub fn get_start_time(&self, line: &str) -> Result<f64, ParserError> {
        Ok(self.parse_ass_line(line)?.start)
    }

    pub fn get_end_time(&self, line: &str) -> Result<f64, ParserError> {
        Ok(self.parse_ass_line(line)?.end)
    }

    pub fn get_line_key(&self, line: &str) -> Result<String, ParserError> {
        let ass_line = self.parse_ass_line(line)?;
        Ok(self.build_line_key(
            &ass_line.start.to_string(),
            &ass_line.end.to_string(),
            &ass_line.style,
        ))
    }

    pub fn set_start_time(&self, line: &str, new_start: f64) -> Result<String, ParserError> {
        let mut ass_line = self.parse_ass_line(line)?;
        ass_line.start = new_start;
        Ok(self.ass_line_to_string(&ass_line))
    }

    pub fn set_end_time(&self, line: &str, new_end: f64) -> Result<String, ParserError> {
        let mut ass_line = self.parse_ass_line(line)?;
        ass_line.end = new_end;
        Ok(self.ass_line_to_string(&ass_line))
    }

    pub fn is_scene_line(&self, line: &str) -> Result<bool, ParserError> {
        Ok(self.is_dialogue(line) && self.is_additional_scene(line)?)
    }

    pub fn is_special_style(&self, line: &str) -> Result<bool, ParserError> {
        Ok(self.style_re.is_match(&self.parse_ass_line(line)?.style))
    }

    pub fn is_text_empty(&self, line: &str) -> Result<bool, ParserError> {
        let text = self.parse_ass_line(line)?.text;
        Ok(self.tags_re.replace_all(&text, "").trim().is_empty())
    }

    pub fn has_excessive_tags(&self, text: &str) -> bool {
        let total_tags_len: usize = self.tags_re.find_iter(text).map(|m| m.as_str().len()).sum();
        total_tags_len > Self::EXCESSIVE_TAGS_THRESHOLD
    }

    fn is_technical_effect(&self, effect: &str) -> bool {
        let lower_effect = effect.to_lowercase();
        let trimmed = lower_effect.trim();

        trimmed == Self::FX_KEYWORD
            || trimmed.starts_with(Self::TEMPLATE_KEYWORD)
            || trimmed.starts_with(Self::CODE_KEYWORD)
    }

    pub fn is_technical_garbage(&self, line: &str) -> Result<bool, ParserError> {
        let ass_line = self.parse_ass_line(line)?;
        if self.is_technical_effect(&ass_line.effect) {
            return Ok(true);
        }
        if self.has_excessive_tags(&ass_line.text) {
            return Ok(true);
        }
        let clean_text = self
            .tags_re
            .replace_all(&ass_line.text, "")
            .trim()
            .to_string();

        Ok(clean_text.is_empty())
    }

    pub fn is_events_section(&self, line: &str) -> bool {
        line.starts_with(Self::EVENTS_PREFIX)
    }

    pub fn is_format_line(&self, line: &str) -> bool {
        line.starts_with(Self::FORMAT_PREFIX)
    }

    pub fn is_dialogue(&self, line: &str) -> bool {
        line.starts_with(Self::DIALOGUE_PREFIX)
    }

    pub fn is_script_info(&self, line: &str) -> bool {
        line.contains(Self::SCRIPT_INFO_PREFIX)
    }

    pub fn is_v4_styles(&self, line: &str) -> bool {
        line.trim() == Self::STYLES_PREFIX
    }

    pub fn is_section_start(&self, line: &str) -> bool {
        line.trim().starts_with('[')
    }

    pub fn is_playresx(&self, line: &str) -> bool {
        line.trim().starts_with(Self::PLAYRESX_PREFIX)
    }

    pub fn is_playresy(&self, line: &str) -> bool {
        line.trim().starts_with(Self::PLAYRESY_PREFIX)
    }

    pub fn is_scaled_border(&self, line: &str) -> bool {
        line.trim().starts_with(Self::SCALED_PREFIX)
    }

    pub fn format_playresx(&self, value: u32) -> String {
        format!("{} {}", Self::PLAYRESX_PREFIX, value)
    }

    pub fn format_playresy(&self, value: u32) -> String {
        format!("{} {}", Self::PLAYRESY_PREFIX, value)
    }

    fn bool_to_str(active: bool) -> &'static str {
        if active { "yes" } else { "no" }
    }

    pub fn format_scaled_border(&self, active: bool) -> String {
        format!("{} {}", Self::SCALED_PREFIX, Self::bool_to_str(active))
    }

    pub fn is_style_definition(&self, line: &str) -> bool {
        line.trim().starts_with(Self::STYLE_PREFIX)
    }

    pub fn replace_text(&self, line: &str, translation: &str) -> Result<String, ParserError> {
        let mut ass_line = self.parse_ass_line(line)?;
        ass_line.text = translation.to_string();
        Ok(self.ass_line_to_string(&ass_line))
    }

    pub fn replace_style(&self, line: &str, style: &str) -> Result<String, ParserError> {
        let mut ass_line = self.parse_ass_line(line)?;
        ass_line.style = style.to_string();
        Ok(self.ass_line_to_string(&ass_line))
    }
}
