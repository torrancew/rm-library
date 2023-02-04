use std::{collections::VecDeque, env, fs, io, path::PathBuf};

use rm_library as rm;

fn main() -> anyhow::Result<()> {
    let mut args = env::args().take(2);

    let lib_path = args.next().expect("No library path provided");
    let tpl_path = args.next();
    let lib = rm::Library::load(lib_path, tpl_path)?;

    fs::create_dir_all("target/pdf")?;
    env::set_current_dir("target/pdf")?;

    for entry in lib.clone().values_mut() {
        if let rm::Entry::Document(ref mut doc) = entry {
            if let Some(dir) = lib.template_dir() {
                doc.set_template_dir(dir.as_ref());
            }

            // Walk the graph and collect all of our parent entries
            // to construct our logical output file path
            let mut prefix = VecDeque::new();
            let mut maybe_parent_id = doc.parent();
            while let Some(parent_id) = maybe_parent_id {
                if let Some(rm::Entry::Collection(c)) = lib.get(&parent_id.to_string()) {
                    prefix.push_front(c.name().to_string());
                    maybe_parent_id = c.parent();
                }
            }

            let path = PathBuf::from_iter(prefix)
                .join(doc.name())
                .with_extension("pdf");

            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir)?;
            }

            doc.render_pdf()?
                .save(&mut io::BufWriter::new(fs::File::create(path)?))?;
        }
    }

    Ok(())
}
