//! Soundness regression tests for issue #2036:
//! Rejecting `nil` in non-nilable composite literals (HYB-E1002 / Row 13 BUILDABLE invariant).

use hy_check::{check_file, Verdict};
use hy_diag::Severity;
use hy_syntax::parse_file;

fn check(src: &str) -> hy_check::CheckResult {
    let ast = parse_file("test.hy", src).expect("test snippet must parse");
    check_file(&ast)
}

fn error_codes(r: &hy_check::CheckResult) -> Vec<String> {
    r.diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .map(|d| d.code.clone())
        .collect()
}

#[test]
fn rejects_nil_in_non_nilable_slice_literals() {
    let cases = [
        "fn main() { s := []int{nil}; _ = s }",
        "fn main() { s := []string{nil}; _ = s }",
        "fn main() { s := []bool{nil}; _ = s }",
        "fn main() { s := []float64{nil}; _ = s }",
    ];

    for src in cases {
        let r = check(src);
        assert_eq!(r.verdict, Verdict::Reject, "expected Reject for:\n{src}");
        assert!(
            error_codes(&r).contains(&"HYB-E1002".to_string()),
            "expected HYB-E1002 for:\n{src}, got: {:?}",
            error_codes(&r)
        );
    }
}

#[test]
fn rejects_nil_in_non_nilable_array_literals() {
    let cases = [
        "fn main() { a := [2]int{nil, 1}; _ = a }",
        "fn main() { a := [2]string{nil, nil}; _ = a }",
        "fn main() { a := [1]bool{nil}; _ = a }",
    ];

    for src in cases {
        let r = check(src);
        assert_eq!(r.verdict, Verdict::Reject, "expected Reject for:\n{src}");
        assert!(
            error_codes(&r).contains(&"HYB-E1002".to_string()),
            "expected HYB-E1002 for:\n{src}, got: {:?}",
            error_codes(&r)
        );
    }
}

#[test]
fn rejects_nil_in_non_nilable_map_literals() {
    let cases = [
        "fn main() { m := map[string]int{\"a\": nil}; _ = m }",
        "fn main() { m := map[int]int{nil: 1}; _ = m }",
        "fn main() { m := map[string]string{\"k\": nil}; _ = m }",
    ];

    for src in cases {
        let r = check(src);
        assert_eq!(r.verdict, Verdict::Reject, "expected Reject for:\n{src}");
        assert!(
            error_codes(&r).contains(&"HYB-E1002".to_string()),
            "expected HYB-E1002 for:\n{src}, got: {:?}",
            error_codes(&r)
        );
    }
}

#[test]
fn accepts_nil_in_nilable_composite_literals() {
    let cases = [
        "fn main() { s := []*int{nil}; _ = s }",
        "fn main() { s := [][]int{nil}; _ = s }",
        "fn main() { m := map[string][]int{\"a\": nil}; _ = m }",
        "fn main() { m := map[string]map[int]int{\"a\": nil}; _ = m }",
    ];

    for src in cases {
        let r = check(src);
        assert_eq!(
            r.verdict,
            Verdict::Accept,
            "expected Accept for:\n{src}, got errors: {:?}",
            error_codes(&r)
        );
    }
}
