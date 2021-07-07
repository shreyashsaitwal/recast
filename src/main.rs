mod archive;
mod dexer;
mod jetifier;
mod util;

fn main() {
    let base_dir = archive::extract_aix("./io.shreyash.phase.aix".as_ref(), "./build".as_ref());

    let needs_jetification = jetifier::jetify(&base_dir);

    if needs_jetification {
        dexer::dex(&base_dir);
        archive::pack_aix();
    }
}
