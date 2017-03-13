use regex::Regex;
use regex::RegexSet;
use regex::Captures;


fn must_be_lower(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?i)^(en|sur|et|sous|de|du|des|le|la|les|au|aux|un|une|à)$").unwrap();
    }
    RE.is_match(text)
}

fn must_be_upper(text: &str) -> bool {
    lazy_static! {
        static ref RE: RegexSet =
            RegexSet::new(&[
                r"(?i)^(RER|CDG|CES|ASPTT|PTT|EDF|INRIA|INRA|CRC|HEC|SNCF|RATP|HLM|CHR|CHU)$",
                r"(?i)^(ZA|ZI|RPA|CFA|CEA|CC|CCI|UFR|CPAM|ANPE|RN\d*|\w*\d\w*|RD\d*)$",
                r"(?i)^(XL|X{0,3})(IX|IV|V?I{0,3})$",
                ]).unwrap();
    }
    RE.is_match(text)
}

pub fn sed_all(name: String) -> String {
    if must_be_lower(&name) {
        return name.to_lowercase();
    }

    if must_be_upper(&name) {
        return name.to_uppercase();
    }

    let lower_case = name.to_lowercase();
    if lower_case == "gal" {
        return "Général".to_string();
    } else if lower_case == "mal" {
        return "Maréchal".to_string();
    }

    name
}


pub fn regex_all_name(name: String) -> String {
    lazy_static! {
        static ref RE_SAINT: Regex =
            Regex::new(r"(?i)(^|\W)s(?:ain)?t(e?)\W").unwrap();
    }
    let res = RE_SAINT.replace_all(&name, "${1}Saint${2}-");
    lazy_static! {
        static ref RE_SAINT_LOUIS: Regex =
            Regex::new(r"(?i)(^|\W)Saint-Louis(\W|$)").unwrap();
    }
    let res = RE_SAINT_LOUIS.replace_all(&res, "${1}Saint Louis${2}");

    lazy_static! {
        static ref RE_ND: Regex =
            Regex::new(r"(?i)(^|\W)n(?:otre)?[ -]*d(?:ame)?(\W|$)").unwrap();
    }
    let res = RE_ND.replace_all(&res, "${1}Notre-Dame${2}");

    lazy_static! {
        static ref RE_PLACE: Regex =
            Regex::new(r"(?i)(^|\W)pl\.?(\W|$)").unwrap();
    }
    let res = RE_PLACE.replace_all(&res, "${1}Place${2}");

    lazy_static! {
        static ref RE_BOULEVARD: Regex =
            Regex::new(r"(?i)(^|\W)bl?v?d\.?(\W|$)").unwrap();
    }
    let res = RE_BOULEVARD.replace_all(&res, "${1}Boulevard${2}");

    lazy_static! {
        static ref RE_AVENUE: Regex =
            Regex::new(r"(?i)(^|\W)ave?\.?(\W|$)").unwrap();
    }
    let res = RE_AVENUE.replace_all(&res, "${1}Avenue${2}");

    lazy_static! {
        static ref RE_LIEU_DIT: Regex =
            Regex::new(r"(?i)(^|\W)rte(\W|$)").unwrap();
    }
    let res = RE_LIEU_DIT.replace_all(&res, "${1}Route${2}");

    lazy_static! {
        static ref RE_A: Regex =
            Regex::new(r"(?i) a ").unwrap();
    }
    let res = RE_A.replace_all(&res, " à ");

    lazy_static! {
        static ref RE_ROND_POINT: Regex =
            Regex::new(r"(?i)(^|\W)ro?n?d[ \.-]?po?i?n?t ").unwrap();
    }
    let res = RE_ROND_POINT.replace_all(&res, "${1}Rond-Point ");

    lazy_static! {
        static ref RE_SPACES: Regex =
            Regex::new(r"(?i)  +").unwrap();
    }
    let res = RE_SPACES.replace_all(&res, " ");

    lazy_static! {
        static ref RE_QUOTE: Regex =
            Regex::new(r"(?i)(^|\W)([ld])[ '](h[aiouye]|[aiouy]|et[^ ]|e[^t].)").unwrap();
    }
    let res = RE_QUOTE.replace_all(&res, |caps: &Captures| {
        format!("{}{}'{}", &caps[1], &caps[2].to_lowercase(), &caps[3])
    });

    res.into_owned()
}
