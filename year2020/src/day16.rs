use common::day::Day;
use std::collections::HashMap;

pub struct Day16 {}

struct InputInfo {
    field_defs: HashMap<String, FieldDef>,
    your_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

#[derive(Clone)]
struct FieldDef {
    ranges: Vec<(usize, usize)>,
}

fn parse_input(input: &str) -> InputInfo {
    let mut sections = input.split("\n\n");

    let fields_str = sections.next().unwrap();
    let field_defs = parse_field_defs(&fields_str);

    let your_ticket_str = sections.next().unwrap();
    let your_ticket = parse_ticket(&your_ticket_str.lines().nth(1).unwrap());

    let nearby_tickets_str = sections.next().unwrap();
    let nearby_tickets: Vec<_> = nearby_tickets_str
        .lines()
        .skip(1)
        .map(|l| parse_ticket(l))
        .collect();

    InputInfo {
        field_defs,
        your_ticket,
        nearby_tickets,
    }
}

fn parse_field_defs(fields_str: &str) -> HashMap<String, FieldDef> {
    fields_str
        .lines()
        .map(|l| {
            let mut line_parts = l.split(": ");
            let name = String::from(line_parts.next().unwrap());
            let ranges_str = line_parts.next().unwrap();
            let ranges: Vec<_> = ranges_str
                .split(" or ")
                .map(|r| {
                    let range: Vec<_> = r.split('-').map(|x| x.parse::<usize>().unwrap()).collect();
                    (range[0], range[1])
                })
                .collect();
            (name, FieldDef { ranges })
        })
        .collect()
}

fn parse_ticket(ticket_str: &str) -> Vec<usize> {
    ticket_str
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn error_fields(ticket: &[usize], field_defs: &HashMap<String, FieldDef>) -> Vec<usize> {
    ticket
        .iter()
        .filter(|field| {
            !field_defs.values().any(|def| {
                def.ranges
                    .iter()
                    .any(|range| field >= &&range.0 && field <= &&range.1)
            })
        })
        .copied()
        .collect()
}

fn check_row(row_values: &[usize], field_def: &FieldDef) -> bool {
    row_values.iter().all(|v| {
        field_def
            .ranges
            .iter()
            .any(|range| v >= &range.0 && v <= &range.1)
    })
}

fn match_rows(
    rows: &HashMap<usize, Vec<usize>>,
    field_defs: &HashMap<String, FieldDef>,
) -> HashMap<String, usize> {
    let mut new_rows: HashMap<usize, Vec<usize>> = (*rows).clone();
    let mut new_field_defs: HashMap<String, FieldDef> = (*field_defs).clone();
    let mut found_rows = HashMap::new();

    for (row_id, row_values) in rows.iter() {
        let possible_fields: Vec<_> = field_defs
            .iter()
            .filter(|f| check_row(&row_values, f.1))
            .collect();
        if possible_fields.len() == 1 {
            let field_name = possible_fields[0].0;

            new_rows.remove(row_id);
            new_field_defs.remove(field_name);

            found_rows.insert(field_name.clone(), *row_id);
        }
    }
    if !new_rows.is_empty() {
        let new_found_rows = match_rows(&new_rows, &new_field_defs);
        found_rows.extend(new_found_rows);
    }
    found_rows
}

impl Day for Day16 {
    fn star1(&self, input: &str) -> String {
        let input_info = parse_input(input);
        let global_error_rate: usize = input_info
            .nearby_tickets
            .iter()
            .map(|t| {
                error_fields(t, &input_info.field_defs)
                    .iter()
                    .sum::<usize>()
            })
            .sum();
        format!("{}", global_error_rate)
    }

    fn star2(&self, input: &str) -> String {
        let input_info = parse_input(input);
        let mut valid_tickets: Vec<_> = input_info
            .nearby_tickets
            .iter()
            .filter(|t| error_fields(t, &input_info.field_defs).is_empty())
            .collect();
        valid_tickets.push(&input_info.your_ticket);

        let rows: HashMap<usize, Vec<usize>> = (0..input_info.field_defs.len())
            .map(|r| {
                let values: Vec<_> = valid_tickets.iter().map(|t| t[r]).collect();
                (r, values)
            })
            .collect();
        let found_rows = match_rows(&rows, &input_info.field_defs);

        let res: usize = found_rows
            .iter()
            .filter(|x| x.0.starts_with("departure"))
            .map(|x| input_info.your_ticket[*x.1])
            .product();

        format!("{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;
        let d = Day16 {};
        assert_eq!(d.star1(input), "71");
    }
}
