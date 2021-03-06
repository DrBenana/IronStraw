use crate::commands::Command;
use crate::preprocessing::PreprocessedObject;
use crate::processing::full_process;
use crate::config::{get_config, Config};
use crate::utils::get_blocks_as_content;

/// Allows you to use HTML "as is"
/// For example, @raw <b> Hey </b> will return <b> Hey </b>
/// @raw(use_text_as_tag: bool DEFAULT true) Optional:TAG/TEXT
///     Optional: TEXT
pub struct Raw {}

impl Command for Raw {
    fn run(&self, _command: &String, parameters: &Vec<String>, text: &String, _spaces: &usize, blocks: &Vec<Box<PreprocessedObject>>, config: &Config) -> String {
        let use_tag = parameters.get(0).unwrap_or(&"true".to_string()).eq("true");
        let mut raw: Vec<String> = Vec::new();

        if !use_tag {
            raw.push(text.clone());
        }

        raw.push(get_blocks_as_content(&blocks));

        // If we use the tag, we have to the open of the tag in the start and the end of the tag in the end
        if use_tag {
            let processed_tag = full_process(&text, &get_config(false, config.debug));
            let mut splitted_tag = processed_tag.lines();

            // If the amount of the lines is smaller than two, skip
            if splitted_tag.clone().collect::<Vec<&str>>().len() < 2 {
                println!("The text '{}' of the command @raw is invalid. Skipping.", &text);
                return "".to_string();
            }

            return format!(
                "{}\n{}\n{}",
                splitted_tag.nth(0).unwrap(),
                raw.join("\n"),
                splitted_tag.nth(0).unwrap()
            );
        }
        else {
            raw.join("\n")
        }
    }
}