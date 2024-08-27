// { dg-additional-options "-frust-edition= 2021" }

#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait HealthCheck {
    async fn check<const N: usize>() -> bool;
}

async fn do_health_check_par<HC>(hc: HC)
where
    HC: HealthCheck<check(..): Send> + Send + 'static,
// { dg-error "" "" { target *-*-* } .-1 }
{
}

fn main() {}

