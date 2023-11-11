use crate::reg;


pub fn contain_keywords(s: &str) -> bool {
    let pat = reg!(r"(XT ABAs)|(XT I'm running ABAs)|((T|t)hanks\s(?P<t>(\d{1})|(\d\.\d{1,3}))!)");
    pat.is_match(s)
}
pub fn contains_time_spent_pattern(s: &str) -> bool {
    let pat = reg!(r"(T|t)hanks\s(?P<t>(\d{1})|(\d\.\d{1,3}))!");
    pat.captures(s).is_some()
}
pub fn contains_rally_pattern(s: &str) -> bool {
    let pat = reg!("26803fda-2e78-4d9f-931d-84b8261d6f7b");
    pat.is_match(s)
}

pub fn contains_ocr_patten(s: &str) -> bool {
    let pat = reg!(r"(XT ABAs)|(XT I'm running ABAs)");
    pat.is_match(s)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn time_spent() {
        let s = "Thanks 3!";
        assert!(contains_time_spent_pattern(s));
        let s = "Thanks 0.4!";
        assert!(contains_time_spent_pattern(s));
        let s = "hi 26803fda-2e78-4d9f-931d-84b8261d6f7b";
        assert!(contains_rally_pattern(s));
        assert!(contains_ocr_patten("XT ABAs"));
        assert!(contains_ocr_patten("XT I'm running ABAs:"));
        assert!(contain_keywords("XT ABAs"));
        assert!(contain_keywords("XT I'm running ABAs: Hi"));
        assert!(contain_keywords("Thanks 3!"));
        assert!(contain_keywords("Thanks 0.4!"));

    }
}
