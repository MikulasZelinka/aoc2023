use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

// Part structure looks like this:
// {x=787,m=2655,a=1222,s=2876}
// {x=1679,m=44,a=2067,s=496}
// {x=2036,m=264,a=79,s=2244}
// {x=2461,m=1339,a=466,s=291}
// {x=2127,m=1623,a=2188,s=1013}

#[derive(Debug)]
enum Category {
    X(u32),
    M(u32),
    A(u32),
    S(u32),
}
impl Category {
    fn value(&self) -> u32 {
        match self {
            Category::X(value) => *value,
            Category::M(value) => *value,
            Category::A(value) => *value,
            Category::S(value) => *value,
        }
    }
}

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        let (key, value) = s
            .split_once(|c| c == '<' || c == '>' || c == '=')
            .expect("<key>=<value>");
        match key {
            "x" => Category::X(value.parse().expect("X value is a number")),
            "m" => Category::M(value.parse().expect("M value is a number")),
            "a" => Category::A(value.parse().expect("A value is a number")),
            "s" => Category::S(value.parse().expect("S value is a number")),
            _ => panic!("unknown key"),
        }
    }
}

#[derive(Debug)]
struct Part {
    x: Category,
    m: Category,
    a: Category,
    s: Category,
}

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        let parts = s.trim_start_matches('{').trim_end_matches('}').split(',');

        let mut x: Option<Category> = None;
        let mut m: Option<Category> = None;
        let mut a: Option<Category> = None;
        let mut s: Option<Category> = None;

        for part in parts {
            let category: Category = part.trim().into();
            match category {
                Category::X(_) => x = Some(category),
                Category::M(_) => m = Some(category),
                Category::A(_) => a = Some(category),
                Category::S(_) => s = Some(category),
            }
        }
        Part {
            x: x.expect("x is present"),
            m: m.expect("m is present"),
            a: a.expect("a is present"),
            s: s.expect("s is present"),
        }
    }
}

// impl rating for a part, a sum of all values
impl Part {
    fn rating(&self) -> u32 {
        self.x.value() + self.m.value() + self.a.value() + self.s.value()
    }
}

// Each workflow has a name and contains a list of rules; each rule specifies a condition and where to send the part if the condition is true. The first rule that matches the part being considered is applied immediately, and the part moves on to the destination described by the rule. (The last rule in each workflow has no condition and always applies if reached.)

// Consider the workflow ex{x>10:one,m<20:two,a>30:R,A}. This workflow is named ex and contains four rules. If workflow ex were considering a specific part, it would perform the following steps in order:

//     Rule "x>10:one": If the part's x is more than 10, send the part to the workflow named one.
//     Rule "m<20:two": Otherwise, if the part's m is less than 20, send the part to the workflow named two.
//     Rule "a>30:R": Otherwise, if the part's a is more than 30, the part is immediately rejected (R).
//     Rule "A": Otherwise, because no other rules matched the part, the part is immediately accepted (A).

#[derive(Debug)]
enum Rule {
    Condition(Category, std::cmp::Ordering, Decision),
    Decision(Decision),
}

#[derive(Debug)]
enum Decision {
    Accept,
    Reject,
    SendTo(String),
}

impl From<&str> for Decision {
    fn from(s: &str) -> Self {
        match s {
            "A" => Decision::Accept,
            "R" => Decision::Reject,
            _ => Decision::SendTo(s.to_string()),
        }
    }
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        // if rule contains ":" then it's a condition
        // otherwise it's a decision
        if s.contains(':') {
            let (condition, decision) = s.split_once(':').expect("<condition>:<decision>");
            let operator = {
                if condition.contains('<') {
                    "<"
                } else if condition.contains('>') {
                    ">"
                } else {
                    panic!("unknown operator")
                }
            };
            let category = condition.trim().into();
            let operator = match operator {
                "<" => std::cmp::Ordering::Less,
                ">" => std::cmp::Ordering::Greater,
                _ => panic!("unknown operator"),
            };
            Rule::Condition(category, operator, decision.into())
        } else {
            Rule::Decision(s.into())
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (name, rules) = value.split_once('{').expect("<name>:<rules>");
        let rules = rules
            .trim_end_matches('}')
            .split(',')
            .map(|rule| rule.into())
            .collect();
        Workflow {
            name: name.to_string(),
            rules,
        }
    }
}

fn part1(input: &str) -> u32 {
    input.trim().to_string();

    let (workflow_input, parts_input) = input
        .split_once("\n\n")
        .expect("<workflows> <empty line> <parts>");

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    for workflow in workflow_input.lines() {
        let workflow: Workflow = workflow.into();
        workflows.insert(workflow.name.clone(), workflow);
    }

    let parts: Vec<Part> = parts_input.lines().map(|part| part.into()).collect();

    dbg!(&workflows);
    dbg!(&parts);

    let mut sum_of_ratings = 0;
    for part in parts {
        // let mut current_workflow_name = "in";
        let mut current_workflow = workflows.get("in").expect("valid workflow name");
        let mut current_rule_index = 0;

        loop {
            let rule = current_workflow
                .rules
                .get(current_rule_index)
                .expect("valid rule");
            match rule {
                Rule::Decision(decision) => match decision {
                    Decision::Accept => {
                        sum_of_ratings += part.rating();
                        break;
                    }
                    Decision::Reject => break,
                    Decision::SendTo(workflow_name) => {
                        current_workflow =
                            workflows.get(workflow_name).expect("valid workflow name");
                        current_rule_index = 0;
                    }
                },

                Rule::Condition(category, operator, decision) => {
                    let value = match category {
                        Category::X(value) => *value,
                        Category::M(value) => *value,
                        Category::A(value) => *value,
                        Category::S(value) => *value,
                    };
                    let part_value = match category {
                        Category::X(_) => part.x.value(),
                        Category::M(_) => part.m.value(),
                        Category::A(_) => part.a.value(),
                        Category::S(_) => part.s.value(),
                    };
                    if part_value.cmp(&value) == *operator {
                        match decision {
                            Decision::Accept => {
                                sum_of_ratings += part.rating();
                                break;
                            }
                            Decision::Reject => break,
                            Decision::SendTo(workflow_name) => {
                                current_workflow =
                                    workflows.get(workflow_name).expect("valid workflow name");
                                current_rule_index = 0;
                            }
                        };
                    } else {
                        current_rule_index += 1;
                    }
                }
            }
        }
    }

    sum_of_ratings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part1(include_str!("example.txt")), 19114);
    }
}
