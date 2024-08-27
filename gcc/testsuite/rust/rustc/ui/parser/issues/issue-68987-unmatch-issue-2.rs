// FIXME: this case need more work to fix
// currently the TokenTree matching ')' with '{', which is not user friendly for diagnostics
async fn obstest() -> Result<> {
    let obs_connect = || -> Result<(), MyError) { // { dg-error "" "" { target *-*-* } }
        async {
        }
    }

    if let Ok(version, scene_list) = obs_connect() {

    } else {

    }
} // { dg-error "" "" { target *-*-* } }

