type NodeId = u32;
struct Type<'a>(std::marker::PhantomData::<&'a ()>);

type Ast<'ast> = &'ast AstStructure<'ast>;

struct AstStructure<'ast> {
// { dg-error "" "" { target *-*-* } .-1 }
    id: NodeId,
    _: AstKind<'ast>
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
}

enum AstKind<'ast> {
    ExprInt,
    ExprLambda(Ast<'ast>),
}

fn compute_types<'tcx,'ast>(ast: Ast<'ast>) -> Type<'tcx>
{
    match ast.kind {}
// { dg-error ".E0609." "" { target *-*-* } .-1 }
}

fn main() {}

