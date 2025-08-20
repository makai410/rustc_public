//! Small utility that print some information about a crate.

#![feature(rustc_private)]
#![feature(assert_matches)]

extern crate rustc_driver;
extern crate rustc_interface;
#[macro_use]
extern crate rustc_smir;
extern crate stable_mir;

use std::collections::HashSet;
use std::io::stdout;
use rustc_smir::{run, rustc_internal};
use stable_mir::{CompilerError, CrateDef};
use std::ops::ControlFlow;
use std::process::ExitCode;
use stable_mir::mir::{LocalDecl, MirVisitor, Terminator, TerminatorKind};
use stable_mir::mir::mono::Instance;
use stable_mir::mir::visit::Location;
use stable_mir::ty::{RigidTy, Ty, TyKind};


/// This is a wrapper that can be used to replace rustc.
fn main() -> ExitCode {
    let rustc_args = std::env::args().into_iter().collect();
    let result = run!(rustc_args, start_demo);
    match result {
        Ok(_) | Err(CompilerError::Skipped | CompilerError::Interrupted(_)) => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    }
}

fn start_demo() -> ControlFlow<()> {
    let crate_name = stable_mir::local_crate().name;
    eprintln!("--- Analyzing crate: {crate_name}");

    let crate_items = stable_mir::all_local_items();
    for item in crate_items {
        eprintln!("  - {} @{:?}", item.name(), item.span())
    }

    let entry_fn = stable_mir::entry_fn().unwrap();
    let entry_instance = Instance::try_from(entry_fn).unwrap();
    analyze_instance(entry_instance);
    ControlFlow::Break(())
}

fn analyze_instance(instance: Instance) {
    eprintln!("--- Analyzing instance: {}", instance.name());
    eprintln!("  - Mangled name: {}", instance.mangled_name());
    eprintln!("  - FnABI: {:?}", instance.fn_abi().unwrap());

    let body = instance.body().unwrap();
    let mut visitor = Visitor {
        locals: body.locals(),
        tys: Default::default(),
        fn_calls: Default::default(),
    };
    visitor.visit_body(&body);
    visitor.tys.iter().for_each(|ty| eprintln!("  - Visited: {ty}"));
    visitor.fn_calls.iter().for_each(|instance| eprintln!("  - Call: {}", instance.name()));

    body.dump(&mut stdout().lock(), &instance.name()).unwrap();
}

struct Visitor<'a> {
    locals: &'a [LocalDecl],
    tys: HashSet<Ty>,
    fn_calls: HashSet<Instance>,
}

impl<'a> MirVisitor for Visitor<'a> {
    fn visit_terminator(&mut self, term: &Terminator, _location: Location) {
        match term.kind {
            TerminatorKind::Call { ref func, .. } => {
                let op_ty = func.ty(self.locals).unwrap();
                let TyKind::RigidTy(RigidTy::FnDef(def, args)) = op_ty.kind() else { return; };
                self.fn_calls.insert(Instance::resolve(def, &args).unwrap());
            }
            _ => {}
        }
    }

    fn visit_ty(&mut self, ty: &Ty, _location: Location) {
        self.tys.insert(*ty);
    }
}
