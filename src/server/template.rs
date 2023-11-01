pub fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>es-rs web server</title>
  </head>
  <body>
    {}
  </body>
</html>
  "#,
        content.as_ref()
    )
}
