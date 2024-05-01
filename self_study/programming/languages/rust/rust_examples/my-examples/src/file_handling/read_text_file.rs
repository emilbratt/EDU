pub mod read_text;

#[allow(unused_must_use)]
pub fn run() {
    let path = "./src/file_handling/read_text_file/text.txt";

    read_text::print_file_content(&path);
    read_text::get_content_as_result_with_questionmark_operator(&path);
    read_text::get_content_as_result_with_match(&path);
    read_text::get_content_as_empty_string_if_failed(&path);
}
