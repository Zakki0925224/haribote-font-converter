use std::{fs::File, io::{BufReader, BufRead, Write}};

const INPUT_FILE_PATH: &str = "assets/hankaku.txt";
const OUTPUT_FILE_PATH: &str = "assets/font.rs";


fn main()
{
    let inf = File::open(INPUT_FILE_PATH).unwrap();
    let reader = BufReader::new(inf);
    let mut lines = Vec::new();
    let mut data_list = Vec::new();

    for line in reader.lines()
    {
        let line = line.unwrap();
        lines.push(line);
    }

    for mut i in 0..lines.len()
    {
        let chars: Vec<char> = lines[i].chars().collect();

        if chars.len() == 0 { continue; }
        if &(lines[i])[..4] == "char"
        {
            let mut data = Vec::new();

            for j in 1..17 { data.push(convert_strs_to_u8(&(lines[i+j]))); }
            data_list.push(FontData { data });

            i += 17;
        }
    }

    let mut ouf = File::create(OUTPUT_FILE_PATH).unwrap();
    let mut content = format!("pub const FONT: [[u8; 16]; {}] = [\n", data_list.len());
    let mut data_dec = "".to_string();

    for data in data_list
    {
        let mut strs = "\t[".to_string();
        for d in data.data { strs.push_str(&format!("{},", d).to_string()); }
        strs.push_str("],\n");
        data_dec.push_str(&strs);
    }

    content.push_str(&data_dec);
    content.push_str("];");

    ouf.write_all(content.as_bytes()).unwrap();
}

fn convert_strs_to_u8(strs: &str) -> String
{
    let mut result = 0;
    let mut bytes = Vec::new();

    // . is 0, * is 1
    // ..***... is 0x38
    // .*...*.. is 0x44

    for c in strs.chars()
    {
        if c == '.' { bytes.push(false); }
        if c == '*' { bytes.push(true); }
    }

    for i in 0..bytes.len()
    {
        if bytes[i] { result |= 1 << bytes.len() - i - 1; }
    }

    return format!("0x{:x}", result);
}

#[derive(Debug)]
struct FontData
{
    data: Vec<String>
}