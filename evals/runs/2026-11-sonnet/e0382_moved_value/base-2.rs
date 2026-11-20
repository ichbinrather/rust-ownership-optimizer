struct Report {
    title: String,
    lines: Vec<String>,
}

fn print_report(report: &Report) {
    println!("{}: {} lines", report.title, report.lines.len());
}

fn archive(report: Report) -> usize {
    report.lines.len()
}

fn main() {
    let report = Report { title: "Q3".into(), lines: vec!["a".into(), "b".into()] };
    print_report(&report);
    let n = archive(report);
    println!("archived {n}");
}
