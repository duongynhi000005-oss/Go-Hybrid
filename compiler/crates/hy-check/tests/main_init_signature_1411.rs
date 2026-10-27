use hy_check::{Verdict, check_file};
use hy_syntax::parse_file;

fn check(src: &str) -> hy_check::CheckResult {
    let ast = parse_file("inline.hy", src).expect("must parse");
    check_file(&ast)
}

#[test]
fn rejects_main_with_parameters_issue_1411() {
    let r = check("fn main(a int) {}\n");
    assert_eq!(r.verdict, Verdict::Reject, "{:?}", r.diagnostics);
    assert!(
        r.diagnostics.iter().any(|d| d.code == "HYB-E1002"),
        "expected HYB-E1002 for parameterized main, got: {:?}",
        r.diagnostics
    );
}

#[test]
fn rejects_init_with_parameters_issue_1411() {
    let r = check("fn init(a int) {}\nfn main() {}\n");
    assert_eq!(r.verdict, Verdict::Reject, "{:?}", r.diagnostics);
    assert!(
        r.diagnostics.iter().any(|d| d.code == "HYB-E1002"),
        "expected HYB-E1002 for parameterized init, got: {:?}",
        r.diagnostics
    );
}

#[test]
fn rejects_init_with_result_issue_1411() {
    let r = check("fn init() int {\n    return 0\n}\nfn main() {}\n");
    assert_eq!(r.verdict, Verdict::Reject, "{:?}", r.diagnostics);
    assert!(
        r.diagnostics.iter().any(|d| d.code == "HYB-E1002"),
        "expected HYB-E1002 for result-bearing init, got: {:?}",
        r.diagnostics
    );
}

#[test]
fn accepts_valid_main_init_and_methods_issue_1411() {
    let r_empty = check("fn init() {}\nfn main() {}\n");
    assert_eq!(r_empty.verdict, Verdict::Accept, "{:?}", r_empty.diagnostics);

    let r_main_err = check("fn main() error {\n    return nil\n}\n");
    assert_eq!(
        r_main_err.verdict,
        Verdict::Accept,
        "{:?}",
        r_main_err.diagnostics
    );

    let r_methods = check(
        "struct svc {}\n\
         fn (s svc) init(a int) int { return a }\n\
         fn (s svc) main(a int) int { return a }\n\
         fn main() {\n\
             s := svc{}\n\
             _ = s.init(1) + s.main(2)\n\
         }\n",
    );
    assert_eq!(
        r_methods.verdict,
        Verdict::Accept,
        "{:?}",
        r_methods.diagnostics
    );
}

#[test]
fn rejects_unsupported_main_result_signatures_issue_2048() {
    for src in [
        "fn main() string {\n    return \"ok\"\n}\n",
        "fn main() (int, error) {\n    return 0, nil\n}\n",
        "fn main() (int, int) {\n    return 0, 0\n}\n",
    ] {
        let r = check(src);
        assert_eq!(r.verdict, Verdict::Reject, "expected Reject for:\n{src}");
        assert!(
            r.diagnostics.iter().any(|d| d.code == "HYB-E1002"),
            "expected HYB-E1002 for:\n{src}\ngot: {:?}",
            r.diagnostics
        );
    }
}

#[test]
fn rejects_type_param_and_receiver_type_arg_signature_collisions_issue_2048() {
    for src in [
        "fn id[t any](t int) int { return t }\nfn main() { _ = id[int](1) }\n",
        "fn id[t any](x int) (t int) { return x }\nfn main() { _ = id[int](1) }\n",
        "struct box[t any] { v t }\nfn (t box[t]) get() int { return 1 }\nfn main() {}\n",
        "struct box[t any] { v t }\nfn (b box[t]) get(t int) int { return t }\nfn main() {}\n",
        "struct box[t any] { v t }\nfn (b box[t]) get(x int) (t int) { return x }\nfn main() {}\n",
        "struct pair[t any, u any] { a t\n b u }\nfn (p pair[t, t]) get() int { return 1 }\nfn main() {}\n",
    ] {
        let r = check(src);
        assert_eq!(r.verdict, Verdict::Reject, "expected Reject for:\n{src}");
        assert!(
            r.diagnostics.iter().any(|d| d.code == "HYB-E1002"),
            "expected HYB-E1002 for:\n{src}\ngot: {:?}",
            r.diagnostics
        );
    }
}
