pub fn get_aws_path(file_path: &str) -> String {
    format!(
        "https://work-flow-wise.s3.ap-southeast-1.amazonaws.com/{}",
        file_path
    )
    .to_string()
}
