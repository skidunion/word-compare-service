pub mod similarity {

    use std::collections::HashMap;
    use ndarray::Array2;

    pub fn damerau_levenshtein_dist(source: &str, target: &str) -> i32 {
        let s: Vec<char> = source.chars().collect();
        let t: Vec<char> = target.chars().collect();

        let inf: i32 = (s.len() + t.len()) as i32;
        let mut matrix = Array2::<i32>::zeros((s.len() + 2, t.len() + 2));

        // В первую строку записываем inf
        for i in 0..t.len() + 2 {
            matrix[[0, i]] = inf;
        }

        // Во вторую строку записываем [inf, 0, ..., |t|]
        matrix[[1, 0]] = inf;

        for i in 1..t.len() + 2 {
            matrix[[1, i]] = (i - 1) as i32;
        }

        // В (2 + |s|) строки записываем [inf, 0..|s|]
        for j in 0..s.len() {
            matrix[[2 + j, 0]] = inf;
            matrix[[2 + j, 1]] = (j + 1) as i32;
        }

        let mut last_row = HashMap::<char, usize>::new();

        for row in 1..s.len() + 1 {
            let s_char = s[row - 1];
            let mut last_match_column = 0;

            for column in 1..t.len() + 1 {
                let t_char = t[column - 1];
                let last_match_row = *last_row.get(&t_char).unwrap_or(&0);
                let cost = if s_char == t_char { 0 } else { 1 };

                matrix[[row + 1, column + 1]] = (matrix[[row, column]] + cost) // Замена
                    .min(matrix[[row + 1, column]] + 1) // Добавление
                    .min(matrix[[row, column + 1]] + 1) // Удаление
                    .min(matrix[[last_match_row, last_match_column]] // Изменение позиции
                        + (row - last_match_row - 1) as i32 + 1
                        + (column - last_match_column - 1) as i32
                    );

                if cost == 0 {
                    last_match_column = column;
                }
            }

            last_row.insert(s_char, row);
        }

        matrix[[s.len() + 1, t.len() + 1]]
    }

    pub fn similarity_with_dist(first: &str, second: &str, distance: i32) -> f32 {
        let first_count = first.chars().count();
        let second_count = second.chars().count();

        let max_length = first_count.max(second_count) as f32;
        let similarity = (max_length - distance as f32) / max_length;

        // only leave 2 decimal places
        f32::trunc(similarity * 100.0) / 100.0
    }

    pub fn similarity(first: &str, second: &str, distance_function: fn(&str, &str) -> i32) -> f32 {
        let distance = distance_function(&first.to_uppercase(), &second.to_uppercase());
        similarity_with_dist(first, second, distance)
    }
}

pub mod transcode {

    use phf::phf_map;

    type StringRefMap = phf::Map<&'static str, &'static str>;
    type CharMap      = phf::Map<char, char>;

    const CYRILLIC_ALPHABET: &str   = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ";
    const CYRILLIC_VOWELS: &[char]  = &['А', 'Е', 'Ё', 'И', 'О', 'Ы', 'Э', 'Я'];

    static STAGE_1_REPLACE_TABLE: StringRefMap = phf_map! {
        "A" => "А",
        "E" => "Е",
        "O" => "О",
        "C" => "С",
        "X" => "Х",
        "B" => "В",
    };

    static STAGE_5_REPLACE_TABLE: CharMap = phf_map! {
        'Б' => 'П',
        'В' => 'Ф',
        'Г' => 'К',
        'Д' => 'Т',
        'З' => 'С',
        'Щ' => 'Ш',
        'Ж' => 'Ш',
        'М' => 'Н',
        'Ю' => 'У',
    };

    static STAGE_6_REPLACE_TABLE: StringRefMap = phf_map! {
        "АКA" => "AФA",
        "AН" => "Н",
        "ЗЧ" => "Ш",
        "ЛНЦ" => "НЦ",
        "ЛФCТФ" => "ЛСТФ",
        "НAТ" => "Н",
        "НТЦ" => "НЦ",
        "НТ" => "Н",
        "НТA" => "НA",
        "НТК" => "НК",
        "НТC" => "НC",
        "НТCК" => "НCК",
        "НТШ" => "НШ",
        "OКO" => "OФO",
        "ПAЛ" => "ПЛ",
        "PТЧ" => "PЧ",
        "PТЦ" => "PЦ",
        "CП" => "CФ",
        "ТCЯ" => "Ц",
        "CТЛ" => "CЛ",
        "CТН" => "CН",
        "CЧ" => "Ш",
        "CШ" => "Ш",
        "ТAТ" => "Т",
        "ТCA" => "Ц",
        "ТAФ" => "ТФ",
        "ТC" => "ТЦ",
        "ТЦ" => "Ц",
        "ТЧ" => "Ч",
        "ФAК" => "ФК",
        "ФCТФ" => "CТФ",
        "ШЧ" => "Ш",
    };

    fn replace_in_string(input: &str, map: &StringRefMap) -> String {
        let mut processed = input.to_string();

        map
            .into_iter()
            .for_each(|(src, tar)| processed = processed.replace(src, tar));

        processed
    }

    fn remove_non_cyrillic_letters(input: &str) -> String {
        input.replace(|p| !CYRILLIC_ALPHABET.contains(p), "")
    }

    fn remove_soft_hard_signs(input: &str) -> String {
        input.replace('Ъ', "").replace('Ь', "")
    }

    const REPLACED_PAIR: char = '\0';

    fn replace_two_char_pairs(input: &str) -> String {
        let mut chars: Vec<char> = input.chars().collect();

        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                chars[i] = REPLACED_PAIR;
            }
        }

        chars.iter().collect()
    }

    fn replace_singulars_by_table_5(input: &str) -> String {
        let mut chars: Vec<char> = input.chars().collect();

        for i in 0..chars.len() - 1 {
            if chars[i + 1] == REPLACED_PAIR {
                continue
            }

            if CYRILLIC_VOWELS.contains(&chars[i]) {
                chars[i] = 'А';
            } else if let Some(replacement) = STAGE_5_REPLACE_TABLE.get(&chars[i]) {
                chars[i] = *replacement;
            }
        }

        chars.iter().collect()
    }

    pub fn polyphone_transcode(input: &str) -> String {
        let mut processed = input.to_string().to_uppercase();

        processed = replace_in_string(&processed, &STAGE_1_REPLACE_TABLE);
        processed = remove_non_cyrillic_letters(&processed);
        processed = remove_soft_hard_signs(&processed);
        processed = replace_two_char_pairs(&processed);
        processed = replace_singulars_by_table_5(&processed);
        processed = replace_in_string(&processed, &STAGE_6_REPLACE_TABLE);

        // remove leftover nul bytes
        processed.replace('\0', "")
    }
}