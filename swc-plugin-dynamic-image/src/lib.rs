pub mod elements;
mod helpers;
mod reactive;

use std::sync::Arc;

use crate::elements::transform_elements;
use crate::helpers::generate_values;
use helpers::{arrow_fn_expr, call_expr, const_var_decl, generate_params, ident};
use reactive::ReactiveVisitor;
use swc_core::common::errors::{ColorConfig, Handler};
use swc_core::common::{FileName, SourceFile, SourceMap, GLOBALS};
use swc_core::ecma::ast::{
    ArrayLit, ArrayPat, ArrowExpr, BinExpr, BinaryOp, BindingIdent, BlockStmt, BlockStmtOrExpr,
    CallExpr, Callee, Decl, EsVersion, ExprOrSpread, Ident, ImportDecl, ImportDefaultSpecifier,
    ImportNamedSpecifier, ImportSpecifier, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue,
    JSXClosingElement, JSXClosingFragment, JSXExpr, JSXExprContainer, JSXFragment,
    JSXOpeningFragment, KeyValueProp, MemberExpr, MemberProp, Module, ModuleDecl, ModuleItem,
    ObjectLit, Pat, Prop, PropName, PropOrSpread, ReturnStmt, Stmt, Tpl, TplElement, VarDecl,
    VarDeclKind, VarDeclarator,
};
use swc_core::ecma::parser::{Syntax, TsConfig};
use swc_core::ecma::utils::{prepend_stmt, prepend_stmts, swc_common};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{Expr, JSXElement, JSXElementChild, JSXElementName, JSXOpeningElement, Program},
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
};
use wasm_bindgen::prelude::wasm_bindgen;
#[derive(Debug)]
pub struct DynamicImage {
    reactives: usize,
    element: Box<JSXElement>,
}
impl DynamicImage {
    fn new(reactives: usize, element: Box<JSXElement>) -> Self {
        Self { reactives, element }
    }
}
#[derive(Debug)]
pub struct Import {
    pub source: String,
    pub name: String,
    pub default: bool,
}
impl Import {
    fn new(source: String, name: String, default: bool) -> Self {
        Self {
            source,
            name,
            default,
        }
    }
}
#[derive(Debug)]
pub struct TransformVisitor {
    dynamic_images: Vec<DynamicImage>,
    imports: Vec<Import>,
}
impl Default for TransformVisitor {
    fn default() -> Self {
        Self {
            dynamic_images: vec![],
            imports: vec![],
        }
    }
}
impl TransformVisitor {
    fn insert_imports(&mut self, module: &mut Module) {
        for import in &self.imports {
            let specifier = if import.default {
                ImportSpecifier::Default(ImportDefaultSpecifier {
                    span: DUMMY_SP,
                    local: ident(&import.name),
                })
            } else {
                ImportSpecifier::Named(ImportNamedSpecifier {
                    span: DUMMY_SP,
                    is_type_only: false,
                    local: ident(&import.name),
                    imported: None,
                })
            };
            let stmt = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                span: DUMMY_SP,
                type_only: false,
                asserts: None,
                specifiers: vec![specifier],
                src: Box::new(swc_core::ecma::ast::Str {
                    span: DUMMY_SP,
                    value: import.source.clone().into(),
                    raw: None,
                }),
            }));
            prepend_stmt(&mut module.body, stmt);
        }
    }
    fn insert_dynamics(&mut self, module: &mut Module) {
        if self.dynamic_images.len() == 0 {
            return;
        }
        self.imports.push(Import::new(
            "solid-start/server".into(),
            "server$".into(),
            true,
        ));
        self.imports.push(Import::new(
            "@solid-mediakit/open-graph".into(),
            "createOpenGraphImage".into(),
            false,
        ));
        // self.imports
        //     .push(Import::new("solid-js".into(), "createMemo".into(), false));
        for image in &self.dynamic_images {
            let stmt = Stmt::Decl(const_var_decl(
                "DynamicImage",
                arrow_fn_expr(
                    vec![Pat::Ident(BindingIdent {
                        id: ident("props"),
                        type_ann: None,
                    })],
                    vec![
                        Stmt::Decl(const_var_decl(
                            "img",
                            Expr::Call(CallExpr {
                                span: DUMMY_SP,
                                type_args: None,
                                callee: Callee::Expr(Box::new(Expr::Ident(ident("server$")))),
                                args: vec![ExprOrSpread {
                                    spread: None,
                                    expr: Box::new(arrow_fn_expr(
                                        generate_params(image.reactives),
                                        vec![Stmt::Return(ReturnStmt {
                                            span: DUMMY_SP,
                                            arg: Some(Box::new(call_expr(
                                                "createOpenGraphImage",
                                                Expr::JSXElement(image.element.clone()),
                                            ))),
                                        })],
                                    )),
                                }],
                            }),
                        )),
                        Stmt::Decl(const_var_decl(
                            "url",
                            call_expr(
                                "createMemo",
                                arrow_fn_expr(
                                    vec![],
                                    vec![Stmt::Return(ReturnStmt {
                                        span: DUMMY_SP,
                                        arg: Some(Box::new(Expr::Bin(BinExpr {
                                            span: DUMMY_SP,
                                            op: BinaryOp::Add,
                                            left: Box::new(Expr::Member(MemberExpr {
                                                span: DUMMY_SP,
                                                obj: Box::new(Expr::Ident(ident("img"))),
                                                prop: MemberProp::Ident(ident("url")),
                                            })),
                                            right: Box::new(Expr::Tpl(Tpl {
                                                span: DUMMY_SP,
                                                exprs: vec![Box::new(Expr::Call(CallExpr {
                                                    span: DUMMY_SP,
                                                    callee: Callee::Expr(Box::new(Expr::Ident(
                                                        ident("encodeURIComponent"),
                                                    ))),
                                                    args: vec![ExprOrSpread {
                                                        spread: None,
                                                        expr: Box::new(Expr::Call(CallExpr {
                                                            span: DUMMY_SP,
                                                            type_args: None,
                                                            callee: Callee::Expr(Box::new(
                                                                Expr::Member(MemberExpr {
                                                                    span: DUMMY_SP,
                                                                    obj: Box::new(Expr::Ident(
                                                                        ident("JSON"),
                                                                    )),
                                                                    prop: MemberProp::Ident(ident(
                                                                        "stringify",
                                                                    )),
                                                                }),
                                                            )),
                                                            args: vec![ExprOrSpread {
                                                                spread: None,
                                                                expr: Box::new(Expr::Member(
                                                                    MemberExpr {
                                                                        span: DUMMY_SP,
                                                                        obj: Box::new(Expr::Ident(
                                                                            ident("props"),
                                                                        )),
                                                                        prop: MemberProp::Ident(
                                                                            ident("values"),
                                                                        ),
                                                                    },
                                                                )),
                                                            }],
                                                        })),
                                                    }],
                                                    type_args: None,
                                                }))],
                                                quasis: vec![
                                                    TplElement {
                                                        span: DUMMY_SP,
                                                        tail: false,
                                                        raw: "?args=".into(),
                                                        cooked: None,
                                                    },
                                                    TplElement {
                                                        span: DUMMY_SP,
                                                        tail: true,
                                                        raw: "".into(),
                                                        cooked: None,
                                                    },
                                                ],
                                            })),
                                        }))),
                                    })],
                                ),
                            ),
                        )),
                        Stmt::Return(ReturnStmt {
                            span: DUMMY_SP,
                            arg: Some(Box::new(Expr::JSXFragment(JSXFragment {
                                span: DUMMY_SP,
                                opening: JSXOpeningFragment { span: DUMMY_SP },
                                closing: JSXClosingFragment { span: DUMMY_SP },
                                children: vec![JSXElementChild::JSXExprContainer(
                                    JSXExprContainer {
                                        span: DUMMY_SP,
                                        expr: JSXExpr::Expr(Box::new(Expr::Call(CallExpr {
                                            span: DUMMY_SP,
                                            callee: Callee::Expr(Box::new(Expr::Ident(ident(
                                                "url",
                                            )))),
                                            args: vec![],
                                            type_args: None,
                                        }))),
                                    },
                                )],
                            }))),
                        }),
                    ],
                ),
            ));
            prepend_stmt(&mut module.body, ModuleItem::Stmt(stmt))
        }
    }
}
impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    fn visit_mut_jsx_element(&mut self, n: &mut JSXElement) {
        n.visit_mut_children_with(self);
        if let JSXElementName::Ident(i) = &n.opening.name {
            // Very basic heuristic (should really check where it's imported from)
            if i.sym.to_string() == "DynamicImage" {
                // Collect all our reactive expressions
                let mut element = transform_elements(&n.children);
                let mut visitor = ReactiveVisitor::default();
                element.visit_mut_children_with(&mut visitor);
                self.dynamic_images
                    .push(DynamicImage::new(visitor.reactives.len(), element));
                *n = JSXElement {
                    span: DUMMY_SP,
                    opening: JSXOpeningElement {
                        span: DUMMY_SP,
                        self_closing: true,
                        attrs: vec![JSXAttrOrSpread::JSXAttr(JSXAttr {
                            span: DUMMY_SP,
                            name: JSXAttrName::Ident(ident("values")),
                            value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                                span: DUMMY_SP,
                                expr: JSXExpr::Expr(Box::new(generate_values(&visitor.reactives))),
                            })),
                        })],
                        type_args: None,
                        name: JSXElementName::Ident(ident("DynamicImage")),
                    },
                    children: vec![],
                    closing: None,
                };
                dbg!(self);
            }
        }
    }
    fn visit_mut_module(&mut self, module: &mut Module) {
        module.visit_mut_children_with(self);
        self.insert_dynamics(module);
        self.insert_imports(module);
    }
}

// #[plugin_transform]
// pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
//     program.fold_with(&mut as_folder(TransformVisitor::default()))
// }
#[wasm_bindgen]
pub fn transform(code: String, id: String) -> String {
    let cm: Arc<SourceMap> = Arc::<SourceMap>::default();
    let handler: Handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let compiler: swc::Compiler = swc::Compiler::new(cm.clone());

    let fm: Arc<SourceFile> = cm.new_source_file(FileName::Custom(id), code);
    GLOBALS.set(&Default::default(), || {
        let result = compiler.parse_js(
            fm,
            &handler,
            EsVersion::EsNext,
            Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: false,
                dts: false,
                no_early_errors: false,
                disallow_ambiguous_jsx_like: true,
            }),
            swc::config::IsModule::Bool(true),
            None,
        );
        let inter = result
            .unwrap()
            .fold_with(&mut as_folder(TransformVisitor::default()));

        let out = compiler.print(
            &inter,
            None,
            None,
            false,
            EsVersion::EsNext,
            swc::config::SourceMapsConfig::Bool(false),
            &Default::default(),
            None,
            false,
            None,
            false,
            false,
            "",
        );
        out.unwrap().code
    })
}
