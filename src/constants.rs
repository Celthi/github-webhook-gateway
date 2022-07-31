
pub static GITHUB_ISSUE_COMMENT_URL:&str = "https://api.github.com/repos/{}/issues/{}/comments";
static KEYWORDS: [&str; 2] = ["keyword1", "keyword2"];

pub fn contains_keywords_we_focus(s: &str) -> bool {
    KEYWORDS.iter().any(|k| s.contains(k))
}
