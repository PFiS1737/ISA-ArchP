use std::iter::repeat_n;

pub fn align_tabbed_lines(lines: &[String]) -> impl Iterator<Item = String> {
    let split_lines: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.split('\t').collect())
        .collect();

    let cols = split_lines[0].len();

    let mut max_width = vec![0usize; cols];
    for row in &split_lines {
        for (i, part) in row.iter().enumerate() {
            max_width[i] = max_width[i].max(part.len());
        }
    }

    split_lines.into_iter().map(move |row| {
        let mut out = String::new();

        for (i, part) in row.iter().enumerate() {
            out += part;

            if i + 1 < cols {
                let padding = max_width[i] - part.len() + 2;
                out.extend(repeat_n(' ', padding));
            }
        }

        out.trim_end().to_string()
    })
}
