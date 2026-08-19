use std::{collections::HashMap, hash::Hash, iter::repeat_n};

pub fn merge_maps<K, V1, V2, A, B>(a: A, b: B) -> HashMap<K, (Option<V1>, Option<V2>)>
where
    K: Hash + Eq,
    A: IntoIterator<Item = (K, V1)>,
    B: IntoIterator<Item = (K, V2)>,
{
    let mut res = HashMap::new();

    for (k, v) in a {
        res.insert(k, (Some(v), None));
    }

    for (k, v) in b {
        if let Some(entry) = res.get_mut(&k) {
            entry.1 = Some(v);
        } else {
            res.insert(k, (None, Some(v)));
        }
    }

    res
}

pub fn align_tabbed_lines(lines: HashMap<usize, String>) -> HashMap<usize, String> {
    let split_lines: Vec<Vec<&str>> = lines
        .values()
        .map(|line| line.split('\t').collect())
        .collect();

    let cols = split_lines[0].len();

    let mut max_width = vec![0usize; cols];
    for row in &split_lines {
        for (i, part) in row.iter().enumerate() {
            max_width[i] = max_width[i].max(part.len());
        }
    }

    let values = split_lines.into_iter().map(|row| {
        let mut out = String::new();

        for (i, part) in row.iter().enumerate() {
            out += part;

            if i + 1 < cols {
                let padding = max_width[i] - part.len() + 2;
                out.extend(repeat_n(' ', padding));
            }
        }

        out.trim_end().to_string()
    });

    lines.keys().copied().zip(values).collect()
}
