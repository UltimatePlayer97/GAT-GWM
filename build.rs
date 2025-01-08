extern crate embed_resource;

use embed_resource::CompilationResult;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match embed_resource::compile("src/res/icons.rc", embed_resource::NONE) {
        CompilationResult::NotWindows => {}
        CompilationResult::Ok => {}
        CompilationResult::NotAttempted(e) => return Err(e.into()),
        CompilationResult::Failed(e) => return Err(e.into()),
    };

    Ok(())
}
