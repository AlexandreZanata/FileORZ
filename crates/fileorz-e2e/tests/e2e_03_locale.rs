//! E2E-03: locale en → pt-BR key label probe (Fluent via --demo-i18n).

use fileorz_e2e::{failure_guard, run_fileorz, skip_unless_e2e, IsolatedHome};

#[test]
fn e2e_03_locale_switch_en_to_pt_br() {
    if skip_unless_e2e("e2e-03") {
        return;
    }
    let _guard = failure_guard("e2e-03");
    let home = IsolatedHome::new().expect("isolate");

    let en = run_fileorz(&home, &["--demo-i18n", "--locale", "en"]).expect("en");
    en.assert_ok("E2E-03 en");
    assert!(en.stdout.contains("locale=en"), "{}", en.stdout);
    assert!(
        en.stdout.contains("main-btn-start=Start organizing"),
        "en labels: {}",
        en.stdout
    );
    assert!(
        en.stdout
            .contains("error-folder-missing=Choose a valid folder first."),
        "en error: {}",
        en.stdout
    );

    let pt = run_fileorz(&home, &["--demo-i18n", "--locale", "pt-BR"]).expect("pt");
    pt.assert_ok("E2E-03 pt-BR");
    assert!(pt.stdout.contains("locale=pt-BR"), "{}", pt.stdout);
    assert!(
        pt.stdout.contains("main-btn-start=Iniciar Organização"),
        "pt labels: {}",
        pt.stdout
    );
    assert!(
        pt.stdout
            .contains("error-folder-missing=Selecione uma pasta primeiro!"),
        "pt error: {}",
        pt.stdout
    );
}
