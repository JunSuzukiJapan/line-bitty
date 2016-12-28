extern crate line_botty;
extern crate json_flex;

#[cfg(test)]
mod test {
    #[test]
    fn test_pretty(){
        use json_flex;
        use line_botty::pretty_print::Prettyable;

        let s = "{\"license\":\"\",\"name\":\"foo\",\"scripts\":{\"test\":\"echo \\\"Error: no test specified\\\" && exit 1\"},\"description\":\"\",\"version\":\"1.0.0\",\"author\":\"\",\"main\":\"handler.js\",\"dependencies\":{\"superagent\":\"^3.3.1\"}}".to_owned();
        let decoded = json_flex::decode(s);
        let prettified = decoded.pretty();
    }
}