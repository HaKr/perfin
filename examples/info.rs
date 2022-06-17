use regex::Regex;

fn main() {
    let re = Regex::new(r"[a-zA-Z][^\s]+( ID)?:\s*").unwrap();
    let name_re = Regex::new(r"[^:]+").unwrap();
    let info = "Naam: GBLT Omschrijving: GBLT incasso maandelijkse termijn Termijn 7 van 10 Vervaldatum 26 jan 2022 Betaalkenmerk 87419536 Heffingjaar 2021 IBAN: NL82DEUT0319804615 Kenmerk: K2BGBL000000086581774 Machtiging ID: 11740450 Incassant ID: NL07ZZZ082053570000 Doorlopende incasso";
    let n = info.len() - 1;
    let bounds: Vec<(usize, usize, usize)> = re
        .find_iter(info)
        .map(|m| (m.start(), m.end(), n))
        .collect();

    let mut bound: Option<usize> = None;
    for (name, val) in bounds.into_iter().rev().map(|(start, mid, end)| {
        let name = name_re.find(&info[start..mid]).unwrap().as_str();
        let str_end = match bound {
            Some(e) => e,
            None => end,
        };
        bound = Some(start);

        let val = &info[mid..str_end];

        (name, val)
    }) {
        println!("Name: {}, Val: {}.", name, val);
    }
}
