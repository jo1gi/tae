use url::Url;
use crate::config::{Config, Rule};

/// Checks if the param matches the corresponding part of the url
fn url_matches_param(url_data: &Option<&str>, param: &Option<String>) -> bool {
    match (param, url_data) {
        (None, _) => true,
        (Some(param), Some(url_data)) if param == url_data => true,
        _ => false
    }
}

/// Returns true if `rule` matches `url`
fn url_match_rule(url: &Url, rule: &Rule) -> bool {
    [
        (Some(url.scheme()), &rule.scheme),
        (url.host_str(), &rule.host),
        (Some(url.path()), &rule.path),
    ].iter().all(|(url_data, param)| url_matches_param(url_data, param))
}

/// Find the first rule that matches `url`
fn find_matching_rule<'a>(url: &Url, config: &'a Config) -> Option<&'a Rule> {
    config.rules.iter()
        .filter(|rule| url_match_rule(url, rule))
        .next()
}

/// Find a command for `url` based on rules in `config`
pub fn find_command<'a>(url: Url, config: &'a Config) -> Result<(Url, &'a String), crate::Error> {
    let rule = find_matching_rule(&url, config);
    match rule.and_then(|rule| rule.redirect.as_ref()) {
        // Redirect to another url if redirect exists in rule
        Some(redirect) => {
            let new_url = crate::format::format_to_string(&url, &redirect);
            let parsed_url = Url::parse(&new_url)
                .map_err(|_| crate::Error::InvalidRedirect)?;
            find_command(parsed_url, config)
        },
        // Return command and new url
        None => Ok((
            url,
            rule.and_then(|rule| rule.command.as_ref())
                .ok_or(crate::Error::NoRuleFound)?
        )),
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn scheme() {
        let url = url::Url::parse("https://example.com").unwrap();
        let config = super::Config{rules: vec![
            super::Rule {
                scheme: Some("https".to_string()),
                ..Default::default()
            }
        ]};
        let rule = super::find_matching_rule(&url, &config);
        assert!(rule.is_some());
    }

    #[test]
    fn host() {
        let url = url::Url::parse("https://example.com").unwrap();
        let config = super::Config{rules: vec![
            super::Rule {
                host: Some("example.com".to_string()),
                ..Default::default()
            }
        ]};
        let rule = super::find_matching_rule(&url, &config);
        assert!(rule.is_some());
    }

    #[test]
    fn find_none() {
        let url = url::Url::parse("https://example.com").unwrap();
        let config = super::Config{ rules: vec![
            super::Rule {
                scheme: Some("gemini".to_string()),
                ..Default::default()
            }
        ]};
        let rule = super::find_matching_rule(&url, &config);
        assert!(rule.is_none());
    }

}
