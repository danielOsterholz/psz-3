use walkdir::WalkDir;
use regex::Regex;
use std::fs;
use std::env;

// Schritt 1: Ausgabe jeder Zeile bei leerem Suchmuster
fn step1(filename: &str) {
    let content = std::fs::read_to_string(filename)
        .expect("Fehler beim Lesen der Datei");

    println!("{}", content);
}

// Schritt 2: Einfache Mustersuche
fn step2(pattern: &str, filename: &str) {
    let content = std::fs::read_to_string(filename)
        .expect("Fehler beim Lesen der Datei");

    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}

// Schritt 3: Rekursive Suche in einem Verzeichnis.
fn step3(pattern: &str, path: &str) {
    // Durchgehen jedes Eintrags im angegebenen Verzeichnis, einschließlich Unterverzeichnissen.
    for entry in WalkDir::new(path) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,  // Fehlerhafte Einträge überspringen
        };

        // Überprüfen, ob der Eintrag eine Datei ist.
        if entry.file_type().is_file() {
            // Lesen des Dateiinhalts. Fehler werden übersprungen.
            let content = match std::fs::read_to_string(entry.path()) {
                Ok(content) => content,
                Err(_) => continue,
            };

            // Überprüfen jeder Zeile auf das Suchmuster.
            for line in content.lines() {
                if line.contains(pattern) {
                    println!("{}: {}", entry.path().display(), line);
                }
            }
        }
    }
}

// Schritt 4: Invertierte Suche
fn step4(pattern: &str, filename: &str) {
    let content = std::fs::read_to_string(filename)
        .expect("Fehler beim Lesen der Datei");

    for line in content.lines() {
        if !line.contains(pattern) {
            println!("{}", line);
        }
    }
}

// Schritt 5: Unterstützung für die regulären Ausdrücke \d und \w.
fn step5(pattern: &str, filename: &str) {
    // Erstellen eines Regex-Objekts aus dem Muster. Programmabbruch bei ungültigem Muster.
    let re = Regex::new(pattern).expect("Ungültiger regulärer Ausdruck");

    // Lesen des Dateiinhalts. Bei Fehlern wird das Programm beendet.
    let content = std::fs::read_to_string(filename)
        .expect("Fehler beim Lesen der Datei");

    // Überprüfen jeder Zeile auf das Suchmuster.
    for line in content.lines() {
        if re.is_match(line) {
            println!("{}", line);
        }
    }
}

// Schritt 6: Suche von Mustern am Zeilenanfang (^) und -ende ($).
fn step6(pattern: &str, filename: &str) {
    let re = Regex::new(pattern).expect("Ungültiger regulärer Ausdruck");

    let content = std::fs::read_to_string(filename)
        .expect("Fehler beim Lesen der Datei");

    for line in content.lines() {
        if re.is_match(line) {
            println!("{}", line);
        }
    }
}

// Schritt 7: Case Insensitive Search.
fn final_step(pattern: &str, filename: &str, case_insensitive: bool) {
    // Erstellen des Regex-Musters, wobei Groß-/Kleinschreibung beachtet wird, basierend auf dem case_insensitive-Flag.
    let regex_pattern = if case_insensitive {
        format!("(?i){}", pattern)
    } else {
        pattern.to_string()
    };

    // Erstellen des Regex-Objekts aus dem erstellten Muster.
    let re = Regex::new(&regex_pattern).expect("Ungültiger regulärer Ausdruck");

    // Lesen des Dateiinhalts und Fehlerbehandlung.
    let content = fs::read_to_string(filename)
        .expect("Fehler beim Lesen der Datei");

    // Durchsuchen jeder Zeile und Ausgeben der Zeilen, die dem Muster entsprechen.
    for line in content.lines() {
        if re.is_match(line) {
            println!("{}", line);
        }
    }
}

fn print_help() {
    println!("Rust Grep Tool");
    println!("Verwendung: cargo run <Schritt> <Argumente>");
    println!("Schritte:");
    println!("  step1 <Dateiname>                 - Gibt den Inhalt der Datei aus");
    println!("  step2 <Muster> <Dateiname>        - Einfache Textsuche");
    println!("  step3 <Muster> <Verzeichnis>      - Rekursive Verzeichnissuche");
    println!("  step4 <Muster> <Dateiname>        - Invertierte Suche");
    println!("  step5 <Regex> <Dateiname>         - Regex-Suche mit \\d und \\w");
    println!("  step6 <Regex> <Dateiname>         - Regex-Suche mit Zeilenanfang/-ende");
    println!("  final <Muster> <Dateiname> [-i]   - Fallunabhängige Suche");
    println!("Optionen:");
    println!("  -h                                - Hilfe anzeigen");
}

fn main() {
    // step1("test.txt");
    // step2("Gutenberg", "test.txt");
    // step3("Breakfast Club", "challenge-grep");
    // step4("Gutenberg", "test.txt");
    // step5(r"\d", "test.txt"); // oder r"\w" für Wörter
    //step6(r"^Gutenberg", "test.txt"); // r"^Muster" für den Anfang und r"Muster$" für das Ende
    // final_step("Gutenberg", "test.txt", true);

    let args: Vec<String> = env::args().collect();

    // Hilfe anzeigen, wenn -h oder zu wenige Argumente übergeben wurden
    if args.len() < 2 || args.contains(&"-h".to_string()) {
        print_help();
        return;
    }

    match args[1].as_str() {
        "step1" => step1(&args[2]),
        "step2" => step2(&args[2], &args[3]),
        "step3" => step3(&args[2], &args[3]),
        "step4" => step4(&args[2], &args[3]),
        "step5" => step5(&args[2], &args[3]),
        "step6" => step6(&args[2], &args[3]),
        "final" => {
            let case_insensitive = args.contains(&"-i".to_string());
            final_step(&args[2], &args[3], case_insensitive)
        },
        _ => print_help(),
    }
}