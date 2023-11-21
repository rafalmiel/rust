use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::{fn_def_id, is_lint_allowed};
use hir::intravisit::{walk_expr, Visitor};
use hir::{Expr, ExprKind, FnRetTy, FnSig, Node};
use rustc_ast::Label;
use rustc_errors::Applicability;
use rustc_hir as hir;
use rustc_lint::LateContext;

use super::INFINITE_LOOPS;

pub(super) fn check<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &Expr<'_>,
    loop_block: &'tcx hir::Block<'_>,
    label: Option<Label>,
) {
    if is_lint_allowed(cx, INFINITE_LOOPS, expr.hir_id) {
        return;
    }

    // Skip check if this loop is not in a function/method/closure. (In some weird case)
    let Some(parent_fn_ret) = get_parent_fn_ret_ty(cx, expr) else {
        return;
    };
    // Or, its parent function is already returning `Never`
    if matches!(
        parent_fn_ret,
        FnRetTy::Return(hir::Ty {
            kind: hir::TyKind::Never,
            ..
        })
    ) {
        return;
    }

    // First, find any `break` or `return` without entering any inner loop,
    // then, find `return` or labeled `break` which breaks this loop with entering inner loop,
    // otherwise this loop is a infinite loop.
    let mut direct_visitor = LoopVisitor {
        cx,
        label,
        is_finite: false,
        enter_nested_loop: false,
    };
    direct_visitor.visit_block(loop_block);

    let is_finite_loop = direct_visitor.is_finite || {
        let mut inner_loop_visitor = LoopVisitor {
            cx,
            label,
            is_finite: false,
            enter_nested_loop: true,
        };
        inner_loop_visitor.visit_block(loop_block);
        inner_loop_visitor.is_finite
    };

    if !is_finite_loop {
        span_lint_and_then(cx, INFINITE_LOOPS, expr.span, "infinite loop detected", |diag| {
            if let FnRetTy::DefaultReturn(ret_span) = parent_fn_ret {
                diag.span_suggestion(
                    ret_span,
                    "if this is intentional, consider specifing `!` as function return",
                    " -> !",
                    Applicability::MaybeIncorrect,
                );
            } else {
                diag.span_help(
                    expr.span,
                    "if this is not intended, try adding a `break` or `return` condition in this loop",
                );
            }
        });
    }
}

fn get_parent_fn_ret_ty<'tcx>(cx: &LateContext<'tcx>, expr: &Expr<'_>) -> Option<FnRetTy<'tcx>> {
    for (_, parent_node) in cx.tcx.hir().parent_iter(expr.hir_id) {
        match parent_node {
            Node::Item(hir::Item {
                kind: hir::ItemKind::Fn(FnSig { decl, .. }, _, _),
                ..
            })
            | Node::TraitItem(hir::TraitItem {
                kind: hir::TraitItemKind::Fn(FnSig { decl, .. }, _),
                ..
            })
            | Node::ImplItem(hir::ImplItem {
                kind: hir::ImplItemKind::Fn(FnSig { decl, .. }, _),
                ..
            })
            | Node::Expr(Expr {
                kind: ExprKind::Closure(hir::Closure { fn_decl: decl, .. }),
                ..
            }) => return Some(decl.output),
            _ => (),
        }
    }
    None
}

struct LoopVisitor<'hir, 'tcx> {
    cx: &'hir LateContext<'tcx>,
    label: Option<Label>,
    is_finite: bool,
    enter_nested_loop: bool,
}

impl<'hir> Visitor<'hir> for LoopVisitor<'hir, '_> {
    fn visit_expr(&mut self, ex: &'hir Expr<'_>) {
        match &ex.kind {
            ExprKind::Break(hir::Destination { label, .. }, ..) => {
                // When entering nested loop, only by breaking this loop's label
                // would be considered as exiting this loop.
                if self.enter_nested_loop {
                    if label.is_some() && *label == self.label {
                        self.is_finite = true;
                    }
                } else {
                    self.is_finite = true;
                }
            },
            ExprKind::Ret(..) => self.is_finite = true,
            ExprKind::Loop(..) if !self.enter_nested_loop => (),
            _ => {
                // Calls to a function that never return
                if let Some(did) = fn_def_id(self.cx, ex) {
                    let fn_ret_ty = self.cx.tcx.fn_sig(did).skip_binder().output().skip_binder();
                    if fn_ret_ty.is_never() {
                        self.is_finite = true;
                        return;
                    }
                }
                walk_expr(self, ex);
            },
        }
    }
}
