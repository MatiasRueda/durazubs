use std::collections::HashMap;

use crate::model::format::ass::parser::{parser::Parser, parser_error::ParseRes};

enum ConfigType {
    PlayResX,
    PlayResY,
    ScaledBorder,
}

pub struct ScriptPreparer<'a> {
    is_script_info: bool,
    has_scaled_border: bool,
    has_playresx: bool,
    has_playresy: bool,
    post_info_idx: Option<usize>,
    style_counter: HashMap<String, usize>,
    last_style_idx: Option<usize>,
    is_v4_styles: bool,
    parser: &'a Parser,
}

impl<'a> ScriptPreparer<'a> {
    const RES_X: u32 = 640;
    const RES_Y: u32 = 360;
    const SCALED_DEFAULT: bool = true;

    pub fn new(parser: &'a Parser) -> Self {
        Self {
            is_script_info: false,
            has_scaled_border: false,
            has_playresx: false,
            has_playresy: false,
            post_info_idx: None,
            style_counter: HashMap::new(),
            last_style_idx: None,
            is_v4_styles: false,
            parser,
        }
    }

    fn update_state(&mut self, line: &str) {
        match line {
            _ if self.parser.is_script_info(line) => self.enter_info(),
            _ if self.parser.is_v4_styles(line) => self.enter_styles(),
            _ if self.parser.is_section_start(line) => self.exit_critical_sections(),
            _ => {}
        }
    }

    fn enter_info(&mut self) {
        self.is_script_info = true;
        self.is_v4_styles = false;
    }

    fn enter_styles(&mut self) {
        self.is_v4_styles = true;
        self.is_script_info = false;
    }

    fn exit_critical_sections(&mut self) {
        self.is_script_info = false;
        self.is_v4_styles = false;
    }

    fn set_val(out: &mut Vec<String>, val: &str, flag: &mut bool) -> bool {
        *flag = true;
        out.push(val.to_string());
        true
    }

    fn get_config(&self, t: ConfigType) -> String {
        match t {
            ConfigType::PlayResX => self.parser.format_playresx(Self::RES_X),
            ConfigType::PlayResY => self.parser.format_playresy(Self::RES_Y),
            ConfigType::ScaledBorder => self.parser.format_scaled_border(Self::SCALED_DEFAULT),
        }
    }

    fn proc_info_line(&mut self, line: &str, out: &mut Vec<String>) -> bool {
        match line {
            _ if self.parser.is_playresx(line) => self.proc_resx(out),
            _ if self.parser.is_playresy(line) => self.proc_resy(out),
            _ if self.parser.is_scaled_border(line) => self.proc_scaled(out),
            _ if line.trim().is_empty() => self.finalize_info(out),
            _ => false,
        }
    }

    fn proc_resx(&mut self, out: &mut Vec<String>) -> bool {
        let v = self.get_config(ConfigType::PlayResX);
        Self::set_val(out, &v, &mut self.has_playresx)
    }

    fn proc_resy(&mut self, out: &mut Vec<String>) -> bool {
        let v = self.get_config(ConfigType::PlayResY);
        Self::set_val(out, &v, &mut self.has_playresy)
    }

    fn proc_scaled(&mut self, out: &mut Vec<String>) -> bool {
        let v = self.get_config(ConfigType::ScaledBorder);
        Self::set_val(out, &v, &mut self.has_scaled_border)
    }

    fn inject_missing(&self, out: &mut Vec<String>) {
        if !self.has_playresx {
            out.push(self.get_config(ConfigType::PlayResX));
        }
        if !self.has_playresy {
            out.push(self.get_config(ConfigType::PlayResY));
        }
        if !self.has_scaled_border {
            out.push(self.get_config(ConfigType::ScaledBorder));
        }
    }

    fn finalize_info(&mut self, out: &mut Vec<String>) -> bool {
        self.inject_missing(out);
        self.close_info_block(out.len());
        false
    }

    fn close_info_block(&mut self, current_pos: usize) {
        self.is_script_info = false;
        if self.post_info_idx.is_none() {
            self.post_info_idx = Some(current_pos);
        }
    }

    fn count_if_dialogue(&mut self, line: &str) -> ParseRes<()> {
        if self.parser.is_dialogue(line) {
            let style = self.parser.get_style(line)?;
            *self.style_counter.entry(style).or_insert(0) += 1;
        }
        Ok(())
    }

    pub fn post_info_idx(&self, out: &Vec<String>) -> usize {
        self.post_info_idx.unwrap_or(out.len())
    }

    pub fn post_style_idx(&self) -> Option<usize> {
        self.last_style_idx
    }

    pub fn dominant_style(&self) -> Option<String> {
        self.style_counter
            .iter()
            .max_by_key(|(_, c)| *c)
            .map(|(s, _)| s.clone())
    }

    fn try_proc_info(&mut self, trim: &str, out: &mut Vec<String>) -> bool {
        if self.is_script_info {
            return self.proc_info_line(trim, out);
        }
        false
    }

    fn track_style_idx(&mut self, line: &str, pos: usize) {
        if self.is_v4_styles && self.parser.is_style_definition(line) {
            self.last_style_idx = Some(pos);
        }
    }

    fn proc_standard_line(&mut self, line: &str, out: &mut Vec<String>) -> ParseRes<()> {
        self.track_style_idx(line, out.len());
        self.count_if_dialogue(line)?;
        out.push(line.to_string());
        Ok(())
    }

    pub fn analyze(&mut self, lines: &[String], out: &mut Vec<String>) -> ParseRes<()> {
        for line in lines {
            let trim = line.trim();
            self.update_state(trim);
            if !self.try_proc_info(trim, out) {
                self.proc_standard_line(line, out)?;
            }
        }
        Ok(())
    }
}
