# Rust Grep Tool

Dieses Tool ist eine Rust-Implementierung des Unix-Kommandos `grep`. Es wurde im Rahmen einer Coding Challenge entwickelt. Die vollständige Aufgabenbeschreibung ist [hier](https://codingchallenges.fyi/challenges/challenge-grep/#step-zero) zu finden.

## Entscheidung für das einfache Szenario

In der Implementierung wurde das einfachere Szenario gewählt, unter Verwendung von Rusts Standardbibliotheken und der `regex`-Bibliothek.

## Evaluation

**Entwicklungsumgebung**: Das Tool wurde unter Verwendung von Visual Studio Code und Rust entwickelt.
 
**Rust**: 
- Pattern Matching sehr ungewohnt
- let und mut sind mir bisher komplett unbekannt gewehsen, stellen aber einen wichtigen Bestandteil von Rust da
- Die alternative Lösung von Rust und die Entscheidung gegen einen GC (Garbage Collector) sind ungewohnt

## Funktionalitäten

Das Tool unterstützt verschiedene Funktionen, die den einzelnen Schritten der Aufgabe entsprechen:

- `step1`: Gibt den Inhalt einer Datei aus.
- `step2`: Führt eine einfache Textsuche in einer Datei durch.
- `step3`: Durchsucht rekursiv ein Verzeichnis nach einem Textmuster.
- `step4`: Führt eine invertierte Suche in einer Datei durch.
- `step5`: Verwendet reguläre Ausdrücke für die Suche in einer Datei.
- `step6`: Unterstützt reguläre Ausdrücke mit Zeilenanfang/-ende in einer Datei.
- `final`: Ermöglicht eine case insensitive Search in einer Datei.

## Verwendung

```bash
cargo run <Schritt> <Argumente>

## Beispiele

cargo run step1 test.txt
cargo run step2 "Muster" test.txt
cargo run step3 "Muster" pfad/zum/verzeichnis
cargo run step4 "Muster" test.txt
cargo run step5 "\\d" test.txt
cargo run step6 "^Muster" test.txt
cargo run final "Muster" test.txt -i
```

## Hilfe

Für eine Anleitung zur Verwendung des Tools folgenden Befehl ausführen:

```bash
cargo run -- -h
```