use clippy_utils::{diagnostics::span_lint_and_then, is_res_lang_ctor, path_res};
use rustc_errors::Applicability;
use rustc_hir as hir;
use rustc_lint::LateContext;

use super::UNNECESSARY_LITERAL_UNWRAP;

pub(super) fn check(cx: &LateContext<'_>, expr: &hir::Expr<'_>, recv: &hir::Expr<'_>, name: &str) {
    if let hir::ExprKind::Call(call, [arg]) = recv.kind {
        let mess = if is_res_lang_ctor(cx, path_res(cx, call), hir::LangItem::OptionSome) {
            Some("Some")
        } else if is_res_lang_ctor(cx, path_res(cx, call), hir::LangItem::ResultOk) {
            Some("Ok")
        } else {
            None
        };

        if let Some(constructor) = mess {
            span_lint_and_then(
                cx,
                UNNECESSARY_LITERAL_UNWRAP,
                expr.span,
                &format!("used `{name}()` on `{constructor}` value"),
                |diag| {
                    let suggestions = vec![
                        (call.span.with_hi(arg.span.lo()), String::new()),
                        (expr.span.with_lo(arg.span.hi()), String::new()),
                    ];

                    diag.multipart_suggestion(
                        format!("remove the `{constructor}` and `{name}()`"),
                        suggestions,
                        Applicability::MachineApplicable,
                    );
                },
            );
        }
    }
}
