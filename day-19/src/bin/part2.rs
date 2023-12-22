use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Copy, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<&str> for Category {
    fn from(s: &str) -> Self {
        match s {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            _ => panic!("unknown key"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl std::ops::Index<Category> for Part {
    type Output = usize;

    fn index(&self, category: Category) -> &Self::Output {
        match category {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }
}

#[derive(Debug)]
enum Rule {
    Condition(Category, std::cmp::Ordering, usize, Decision),
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
            let (category, value) = condition
                .trim()
                .split_once(operator)
                .expect("<category><operator><value>");
            let operator = match operator {
                "<" => std::cmp::Ordering::Less,
                ">" => std::cmp::Ordering::Greater,
                _ => panic!("unknown operator"),
            };
            Rule::Condition(
                category.into(),
                operator,
                value.parse().expect("number"),
                decision.into(),
            )
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

fn num_accepted_combinations(
    workflows: &HashMap<String, Workflow>,
    current_workflow: &Workflow,
    current_rule_index: usize,
    part_from: Part,
    part_to: Part,
) -> usize {
    let rule = current_workflow
        .rules
        .get(current_rule_index)
        .expect("valid rule");

    // dbg!(part_from, part_to, rule);

    match rule {
        Rule::Decision(decision) => match decision {
            Decision::Accept => {
                (1 + part_to.x - part_from.x)
                    * (1 + part_to.m - part_from.m)
                    * (1 + part_to.a - part_from.a)
                    * (1 + part_to.s - part_from.s)
            }
            Decision::Reject => 0,
            Decision::SendTo(workflow_name) => num_accepted_combinations(
                workflows,
                workflows.get(workflow_name).expect("valid workflow name"),
                0,
                part_from,
                part_to,
            ),
        },

        Rule::Condition(category, op, value, decision) => {
            // the condition has zero overlap with the parts
            if (*op == Ordering::Less && value < &part_from[*category])
                || (*op == Ordering::Greater && &part_to[*category] < value)
            {
                // just continue to the next rule, this can't be matched
                num_accepted_combinations(
                    workflows,
                    current_workflow,
                    current_rule_index + 1,
                    part_from,
                    part_to,
                )
            } else if (*op == Ordering::Less && value > &part_to[*category])
                || (*op == Ordering::Greater && &part_from[*category] > value)
            {
                // always accept this rule, it must be matched

                return match decision {
                    Decision::Accept => {
                        (1 + part_to.x - part_from.x)
                            * (1 + part_to.m - part_from.m)
                            * (1 + part_to.a - part_from.a)
                            * (1 + part_to.s - part_from.s)
                    }
                    Decision::Reject => 0,
                    Decision::SendTo(workflow_name) => num_accepted_combinations(
                        workflows,
                        workflows.get(workflow_name).expect("valid workflow name"),
                        0,
                        part_from,
                        part_to,
                    ),
                };
            } else {
                // value is between part_from and part_to
                // so we need to split the part into two parts
                let mut middle_left = *value;
                let mut middle_right = *value;
                match op {
                    Ordering::Less => middle_left -= 1,
                    Ordering::Greater => middle_right += 1,
                    _ => unreachable!(),
                };

                let left_side_from = part_from;
                let mut left_side_to = part_to;

                let mut right_side_from = part_from;
                let right_side_to = part_to;

                match category {
                    Category::X => {
                        left_side_to.x = middle_left;
                        right_side_from.x = middle_right;
                    }
                    Category::M => {
                        left_side_to.m = middle_left;
                        right_side_from.m = middle_right;
                    }
                    Category::A => {
                        left_side_to.a = middle_left;
                        right_side_from.a = middle_right;
                    }
                    Category::S => {
                        left_side_to.s = middle_left;
                        right_side_from.s = middle_right;
                    }
                }

                let mut accepted_from = left_side_from;
                let mut accepted_to = left_side_to;

                let mut declined_from = right_side_from;
                let mut declined_to = right_side_to;

                if *op == Ordering::Greater {
                    (accepted_from, accepted_to, declined_from, declined_to) =
                        (declined_from, declined_to, accepted_from, accepted_to);
                }
                let (part_from, part_to) = (accepted_from, accepted_to);

                return num_accepted_combinations(
                    workflows,
                    current_workflow,
                    current_rule_index + 1,
                    declined_from,
                    declined_to,
                ) + match decision {
                    Decision::Accept => {
                        (1 + part_to.x - part_from.x)
                            * (1 + part_to.m - part_from.m)
                            * (1 + part_to.a - part_from.a)
                            * (1 + part_to.s - part_from.s)
                    }
                    Decision::Reject => 0,
                    Decision::SendTo(workflow_name) => num_accepted_combinations(
                        workflows,
                        workflows.get(workflow_name).expect("valid workflow name"),
                        0,
                        part_from,
                        part_to,
                    ),
                };
            }
        }
    }
}

fn part2(input: &str) -> usize {
    input.trim().to_string();

    let (workflow_input, _parts_input) = input
        .split_once("\n\n")
        .expect("<workflows> <empty line> <parts>");

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    for workflow in workflow_input.lines() {
        let workflow: Workflow = workflow.into();
        workflows.insert(workflow.name.clone(), workflow);
    }

    // dbg!(&workflows);

    let combinations = num_accepted_combinations(
        &workflows,
        workflows.get("in").expect("in"),
        0,
        Part {
            x: 1,
            m: 1,
            a: 1,
            s: 1,
        },
        Part {
            x: 4000,
            m: 4000,
            a: 4000,
            s: 4000,
        },
    );

    combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part2(include_str!("example.txt")), 167409079868000);
    }
}
