use fancy_regex::Regex;

lazy_static::lazy_static! {
    static ref DIFF4_RE: Regex = Regex::new(r"(.)(?!\1)(.)(?!\1|\2)(.)(?!\1|\2|\3)").unwrap();
    static ref DIFF14_RE: Regex = Regex::new(r"(.)(?!\1)(.)(?!\1|\2)(.)(?!\1|\2|\3)(.)(?!\1|\2|\3|\4)(.)(?!\1|\2|\3|\4|\5)(.)(?!\1|\2|\3|\4|\5|\6)(.)(?!\1|\2|\3|\4|\5|\6|\7)(.)(?!\1|\2|\3|\4|\5|\6|\7|\8)(.)(?!\1|\2|\3|\4|\5|\6|\7|\8|\9)(.)(?!\1|\2|\3|\4|\5|\6|\7|\8|\9|\g{10})(.)(?!\1|\2|\3|\4|\5|\6|\7|\8|\9|\g{10}|\g{11})(.)(?!\1|\2|\3|\4|\5|\6|\7|\8|\9|\g{10}|\g{11}|\g{12})(.)(?!\1|\2|\3|\4|\5|\6|\7|\8|\9|\g{10}|\g{11}|\g{12}|\g{13})(.)").unwrap();
}

pub fn main(parts: crate::RunPart) {
    if parts.run_p1() {
        println!(
            "Part 1: {}",
            DIFF4_RE
                .find(include_str!("../../../input/2022/06.txt"))
                .unwrap()
                .unwrap()
                .end()
                + 1,
        );
    }
    if parts.run_p2() {
        println!(
            "Part 2: Does not run due to unsupported backreferences > 9",
            // DIFF14_RE
            //     .find(include_str!("../../../input/2022/06.txt"))
            //     .unwrap()
            //     .unwrap()
            //     .end() + 1,
        );
    }
}
