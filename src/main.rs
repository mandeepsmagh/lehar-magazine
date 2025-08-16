use serde::Deserialize;
use regex::Regex;
use std::fs;
use std::error::Error;

#[derive(Deserialize)]
struct SiteMeta {
    site_name: String,
    default_description: String,
    base_url: String,
    logo: String,
}

#[derive(Deserialize)]
struct Issue {
    title: String,
    pdf: String,
    cover: String,
    description: Option<String>,
}

#[derive(Deserialize)]
struct Metadata {
    site_meta: SiteMeta,
    issues: Vec<Issue>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("metadata.json")?;
    let meta: Metadata = serde_json::from_str(&data)?;
    let sorted = sort_issues(meta.issues);
    
    // Handle case when there are no issues
    let og_tags = if let Some(latest) = sorted.first() {
        format_og_tags(&meta.site_meta, latest)
    } else {
        format_default_og_tags(&meta.site_meta)
    };
    
    let issue_cards = build_issue_cards(&sorted);
    let page_title = &meta.site_meta.site_name;
    let logo_html = if !meta.site_meta.logo.is_empty() {
        format!(r#"<img src="{}" alt="{} Logo">"#, meta.site_meta.logo, meta.site_meta.site_name)
    } else {
        String::new()
    };

    let html_template = fs::read_to_string("index.template.html")?;
    let final_html = html_template
        .replace("{{OG_TAGS}}", &og_tags)
        .replace("{{ISSUE_CARDS}}", &issue_cards)
        .replace("{{PAGE_TITLE}}", page_title)
        .replace("{{LOGO}}", &logo_html);

    fs::write("index.html", final_html)?;
    println!("✅ Successfully generated index.html with {} issues", sorted.len());
    Ok(())
}

fn sort_issues(mut issues: Vec<Issue>) -> Vec<Issue> {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    
    issues.sort_by(|a, b| {
        // Handle cases where regex might not match
        let get_date_tuple = |filename: &str| -> (i32, u32, u32) {
            if let Some(caps) = re.captures(filename) {
                let year: i32 = caps[1].parse().unwrap_or(0);
                let month: u32 = caps[2].parse().unwrap_or(1);
                let day: u32 = caps[3].parse().unwrap_or(1);
                (year, month, day)
            } else {
                (0, 1, 1) // Default for files without date pattern
            }
        };
        
        let (year_a, month_a, day_a) = get_date_tuple(&a.pdf);
        let (year_b, month_b, day_b) = get_date_tuple(&b.pdf);
        
        // Sort in descending order (newest first)
        (year_b, month_b, day_b).cmp(&(year_a, month_a, day_a))
    });
    
    issues
}

fn build_issue_cards(issues: &[Issue]) -> String {
    if issues.is_empty() {
        return r#"<div class="empty-state">
    <h2>No Issues Available</h2>
    <p>Check back soon for new content!</p>
</div>"#.to_string();
    }

    issues.iter().map(|issue| {
        let description = issue.description
            .clone()
            .unwrap_or_else(|| "Download this issue to read the full content.".to_string());
        
        format!(
            r#"<div class="issue-card">
    <div class="image-container">
        <img src="{cover}" alt="{title}" loading="lazy">
    </div>
    <div class="content">
        <h3>{title}</h3>
        <p>{desc}</p>
        <a href="{pdf}" class="download-btn" download>Download PDF</a>
    </div>
</div>"#,
            cover = escape_html(&issue.cover),
            title = escape_html(&issue.title),
            desc = escape_html(&description),
            pdf = escape_html(&issue.pdf)
        )
    }).collect::<Vec<_>>().join("\n")
}

fn format_og_tags(site_meta: &SiteMeta, issue: &Issue) -> String {
    let desc = issue.description
        .clone()
        .unwrap_or_else(|| site_meta.default_description.clone());
    
    format!(
        r#"<meta property="og:title" content="{title} | {site}">
    <meta property="og:description" content="{desc}">
    <meta property="og:image" content="{base}/{cover}">
    <meta property="og:site_name" content="{site}">
    <meta property="og:type" content="website">
    <meta property="og:locale" content="pa_IN">
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:image" content="{base}/{cover}">
    <meta name="description" content="{desc}">"#,
        title = escape_html(&issue.title),
        site = escape_html(&site_meta.site_name),
        desc = escape_html(&desc),
        base = site_meta.base_url.trim_end_matches('/'),
        cover = escape_html(&issue.cover)
    )
}

fn format_default_og_tags(site_meta: &SiteMeta) -> String {
    format!(
        r#"<meta property="og:title" content="{site}">
    <meta property="og:description" content="{desc}">
    <meta property="og:site_name" content="{site}">
    <meta property="og:type" content="website">
    <meta property="og:locale" content="pa_IN">
    <meta name="twitter:card" content="summary">
    <meta name="description" content="{desc}">"#,
        site = escape_html(&site_meta.site_name),
        desc = escape_html(&site_meta.default_description)
    )
}

// Helper function to escape HTML characters
fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}